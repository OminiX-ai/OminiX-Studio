//! Runtime client for the ominix-api load/unload/status endpoints.
//!
//! All requests go to localhost:8080 (ominix-api) and follow the API contracts:
//!
//!   GET  /v1/models               → list + status of every loaded model
//!   POST /v1/models/{id}/load     → load a model into memory (blocks until done)
//!   POST /v1/models/{id}/unload   → free the model from memory

use serde::Deserialize;

// ─── Server-side model status ─────────────────────────────────────────────────

/// Status as reported by the ominix-api `/v1/models` endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerModelStatus {
    Loaded,
    Loading,
    Unloaded,
    Error,
}

impl ServerModelStatus {
    fn from_str(s: &str) -> Self {
        match s {
            "loaded"   => Self::Loaded,
            "loading"  => Self::Loading,
            "error"    => Self::Error,
            _          => Self::Unloaded,
        }
    }
}

/// One entry from `GET /v1/models`.
#[derive(Debug, Clone)]
pub struct ServerModelInfo {
    /// The model ID as known to the API (= RegistryRuntime::api_model_id)
    pub api_id:    String,
    pub status:    ServerModelStatus,
    pub memory_gb: Option<f32>,
}

// ─── Deserialisation helpers ──────────────────────────────────────────────────

#[derive(Deserialize)]
struct ModelsListResponse {
    data: Vec<ModelEntry>,
}

#[derive(Deserialize)]
struct ModelEntry {
    id: String,
    #[serde(default)]
    status: String,
    #[serde(default)]
    memory_gb: Option<f32>,
}

// ─── Client ───────────────────────────────────────────────────────────────────

/// Thin blocking HTTP client for the ominix-api runtime endpoints.
///
/// All calls block the calling thread — run them inside `std::thread::spawn`.
pub struct ModelRuntimeClient {
    base_url: String,
}

impl ModelRuntimeClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        let mut url = base_url.into();
        if url.ends_with('/') {
            url.pop();
        }
        Self { base_url: url }
    }

    pub fn localhost() -> Self {
        Self::new("http://localhost:8080")
    }

    // ── List ─────────────────────────────────────────────────────────────────

    /// `GET /v1/models` — returns status for every model known to the server.
    pub fn list_models(&self) -> Result<Vec<ServerModelInfo>, String> {
        let client = self.client(5)?;
        let url    = format!("{}/v1/models", self.base_url);
        let resp   = client.get(&url).send().map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("HTTP {}", resp.status()));
        }

        let body: ModelsListResponse = resp.json().map_err(|e| e.to_string())?;
        Ok(body.data.into_iter().map(|e| ServerModelInfo {
            api_id:    e.id,
            status:    ServerModelStatus::from_str(&e.status),
            memory_gb: e.memory_gb,
        }).collect())
    }

    // ── Load ──────────────────────────────────────────────────────────────────

    /// `POST /v1/models/load` — blocks until the model is ready.
    /// Large models may take several minutes.
    /// `model_type`: "llm", "vlm", "asr", "tts", or "image"
    pub fn load_model(&self, api_model_id: &str, model_type: &str) -> Result<(), String> {
        let client = self.client(600)?;          // 10-minute ceiling
        let url    = format!("{}/v1/models/load", self.base_url);
        let body   = serde_json::json!({ "model": api_model_id, "model_type": model_type });
        let resp   = client.post(&url).json(&body).send().map_err(|e| e.to_string())?;

        if resp.status().is_success() {
            Ok(())
        } else {
            let status = resp.status();
            let text   = resp.text().unwrap_or_default();
            Err(format!("HTTP {} — {}", status, text.trim()))
        }
    }

    // ── Unload ────────────────────────────────────────────────────────────────

    /// `POST /v1/models/unload` — frees the model from memory.
    /// `model_type`: "llm", "vlm", "asr", "tts", "image", or "all"
    pub fn unload_model(&self, model_type: &str) -> Result<(), String> {
        let client = self.client(30)?;
        let url    = format!("{}/v1/models/unload", self.base_url);
        let body   = serde_json::json!({ "model_type": model_type });
        let resp   = client.post(&url).json(&body).send().map_err(|e| e.to_string())?;

        if resp.status().is_success() {
            Ok(())
        } else {
            let status = resp.status();
            let text   = resp.text().unwrap_or_default();
            Err(format!("HTTP {} — {}", status, text.trim()))
        }
    }

    // ── Internal ─────────────────────────────────────────────────────────────

    fn client(&self, timeout_secs: u64) -> Result<reqwest::blocking::Client, String> {
        reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(timeout_secs))
            .build()
            .map_err(|e| e.to_string())
    }
}
