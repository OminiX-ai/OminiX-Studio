//! Local Models Data and Persistence
//!
//! Manages local AI model information and checks their availability on disk.
//!
//! This module provides two versions of the local models configuration:
//! - V1 (legacy): Simple model definitions with basic status tracking
//! - V2 (current): Comprehensive JSON-based system with per-file tracking,
//!   backup URLs, runtime memory info, and per-model download progress

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const LOCAL_MODELS_FILENAME: &str = "local_models.json";
const LOCAL_MODELS_CONFIG_FILENAME: &str = "local_models_config.json";
const CONFIG_VERSION: &str = "1.0.0";

/// Model category for display and coloring
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ModelCategory {
    Llm,
    Image,
    Asr,
    Tts,
}

impl ModelCategory {
    pub fn as_f64(&self) -> f64 {
        match self {
            ModelCategory::Llm => 0.0,
            ModelCategory::Image => 1.0,
            ModelCategory::Asr => 2.0,
            ModelCategory::Tts => 3.0,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            ModelCategory::Llm => "LLM",
            ModelCategory::Image => "Image",
            ModelCategory::Asr => "ASR",
            ModelCategory::Tts => "TTS",
        }
    }
}

/// Model download status
#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub enum ModelStatus {
    #[default]
    NotDownloaded,
    Downloading,
    Ready,
}

impl ModelStatus {
    pub fn as_f64(&self) -> f64 {
        match self {
            ModelStatus::NotDownloaded => 0.0,
            ModelStatus::Downloading => 1.0,
            ModelStatus::Ready => 2.0,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            ModelStatus::NotDownloaded => "Not Downloaded",
            ModelStatus::Downloading => "Downloading...",
            ModelStatus::Ready => "Ready",
        }
    }
}

/// Local model definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocalModel {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: ModelCategory,
    pub size: String,
    pub download_url: String,
    pub model_path: String,
    #[serde(default)]
    pub status: ModelStatus,
}

impl LocalModel {
    /// Check if the model exists on disk and update status
    pub fn check_availability(&mut self) {
        let expanded_path = expand_tilde(&self.model_path);
        let path = PathBuf::from(&expanded_path);

        if path.exists() && path.is_dir() {
            // Check if the directory has meaningful content (not just hidden files or markers)
            if let Ok(entries) = std::fs::read_dir(&path) {
                let meaningful_files = entries
                    .filter_map(|e| e.ok())
                    .filter(|e| {
                        let name = e.file_name();
                        let name_str = name.to_string_lossy();
                        // Skip hidden files and our marker file
                        !name_str.starts_with('.') && name_str != ".moly-downloaded"
                    })
                    .count();

                if meaningful_files > 0 {
                    self.status = ModelStatus::Ready;
                    return;
                }
            }
        }

        // Files don't exist - always mark as not downloaded
        // (this also clears stale "Downloading" status from interrupted downloads)
        self.status = ModelStatus::NotDownloaded;
    }

    /// Get the expanded model path (with ~ resolved)
    pub fn expanded_path(&self) -> String {
        expand_tilde(&self.model_path)
    }
}

/// Expand ~ to home directory
fn expand_tilde(path: &str) -> String {
    if path.starts_with("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(&path[2..]).to_string_lossy().to_string();
        }
    }
    path.to_string()
}

/// Get the default list of supported local models
pub fn get_default_local_models() -> Vec<LocalModel> {
    vec![
        LocalModel {
            id: "flux".to_string(),
            name: "FLUX.2-klein-4B".to_string(),
            description: "4B parameter FLUX image generation model. Fast inference with 4-step generation, optimized for Apple Silicon.".to_string(),
            category: ModelCategory::Image,
            size: "~13 GB".to_string(),
            download_url: "https://huggingface.co/black-forest-labs/FLUX.2-klein-4B".to_string(),
            model_path: "~/.cache/huggingface/hub/models--black-forest-labs--FLUX.2-klein-4B".to_string(),
            status: ModelStatus::NotDownloaded,
        },
        LocalModel {
            id: "zimage".to_string(),
            name: "Z-Image Turbo".to_string(),
            description: "6B parameter S3-DiT image generation. 9-step turbo inference (~3s/image), 4-bit quantized for efficient memory usage.".to_string(),
            category: ModelCategory::Image,
            size: "~12 GB".to_string(),
            download_url: "https://huggingface.co/uqer1244/MLX-z-image".to_string(),
            model_path: "~/.cache/huggingface/hub/models--uqer1244--MLX-z-image".to_string(),
            status: ModelStatus::NotDownloaded,
        },
        LocalModel {
            id: "qwen3-8b".to_string(),
            name: "Qwen3 8B".to_string(),
            description: "Powerful 8B parameter language model. Excellent for chat, coding assistance, and general text generation.".to_string(),
            category: ModelCategory::Llm,
            size: "~16 GB".to_string(),
            download_url: "https://huggingface.co/mlx-community/Qwen3-8B-bf16".to_string(),
            model_path: "~/.cache/huggingface/hub/models--mlx-community--Qwen3-8B-bf16".to_string(),
            status: ModelStatus::NotDownloaded,
        },
        LocalModel {
            id: "funasr".to_string(),
            name: "FunASR Paraformer".to_string(),
            description: "High-accuracy Chinese ASR. Downloads PyTorch model and auto-converts to MLX format.".to_string(),
            category: ModelCategory::Asr,
            size: "~1.0 GB".to_string(),
            download_url: "https://modelscope.cn/models/damo/speech_seaco_paraformer_large_asr_nat-zh-cn-16k-common-vocab8404-pytorch".to_string(),
            model_path: "~/.dora/models/paraformer".to_string(),
            status: ModelStatus::NotDownloaded,
        },
        LocalModel {
            id: "funasr-nano".to_string(),
            name: "FunASR Nano".to_string(),
            description: "800M LLM-based ASR supporting 31 languages. Combines Whisper encoder with Qwen LLM for high accuracy.".to_string(),
            category: ModelCategory::Asr,
            size: "~1.9 GB".to_string(),
            download_url: "https://huggingface.co/mlx-community/Fun-ASR-Nano-2512-fp16".to_string(),
            model_path: "~/.dora/models/funasr-nano".to_string(),
            status: ModelStatus::NotDownloaded,
        },
    ]
}

/// Persistent storage for local models
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalModelsConfig {
    pub models: Vec<LocalModel>,
}

impl Default for LocalModelsConfig {
    fn default() -> Self {
        Self {
            models: get_default_local_models(),
        }
    }
}

impl LocalModelsConfig {
    /// Load config from disk, or return defaults if not found
    pub fn load() -> Self {
        let path = Self::config_path();
        log::debug!("Loading local models config from {:?}", path);

        if let Ok(contents) = std::fs::read_to_string(&path) {
            match serde_json::from_str::<LocalModelsConfig>(&contents) {
                Ok(mut config) => {
                    log::debug!("Parsed local models config successfully");
                    // Merge with defaults to add any new models
                    config.merge_with_defaults();
                    // Check availability for all models and save updated status
                    config.check_all_availability();
                    config.save();
                    return config;
                }
                Err(e) => {
                    log::error!("Failed to parse local models config: {:?}", e);
                }
            }
        } else {
            log::debug!("No local models config found, using defaults");
        }

        let mut config = LocalModelsConfig::default();
        config.check_all_availability();
        config.save();
        config
    }

    /// Save config to disk
    pub fn save(&self) {
        let path = Self::config_path();

        // Ensure directory exists
        if let Some(parent) = path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                log::error!("Failed to create config directory: {:?}", e);
                return;
            }
        }

        match serde_json::to_string_pretty(self) {
            Ok(json) => {
                if let Err(e) = std::fs::write(&path, &json) {
                    log::error!("Failed to write local models config: {:?}", e);
                } else {
                    log::info!("Saved local models config to {:?}", path);
                }
            }
            Err(e) => {
                log::error!("Failed to serialize local models config: {:?}", e);
            }
        }
    }

    /// Get the path to the config file
    fn config_path() -> PathBuf {
        if let Some(home) = dirs::home_dir() {
            home.join(".moly").join(LOCAL_MODELS_FILENAME)
        } else {
            PathBuf::from(".moly").join(LOCAL_MODELS_FILENAME)
        }
    }

    /// Check availability for all models
    pub fn check_all_availability(&mut self) {
        for model in &mut self.models {
            model.check_availability();
        }
    }

    /// Refresh a specific model's availability
    pub fn refresh_model(&mut self, model_id: &str) {
        if let Some(model) = self.models.iter_mut().find(|m| m.id == model_id) {
            model.check_availability();
            self.save();
        }
    }

    /// Get a model by ID
    pub fn get_model(&self, id: &str) -> Option<&LocalModel> {
        self.models.iter().find(|m| m.id == id)
    }

    /// Get a mutable model by ID
    pub fn get_model_mut(&mut self, id: &str) -> Option<&mut LocalModel> {
        self.models.iter_mut().find(|m| m.id == id)
    }

    /// Set model status and save
    pub fn set_model_status(&mut self, id: &str, status: ModelStatus) {
        if let Some(model) = self.get_model_mut(id) {
            model.status = status;
            self.save();
        }
    }

    /// Merge with default models (add any missing)
    fn merge_with_defaults(&mut self) {
        let defaults = get_default_local_models();
        for default_model in defaults {
            if !self.models.iter().any(|m| m.id == default_model.id) {
                self.models.push(default_model);
            }
        }
    }
}

// ============================================================================
// V2 Configuration System - JSON as Single Source of Truth
// ============================================================================

/// Source type for model downloads
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    Huggingface,
    Modelscope,
    DirectUrl,
    Local,
}

impl Default for SourceType {
    fn default() -> Self {
        Self::Huggingface
    }
}

/// Model download source configuration with backup URLs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelSource {
    pub primary_url: String,
    #[serde(default)]
    pub backup_urls: Vec<String>,
    #[serde(default)]
    pub source_type: SourceType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    #[serde(default = "default_revision")]
    pub revision: String,
}

fn default_revision() -> String {
    "main".to_string()
}

impl ModelSource {
    /// Get all URLs in order (primary first, then backups)
    pub fn all_urls(&self) -> Vec<&str> {
        let mut urls = vec![self.primary_url.as_str()];
        urls.extend(self.backup_urls.iter().map(|s| s.as_str()));
        urls
    }
}

/// Model storage configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelStorage {
    pub local_path: String,
    #[serde(default)]
    pub total_size_bytes: u64,
    #[serde(default)]
    pub total_size_display: String,
}

impl ModelStorage {
    /// Get expanded path with ~ resolved to home directory
    pub fn expanded_path(&self) -> String {
        expand_tilde(&self.local_path)
    }
}

/// Individual file information within a model
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelFileInfo {
    pub path: String,
    #[serde(default)]
    pub size_bytes: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    #[serde(default)]
    pub download_url: String,
    #[serde(default)]
    pub downloaded: bool,
}

/// Runtime requirements for the model
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ModelRuntime {
    /// Minimum memory required to load the model (MB)
    #[serde(default)]
    pub memory_required_mb: u64,
    /// Peak memory during inference (MB)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_peak_mb: Option<u64>,
    /// Recommended VRAM for GPU inference (MB)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_vram_mb: Option<u64>,
    /// Supported platforms (e.g., "macos-arm64", "linux-cuda")
    #[serde(default)]
    pub supported_platforms: Vec<String>,
    /// Quantization format (e.g., "bf16", "4bit", "fp16")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantization: Option<String>,
    /// Inference engine (e.g., "mlx", "onnx")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_engine: Option<String>,
}

/// Model state in the V2 system
#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelState {
    /// Model not downloaded, no files present
    #[default]
    NotAvailable,
    /// Download in progress
    Downloading,
    /// All files downloaded and ready
    Ready,
    /// Some files downloaded (interrupted download)
    Partial,
    /// Download failed, needs retry
    Error,
    /// Verifying file integrity
    Verifying,
}

impl ModelState {
    pub fn as_f64(&self) -> f64 {
        match self {
            ModelState::NotAvailable => 0.0,
            ModelState::Downloading => 1.0,
            ModelState::Ready => 2.0,
            ModelState::Partial => 3.0,
            ModelState::Error => 4.0,
            ModelState::Verifying => 5.0,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            ModelState::NotAvailable => "Not Available",
            ModelState::Downloading => "Downloading...",
            ModelState::Ready => "Ready",
            ModelState::Partial => "Partial",
            ModelState::Error => "Error",
            ModelState::Verifying => "Verifying...",
        }
    }

    /// Convert from legacy ModelStatus
    pub fn from_legacy(status: ModelStatus) -> Self {
        match status {
            ModelStatus::NotDownloaded => ModelState::NotAvailable,
            ModelStatus::Downloading => ModelState::Downloading,
            ModelStatus::Ready => ModelState::Ready,
        }
    }
}

/// Status information for a model
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ModelStatusInfo {
    #[serde(default)]
    pub state: ModelState,
    #[serde(default)]
    pub downloaded_bytes: u64,
    #[serde(default)]
    pub downloaded_files: usize,
    #[serde(default)]
    pub total_files: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_checked: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_downloaded: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// Per-model download progress tracking
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DownloadProgress {
    #[serde(default)]
    pub is_active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_file: Option<String>,
    #[serde(default)]
    pub current_file_index: usize,
    #[serde(default)]
    pub current_file_bytes: u64,
    #[serde(default)]
    pub current_file_total: u64,
    #[serde(default)]
    pub overall_bytes: u64,
    #[serde(default)]
    pub overall_total: u64,
    #[serde(default)]
    pub speed_bytes_per_sec: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eta_seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
}

impl DownloadProgress {
    /// Calculate overall progress percentage (0.0 to 1.0)
    pub fn progress_percent(&self) -> f64 {
        if self.overall_total == 0 {
            return 0.0;
        }
        (self.overall_bytes as f64 / self.overall_total as f64).min(1.0)
    }

    /// Reset progress for a new download
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Start a new download
    pub fn start(&mut self, total_bytes: u64, total_files: usize) {
        self.is_active = true;
        self.overall_total = total_bytes;
        self.overall_bytes = 0;
        self.current_file_index = 0;
        self.started_at = Some(Utc::now().to_rfc3339());
    }

    /// Update progress for current file
    pub fn update(&mut self, file_name: &str, file_bytes: u64, file_total: u64, overall_bytes: u64) {
        self.current_file = Some(file_name.to_string());
        self.current_file_bytes = file_bytes;
        self.current_file_total = file_total;
        self.overall_bytes = overall_bytes;
    }

    /// Mark download as complete
    pub fn complete(&mut self) {
        self.is_active = false;
        self.overall_bytes = self.overall_total;
    }

    /// Mark download as failed
    pub fn fail(&mut self) {
        self.is_active = false;
    }
}

/// Complete local model definition (V2)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocalModelV2 {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: ModelCategory,
    #[serde(default)]
    pub tags: Vec<String>,
    pub source: ModelSource,
    pub storage: ModelStorage,
    #[serde(default)]
    pub files: Vec<ModelFileInfo>,
    #[serde(default)]
    pub runtime: ModelRuntime,
    #[serde(default)]
    pub status: ModelStatusInfo,
    #[serde(default)]
    pub download_progress: DownloadProgress,
}

impl LocalModelV2 {
    /// Get expanded local path
    pub fn expanded_path(&self) -> String {
        self.storage.expanded_path()
    }

    /// Check if model is ready to use
    pub fn is_ready(&self) -> bool {
        self.status.state == ModelState::Ready
    }

    /// Check if model is currently downloading
    pub fn is_downloading(&self) -> bool {
        self.status.state == ModelState::Downloading || self.download_progress.is_active
    }

    /// Scan filesystem to update status
    pub fn scan_filesystem(&mut self) {
        let path = self.expanded_path();
        let base_path = Path::new(&path);

        if !base_path.exists() {
            self.status.state = ModelState::NotAvailable;
            self.status.downloaded_files = 0;
            self.status.downloaded_bytes = 0;
            self.status.last_checked = Some(Utc::now().to_rfc3339());
            return;
        }

        // If we have file list, check each file
        if !self.files.is_empty() {
            let mut downloaded_count = 0;
            let mut downloaded_bytes = 0u64;

            for file in &mut self.files {
                let file_path = base_path.join(&file.path);
                if file_path.exists() {
                    if let Ok(metadata) = std::fs::metadata(&file_path) {
                        let size = metadata.len();
                        // Consider downloaded if size matches or is close (within 1%)
                        if file.size_bytes == 0 || size >= file.size_bytes * 99 / 100 {
                            file.downloaded = true;
                            downloaded_count += 1;
                            downloaded_bytes += size;
                        } else {
                            file.downloaded = false;
                        }
                    }
                } else {
                    file.downloaded = false;
                }
            }

            self.status.downloaded_files = downloaded_count;
            self.status.downloaded_bytes = downloaded_bytes;
            self.status.total_files = self.files.len();

            self.status.state = if downloaded_count == 0 {
                ModelState::NotAvailable
            } else if downloaded_count == self.files.len() {
                ModelState::Ready
            } else {
                ModelState::Partial
            };
        } else {
            // No file list - check if directory has meaningful content
            if let Ok(entries) = std::fs::read_dir(&base_path) {
                let meaningful_files: Vec<_> = entries
                    .filter_map(|e| e.ok())
                    .filter(|e| {
                        let name = e.file_name();
                        let name_str = name.to_string_lossy();
                        !name_str.starts_with('.') && name_str != ".moly-downloaded"
                    })
                    .collect();

                if !meaningful_files.is_empty() {
                    self.status.state = ModelState::Ready;
                    self.status.downloaded_files = meaningful_files.len();
                    // Calculate total size
                    let total_size: u64 = meaningful_files
                        .iter()
                        .filter_map(|e| e.metadata().ok())
                        .map(|m| m.len())
                        .sum();
                    self.status.downloaded_bytes = total_size;
                } else {
                    self.status.state = ModelState::NotAvailable;
                    self.status.downloaded_files = 0;
                    self.status.downloaded_bytes = 0;
                }
            } else {
                self.status.state = ModelState::NotAvailable;
            }
        }

        self.status.last_checked = Some(Utc::now().to_rfc3339());
    }

    /// Convert from legacy LocalModel
    pub fn from_legacy(model: &LocalModel) -> Self {
        let source_type = if model.download_url.contains("modelscope.cn") {
            SourceType::Modelscope
        } else if model.download_url.contains("huggingface.co") {
            SourceType::Huggingface
        } else {
            SourceType::DirectUrl
        };

        Self {
            id: model.id.clone(),
            name: model.name.clone(),
            description: model.description.clone(),
            category: model.category,
            tags: vec![],
            source: ModelSource {
                primary_url: model.download_url.clone(),
                backup_urls: vec![],
                source_type,
                repo_id: None,
                revision: "main".to_string(),
            },
            storage: ModelStorage {
                local_path: model.model_path.clone(),
                total_size_bytes: 0,
                total_size_display: model.size.clone(),
            },
            files: vec![],
            runtime: ModelRuntime::default(),
            status: ModelStatusInfo {
                state: ModelState::from_legacy(model.status),
                ..Default::default()
            },
            download_progress: DownloadProgress::default(),
        }
    }
}

/// V2 Configuration file structure
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocalModelsConfigV2 {
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    pub models: Vec<LocalModelV2>,
}

fn default_version() -> String {
    CONFIG_VERSION.to_string()
}

impl Default for LocalModelsConfigV2 {
    fn default() -> Self {
        Self {
            version: CONFIG_VERSION.to_string(),
            last_updated: Some(Utc::now().to_rfc3339()),
            models: get_default_local_models_v2(),
        }
    }
}

impl LocalModelsConfigV2 {
    /// Load config from disk, migrate from V1 if needed, or return defaults
    pub fn load() -> Self {
        let v2_path = Self::config_path();
        log::debug!("Loading local models V2 config from {:?}", v2_path);

        // Try to load V2 config first
        if let Ok(contents) = std::fs::read_to_string(&v2_path) {
            match serde_json::from_str::<LocalModelsConfigV2>(&contents) {
                Ok(mut config) => {
                    log::info!("Loaded V2 config with {} models", config.models.len());
                    config.merge_with_defaults();
                    config.startup_scan();
                    return config;
                }
                Err(e) => {
                    log::error!("Failed to parse V2 config: {:?}", e);
                }
            }
        }

        // Try to migrate from V1
        let v1_path = Self::legacy_config_path();
        if let Ok(contents) = std::fs::read_to_string(&v1_path) {
            if let Ok(v1_config) = serde_json::from_str::<LocalModelsConfig>(&contents) {
                log::info!("Migrating from V1 config with {} models", v1_config.models.len());
                let mut config = Self::migrate_from_v1(&v1_config);
                config.merge_with_defaults();
                config.startup_scan();
                config.save();
                return config;
            }
        }

        // Return defaults
        log::info!("Creating default V2 config");
        let mut config = Self::default();
        config.startup_scan();
        config.save();
        config
    }

    /// Save config to disk
    pub fn save(&self) {
        let path = Self::config_path();

        if let Some(parent) = path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                log::error!("Failed to create config directory: {:?}", e);
                return;
            }
        }

        match serde_json::to_string_pretty(self) {
            Ok(json) => {
                if let Err(e) = std::fs::write(&path, &json) {
                    log::error!("Failed to write V2 config: {:?}", e);
                } else {
                    log::debug!("Saved V2 config to {:?}", path);
                }
            }
            Err(e) => {
                log::error!("Failed to serialize V2 config: {:?}", e);
            }
        }
    }

    /// Get V2 config file path
    fn config_path() -> PathBuf {
        if let Some(home) = dirs::home_dir() {
            home.join(".moly").join(LOCAL_MODELS_CONFIG_FILENAME)
        } else {
            PathBuf::from(".moly").join(LOCAL_MODELS_CONFIG_FILENAME)
        }
    }

    /// Get legacy V1 config file path
    fn legacy_config_path() -> PathBuf {
        if let Some(home) = dirs::home_dir() {
            home.join(".moly").join(LOCAL_MODELS_FILENAME)
        } else {
            PathBuf::from(".moly").join(LOCAL_MODELS_FILENAME)
        }
    }

    /// Migrate from V1 configuration
    fn migrate_from_v1(v1: &LocalModelsConfig) -> Self {
        let models: Vec<LocalModelV2> = v1.models.iter().map(LocalModelV2::from_legacy).collect();
        Self {
            version: CONFIG_VERSION.to_string(),
            last_updated: Some(Utc::now().to_rfc3339()),
            models,
        }
    }

    /// Scan filesystem for all models on startup
    pub fn startup_scan(&mut self) {
        log::info!("Running startup scan for {} models", self.models.len());
        for model in &mut self.models {
            // Don't scan if actively downloading
            if !model.download_progress.is_active {
                model.scan_filesystem();
            }
        }
        self.last_updated = Some(Utc::now().to_rfc3339());
    }

    /// Merge with default models (add any missing)
    fn merge_with_defaults(&mut self) {
        let defaults = get_default_local_models_v2();
        for default_model in defaults {
            if !self.models.iter().any(|m| m.id == default_model.id) {
                self.models.push(default_model);
            }
        }
    }

    /// Get model by ID
    pub fn get_model(&self, id: &str) -> Option<&LocalModelV2> {
        self.models.iter().find(|m| m.id == id)
    }

    /// Get mutable model by ID
    pub fn get_model_mut(&mut self, id: &str) -> Option<&mut LocalModelV2> {
        self.models.iter_mut().find(|m| m.id == id)
    }

    /// Update model state and save
    pub fn set_model_state(&mut self, id: &str, state: ModelState) {
        if let Some(model) = self.get_model_mut(id) {
            model.status.state = state;
            self.last_updated = Some(Utc::now().to_rfc3339());
            self.save();
        }
    }

    /// Update model download progress and optionally save
    pub fn update_download_progress(
        &mut self,
        id: &str,
        progress: DownloadProgress,
        save: bool,
    ) {
        if let Some(model) = self.get_model_mut(id) {
            model.download_progress = progress;
            if save {
                self.save();
            }
        }
    }

    /// Refresh a specific model's status
    pub fn refresh_model(&mut self, id: &str) {
        if let Some(model) = self.get_model_mut(id) {
            model.scan_filesystem();
            self.last_updated = Some(Utc::now().to_rfc3339());
            self.save();
        }
    }
}

/// Get default V2 model configurations
pub fn get_default_local_models_v2() -> Vec<LocalModelV2> {
    vec![
        LocalModelV2 {
            id: "flux-klein-4b".to_string(),
            name: "FLUX.2-klein-4B".to_string(),
            description: "4B parameter FLUX image generation model. Fast inference with 4-step generation, optimized for Apple Silicon.".to_string(),
            category: ModelCategory::Image,
            tags: vec!["image-generation".into(), "flux".into(), "apple-silicon".into(), "mlx".into()],
            source: ModelSource {
                primary_url: "https://huggingface.co/black-forest-labs/FLUX.2-klein-4B".to_string(),
                backup_urls: vec![
                    "https://hf-mirror.com/black-forest-labs/FLUX.2-klein-4B".to_string(),
                ],
                source_type: SourceType::Huggingface,
                repo_id: Some("black-forest-labs/FLUX.2-klein-4B".to_string()),
                revision: "main".to_string(),
            },
            storage: ModelStorage {
                local_path: "~/.cache/huggingface/hub/models--black-forest-labs--FLUX.2-klein-4B".to_string(),
                total_size_bytes: 13_958_643_712,
                total_size_display: "~13 GB".to_string(),
            },
            files: vec![],
            runtime: ModelRuntime {
                memory_required_mb: 16384,
                memory_peak_mb: Some(18432),
                recommended_vram_mb: Some(16384),
                supported_platforms: vec!["macos-arm64".into()],
                quantization: Some("bf16".into()),
                inference_engine: Some("mlx".into()),
            },
            status: ModelStatusInfo::default(),
            download_progress: DownloadProgress::default(),
        },
        LocalModelV2 {
            id: "zimage-turbo".to_string(),
            name: "Z-Image Turbo".to_string(),
            description: "6B parameter S3-DiT image generation. 9-step turbo inference (~3s/image), 4-bit quantized for efficient memory usage.".to_string(),
            category: ModelCategory::Image,
            tags: vec!["image-generation".into(), "s3-dit".into(), "quantized".into(), "fast".into()],
            source: ModelSource {
                primary_url: "https://huggingface.co/uqer1244/MLX-z-image".to_string(),
                backup_urls: vec![],
                source_type: SourceType::Huggingface,
                repo_id: Some("uqer1244/MLX-z-image".to_string()),
                revision: "main".to_string(),
            },
            storage: ModelStorage {
                local_path: "~/.cache/huggingface/hub/models--uqer1244--MLX-z-image".to_string(),
                total_size_bytes: 12_884_901_888,
                total_size_display: "~12 GB".to_string(),
            },
            files: vec![],
            runtime: ModelRuntime {
                memory_required_mb: 14336,
                memory_peak_mb: None,
                recommended_vram_mb: None,
                supported_platforms: vec!["macos-arm64".into()],
                quantization: Some("4bit".into()),
                inference_engine: Some("mlx".into()),
            },
            status: ModelStatusInfo::default(),
            download_progress: DownloadProgress::default(),
        },
        LocalModelV2 {
            id: "qwen3-8b".to_string(),
            name: "Qwen3 8B".to_string(),
            description: "Powerful 8B parameter language model. Excellent for chat, coding assistance, and general text generation.".to_string(),
            category: ModelCategory::Llm,
            tags: vec!["llm".into(), "chat".into(), "coding".into(), "qwen".into()],
            source: ModelSource {
                primary_url: "https://huggingface.co/mlx-community/Qwen3-8B-bf16".to_string(),
                backup_urls: vec![],
                source_type: SourceType::Huggingface,
                repo_id: Some("mlx-community/Qwen3-8B-bf16".to_string()),
                revision: "main".to_string(),
            },
            storage: ModelStorage {
                local_path: "~/.cache/huggingface/hub/models--mlx-community--Qwen3-8B-bf16".to_string(),
                total_size_bytes: 17_179_869_184,
                total_size_display: "~16 GB".to_string(),
            },
            files: vec![],
            runtime: ModelRuntime {
                memory_required_mb: 18432,
                memory_peak_mb: None,
                recommended_vram_mb: None,
                supported_platforms: vec!["macos-arm64".into()],
                quantization: Some("bf16".into()),
                inference_engine: Some("mlx".into()),
            },
            status: ModelStatusInfo::default(),
            download_progress: DownloadProgress::default(),
        },
        LocalModelV2 {
            id: "funasr-paraformer".to_string(),
            name: "FunASR Paraformer".to_string(),
            description: "High-accuracy Chinese ASR. Downloads PyTorch model and auto-converts to MLX format.".to_string(),
            category: ModelCategory::Asr,
            tags: vec!["asr".into(), "chinese".into(), "speech-recognition".into(), "funasr".into()],
            source: ModelSource {
                primary_url: "https://modelscope.cn/models/damo/speech_seaco_paraformer_large_asr_nat-zh-cn-16k-common-vocab8404-pytorch".to_string(),
                backup_urls: vec![],
                source_type: SourceType::Modelscope,
                repo_id: Some("damo/speech_seaco_paraformer_large_asr_nat-zh-cn-16k-common-vocab8404-pytorch".to_string()),
                revision: "master".to_string(),
            },
            storage: ModelStorage {
                local_path: "~/.dora/models/paraformer".to_string(),
                total_size_bytes: 1_073_741_824,
                total_size_display: "~1.0 GB".to_string(),
            },
            files: vec![],
            runtime: ModelRuntime {
                memory_required_mb: 2048,
                memory_peak_mb: None,
                recommended_vram_mb: None,
                supported_platforms: vec!["macos-arm64".into()],
                quantization: None,
                inference_engine: Some("mlx".into()),
            },
            status: ModelStatusInfo::default(),
            download_progress: DownloadProgress::default(),
        },
        LocalModelV2 {
            id: "funasr-nano".to_string(),
            name: "FunASR Nano".to_string(),
            description: "800M LLM-based ASR supporting 31 languages. Combines Whisper encoder with Qwen LLM for high accuracy.".to_string(),
            category: ModelCategory::Asr,
            tags: vec!["asr".into(), "multilingual".into(), "speech-recognition".into(), "whisper".into(), "qwen".into()],
            source: ModelSource {
                primary_url: "https://huggingface.co/mlx-community/Fun-ASR-Nano-2512-fp16".to_string(),
                backup_urls: vec![],
                source_type: SourceType::Huggingface,
                repo_id: Some("mlx-community/Fun-ASR-Nano-2512-fp16".to_string()),
                revision: "main".to_string(),
            },
            storage: ModelStorage {
                local_path: "~/.dora/models/funasr-nano".to_string(),
                total_size_bytes: 2_040_109_465,
                total_size_display: "~1.9 GB".to_string(),
            },
            files: vec![],
            runtime: ModelRuntime {
                memory_required_mb: 3072,
                memory_peak_mb: None,
                recommended_vram_mb: None,
                supported_platforms: vec!["macos-arm64".into()],
                quantization: Some("fp16".into()),
                inference_engine: Some("mlx".into()),
            },
            status: ModelStatusInfo::default(),
            download_progress: DownloadProgress::default(),
        },
    ]
}
