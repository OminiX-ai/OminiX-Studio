//! Model Registry — JSON-driven, futureproof model catalog
//!
//! This is the single source of truth for all MLX model metadata.
//! Models are defined in a bundled `models_registry.json` and can be
//! overridden / extended by a user-local file at `~/.ominix/models_registry.json`.
//!
//! Adding a new model requires only a JSON entry — no Rust code changes.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// ─── Category ────────────────────────────────────────────────────────────────

/// Broad category that drives filtering and UI coloring in the Model Hub.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryCategory {
    /// Large Language Models — text in, text out
    Llm,
    /// Vision-Language Models — image + text in, text out
    Vlm,
    /// Automatic Speech Recognition — audio in, text out
    Asr,
    /// Text-to-Speech / Voice Cloning — text in, audio out
    Tts,
    /// Image Generation — text in, image out
    ImageGen,
}

impl RegistryCategory {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Llm => "LLM",
            Self::Vlm => "VLM",
            Self::Asr => "ASR",
            Self::Tts => "TTS",
            Self::ImageGen => "Image",
        }
    }

    /// UI accent color (hex) for the category badge
    pub fn color(&self) -> &'static str {
        match self {
            Self::Llm => "#6366f1",     // indigo
            Self::Vlm => "#8b5cf6",     // violet
            Self::Asr => "#10b981",     // emerald
            Self::Tts => "#f59e0b",     // amber
            Self::ImageGen => "#ec4899", // pink
        }
    }
}

// ─── API Type ─────────────────────────────────────────────────────────────────

/// Which ominix-api endpoint this model uses.
/// All endpoints follow OpenAI naming conventions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApiType {
    /// POST /v1/chat/completions  (LLM + VLM)
    ChatCompletions,
    /// POST /v1/audio/transcriptions  (ASR)
    AudioTranscription,
    /// POST /v1/audio/speech  (TTS)
    AudioSpeech,
    /// POST /v1/images/generations  (Image Gen)
    ImageGeneration,
}

// ─── Panel Type ───────────────────────────────────────────────────────────────

/// Which right-panel widget to render in the Model Hub for this model.
/// Adding a new modality = add one variant here + implement the widget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanelType {
    LlmChat,
    VlmChat,
    AsrTranscription,
    TtsSynthesis,
    ImageGeneration,
}

// ─── Source ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    HuggingFace,
    ModelScope,
    DirectUrl,
    /// Requires manual installation — no automatic download
    Manual,
}

impl Default for SourceKind {
    fn default() -> Self {
        Self::HuggingFace
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrySource {
    #[serde(default)]
    pub kind: SourceKind,
    /// HuggingFace / ModelScope repo ID (e.g. "mlx-community/Qwen3-8B-8bit")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    /// Direct download URL (used when kind == DirectUrl or as fallback)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Additional mirror/backup URLs tried in order
    #[serde(default)]
    pub backup_urls: Vec<String>,
    /// Branch / tag / commit (default: "main")
    #[serde(default = "default_revision")]
    pub revision: String,
}

fn default_revision() -> String {
    "main".to_string()
}

// ─── Storage ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryStorage {
    /// Local path with ~ support (e.g. "~/.cache/huggingface/hub/...")
    pub local_path: String,
    /// Approximate on-disk size in bytes (0 = unknown)
    #[serde(default)]
    pub size_bytes: u64,
    /// Human-readable size string (e.g. "~8 GB")
    #[serde(default)]
    pub size_display: String,
}

impl RegistryStorage {
    pub fn expanded_path(&self) -> String {
        expand_tilde(&self.local_path)
    }
}

// ─── Runtime ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryRuntime {
    /// Which API endpoint to call
    pub api_type: ApiType,
    /// Model ID string sent in the API request body (e.g. "qwen3-8b")
    pub api_model_id: String,
    /// Estimated RAM needed to keep the model loaded (GB)
    #[serde(default)]
    pub memory_gb: f32,
    /// Platforms this model runs on (e.g. ["apple_silicon"])
    #[serde(default)]
    pub platforms: Vec<String>,
    /// Whether the model accepts image inputs (VLM)
    #[serde(default)]
    pub supports_images: bool,
    /// Whether the model supports streaming responses
    #[serde(default = "default_true")]
    pub supports_streaming: bool,
    /// Quantization format used (e.g. "8bit", "4bit", "fp16", "bf16")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantization: Option<String>,
}

fn default_true() -> bool {
    true
}

// ─── UI Hints ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryUiHints {
    /// Which right-panel to show when this model is selected
    pub panel_type: PanelType,
    /// Override accent color (falls back to category color if absent)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Icon name from the icons/ resource folder (without extension)
    #[serde(default = "default_icon")]
    pub icon: String,
}

fn default_icon() -> String {
    "app".to_string()
}

// ─── Registry Model ───────────────────────────────────────────────────────────

/// A single model entry in the registry.
/// Every field needed to download, load, and use the model is here.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryModel {
    /// Stable unique identifier (e.g. "qwen3-8b")
    pub id: String,
    /// Display name
    pub name: String,
    /// One-sentence description
    pub description: String,
    /// Broad category
    pub category: RegistryCategory,
    /// Searchable tags
    #[serde(default)]
    pub tags: Vec<String>,
    /// Download source
    pub source: RegistrySource,
    /// Storage location
    pub storage: RegistryStorage,
    /// Runtime / API information
    pub runtime: RegistryRuntime,
    /// UI display hints
    pub ui: RegistryUiHints,
}

impl RegistryModel {
    /// Accent color: model-specific override or category default
    pub fn accent_color(&self) -> &str {
        self.ui.color.as_deref().unwrap_or_else(|| self.category.color())
    }
}

// ─── Registry ─────────────────────────────────────────────────────────────────

/// The full model catalog.
///
/// Load order:
/// 1. Bundled JSON (compiled into the binary via `include_str!`)
/// 2. User override at `~/.ominix/models_registry.json` (merged on top)
///
/// Server updates are fetched in the background on launch and written to
/// the user override file so they take effect on the next startup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRegistry {
    /// Semver string (e.g. "1.0.0")
    pub version: String,
    pub models: Vec<RegistryModel>,
}

/// Bundled default registry JSON
const BUNDLED_REGISTRY: &str = include_str!("models_registry.json");

impl ModelRegistry {
    /// Load registry: bundled defaults merged with user override.
    pub fn load() -> Self {
        // 1. Parse bundled JSON (must succeed — it's compiled-in)
        let mut registry: ModelRegistry = serde_json::from_str(BUNDLED_REGISTRY)
            .expect("bundled models_registry.json is invalid — this is a compile-time bug");

        // 2. Merge user override if present
        if let Some(override_path) = Self::override_path() {
            if let Ok(contents) = std::fs::read_to_string(&override_path) {
                match serde_json::from_str::<ModelRegistry>(&contents) {
                    Ok(user_registry) => {
                        registry.merge(user_registry);
                        log::info!(
                            "ModelRegistry: merged user override from {:?}",
                            override_path
                        );
                    }
                    Err(e) => {
                        log::warn!(
                            "ModelRegistry: failed to parse user override {:?}: {}",
                            override_path,
                            e
                        );
                    }
                }
            }
        }

        log::info!("ModelRegistry: loaded {} models", registry.models.len());
        registry
    }

    /// Merge another registry on top: existing models are updated,
    /// new models are appended.  The caller's version wins.
    pub fn merge(&mut self, other: ModelRegistry) {
        for incoming in other.models {
            if let Some(existing) = self.models.iter_mut().find(|m| m.id == incoming.id) {
                *existing = incoming;
            } else {
                self.models.push(incoming);
            }
        }
    }

    /// Fetch an updated registry from the OminiX server in a background thread.
    /// On success the result is saved to `~/.ominix/models_registry.json` and
    /// will be picked up the next time `ModelRegistry::load()` is called.
    pub fn fetch_updates_async() {
        std::thread::spawn(|| {
            const REGISTRY_URL: &str =
                "https://registry.ominix.ai/models_registry.json";

            let client = reqwest::blocking::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build();

            let client = match client {
                Ok(c) => c,
                Err(e) => {
                    log::debug!("ModelRegistry fetch: failed to build client: {}", e);
                    return;
                }
            };

            match client.get(REGISTRY_URL).send() {
                Ok(resp) if resp.status().is_success() => {
                    match resp.json::<ModelRegistry>() {
                        Ok(registry) => {
                            if let Err(e) = registry.save_override() {
                                log::warn!("ModelRegistry fetch: failed to save override: {}", e);
                            } else {
                                log::info!(
                                    "ModelRegistry: fetched {} models from server",
                                    registry.models.len()
                                );
                            }
                        }
                        Err(e) => {
                            log::debug!("ModelRegistry fetch: failed to parse JSON: {}", e);
                        }
                    }
                }
                Ok(resp) => {
                    log::debug!(
                        "ModelRegistry fetch: server returned {}",
                        resp.status()
                    );
                }
                Err(e) => {
                    log::debug!("ModelRegistry fetch: request failed: {}", e);
                }
            }
        });
    }

    /// Save this registry to the user override file.
    pub fn save_override(&self) -> Result<(), String> {
        let path = Self::override_path()
            .ok_or_else(|| "cannot determine home directory".to_string())?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("create dir: {}", e))?;
        }

        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("serialize: {}", e))?;

        std::fs::write(&path, json).map_err(|e| format!("write: {}", e))?;
        Ok(())
    }

    /// `~/.ominix/models_registry.json`
    fn override_path() -> Option<PathBuf> {
        dirs::home_dir().map(|h| h.join(".ominix").join("models_registry.json"))
    }

    // ── Convenience queries ──────────────────────────────────────────────────

    pub fn get(&self, id: &str) -> Option<&RegistryModel> {
        self.models.iter().find(|m| m.id == id)
    }

    pub fn by_category(&self, cat: RegistryCategory) -> impl Iterator<Item = &RegistryModel> {
        self.models.iter().filter(move |m| m.category == cat)
    }

    pub fn search<'a>(&'a self, query: &'a str) -> impl Iterator<Item = &'a RegistryModel> {
        let q = query.to_lowercase();
        self.models.iter().filter(move |m| {
            m.name.to_lowercase().contains(&q)
                || m.description.to_lowercase().contains(&q)
                || m.tags.iter().any(|t| t.to_lowercase().contains(&q))
        })
    }
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn expand_tilde(path: &str) -> String {
    if path.starts_with("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(&path[2..]).to_string_lossy().to_string();
        }
    }
    path.to_string()
}
