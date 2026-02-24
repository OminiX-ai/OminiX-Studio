pub mod design;

use makepad_widgets::*;
use moly_data::{
    LocalModelsConfigV2, LocalModelV2, ModelState, DownloadProgress, SourceType, ModelCategory,
};
use serde::Deserialize;
use std::sync::{Arc, atomic::{AtomicBool, AtomicU64, Ordering}};
use std::collections::HashMap;

/// A row in the flat list fed to PortalList — either a category header or a model item.
enum ListRow {
    Header(ModelCategory),
    /// Index into `LocalModelsConfigV2::models`
    Model(usize),
}

/// HuggingFace API file/directory item
#[derive(Debug, Deserialize)]
struct HuggingFaceItem {
    #[serde(rename = "type")]
    item_type: String,
    path: String,
    size: Option<u64>,
}

/// ModelScope API response
#[derive(Debug, Deserialize)]
struct ModelScopeResponse {
    #[serde(rename = "Code")]
    code: i32,
    #[serde(rename = "Data")]
    data: Option<ModelScopeData>,
}

#[derive(Debug, Deserialize)]
struct ModelScopeData {
    #[serde(rename = "Files")]
    files: Vec<ModelScopeFile>,
}

#[derive(Debug, Deserialize)]
struct ModelScopeFile {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Path")]
    path: String,
    #[serde(rename = "Size")]
    size: u64,
    #[serde(rename = "Type")]
    file_type: String, // "blob" or "tree"
}

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use crate::screen::design::*;
}

/// Per-model download state shared between UI and download thread
#[derive(Clone)]
struct ModelDownloadState {
    model_id: String,
    is_downloading: Arc<AtomicBool>,
    cancel_requested: Arc<AtomicBool>,
    progress_bytes: Arc<AtomicU64>,
    total_bytes: Arc<AtomicU64>,
    current_file: Arc<std::sync::Mutex<Option<String>>>,
    current_file_index: Arc<AtomicU64>,
    total_files: Arc<AtomicU64>,
    completed: Arc<AtomicBool>,
    error: Arc<std::sync::Mutex<Option<String>>>,
}

impl ModelDownloadState {
    fn new(model_id: &str) -> Self {
        Self {
            model_id: model_id.to_string(),
            is_downloading: Arc::new(AtomicBool::new(false)),
            cancel_requested: Arc::new(AtomicBool::new(false)),
            progress_bytes: Arc::new(AtomicU64::new(0)),
            total_bytes: Arc::new(AtomicU64::new(0)),
            current_file: Arc::new(std::sync::Mutex::new(None)),
            current_file_index: Arc::new(AtomicU64::new(0)),
            total_files: Arc::new(AtomicU64::new(0)),
            completed: Arc::new(AtomicBool::new(false)),
            error: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    fn reset(&self) {
        self.is_downloading.store(false, Ordering::SeqCst);
        self.cancel_requested.store(false, Ordering::SeqCst);
        self.progress_bytes.store(0, Ordering::SeqCst);
        self.total_bytes.store(0, Ordering::SeqCst);
        self.current_file_index.store(0, Ordering::SeqCst);
        self.total_files.store(0, Ordering::SeqCst);
        self.completed.store(false, Ordering::SeqCst);
        *self.current_file.lock().unwrap() = None;
        *self.error.lock().unwrap() = None;
    }

    fn progress_percent(&self) -> f64 {
        let total = self.total_bytes.load(Ordering::SeqCst);
        if total == 0 {
            return 0.0;
        }
        let progress = self.progress_bytes.load(Ordering::SeqCst);
        (progress as f64 / total as f64).min(1.0)
    }

    fn to_download_progress(&self) -> DownloadProgress {
        DownloadProgress {
            is_active: self.is_downloading.load(Ordering::SeqCst),
            current_file: self.current_file.lock().unwrap().clone(),
            current_file_index: self.current_file_index.load(Ordering::SeqCst) as usize,
            current_file_bytes: 0, // Not tracked separately
            current_file_total: 0,
            overall_bytes: self.progress_bytes.load(Ordering::SeqCst),
            overall_total: self.total_bytes.load(Ordering::SeqCst),
            speed_bytes_per_sec: 0, // Could add speed tracking
            eta_seconds: None,
            started_at: None,
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct LocalModelsApp {
    #[deref]
    pub view: View,

    #[rust]
    config: Option<LocalModelsConfigV2>,

    #[rust]
    selected_model_index: Option<usize>,

    #[rust]
    initialized: bool,

    /// Per-model download states (model_id -> state)
    #[rust]
    download_states: HashMap<String, ModelDownloadState>,

    /// Flat list of rows for the PortalList: interleaved category headers and model indices
    #[rust]
    flat_list: Vec<ListRow>,
}

impl Widget for LocalModelsApp {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // Initialize config
        if !self.initialized {
            self.config = Some(LocalModelsConfigV2::load());
            self.selected_model_index = Some(0);
            self.initialized = true;
            self.download_states = HashMap::new();
            self.rebuild_flat_list();
            ::log::info!("Loaded local models V2 config with {} models",
                self.config.as_ref().map(|c| c.models.len()).unwrap_or(0));
        }

        // Handle events
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        // Handle model list item clicks
        self.handle_model_list_clicks(cx, &actions);

        // Handle download button click
        if self.view.button(ids!(download_button)).clicked(&actions) {
            if let (Some(config), Some(idx)) = (&self.config, self.selected_model_index) {
                if idx < config.models.len() {
                    let model = &config.models[idx];
                    // Check if this model is already downloading
                    let is_downloading = self.download_states
                        .get(&model.id)
                        .map(|s| s.is_downloading.load(Ordering::SeqCst))
                        .unwrap_or(false);

                    if !is_downloading {
                        self.start_download(cx, idx);
                    }
                }
            }
        }

        // Handle cancel button click
        if self.view.button(ids!(cancel_button)).clicked(&actions) {
            if let (Some(config), Some(idx)) = (&self.config, self.selected_model_index) {
                if idx < config.models.len() {
                    let model_id = &config.models[idx].id;
                    if let Some(state) = self.download_states.get(model_id) {
                        if state.is_downloading.load(Ordering::SeqCst) {
                            state.cancel_requested.store(true, Ordering::SeqCst);
                            self.view.label(ids!(status_message)).set_text(cx, "Cancelling download...");
                            self.view.redraw(cx);
                        }
                    }
                }
            }
        }

        // Handle remove button click
        if self.view.button(ids!(remove_button)).clicked(&actions) {
            if let Some(idx) = self.selected_model_index {
                // Check if model is downloaded before removing
                let is_ready = self.config.as_ref()
                    .and_then(|c| c.models.get(idx))
                    .map(|m| m.status.state == ModelState::Ready)
                    .unwrap_or(false);

                if is_ready {
                    self.remove_model_files(cx, idx);
                } else {
                    self.view.label(ids!(status_message)).set_text(
                        cx, "Model is not downloaded"
                    );
                    self.view.redraw(cx);
                }
            }
        }

        // Handle refresh button click
        if self.view.button(ids!(refresh_button)).clicked(&actions) {
            if let Some(config) = &mut self.config {
                config.startup_scan();
                config.save();
                ::log::info!("Refreshed model availability");
                self.view.label(ids!(status_message)).set_text(cx, "Model status refreshed");
            }
            self.rebuild_flat_list();
            self.view.redraw(cx);
        }

        // Check for download completion or progress updates for all active downloads
        let mut any_downloading = false;
        let mut completed_ids = Vec::new();

        for (model_id, state) in &self.download_states {
            if state.is_downloading.load(Ordering::SeqCst) {
                any_downloading = true;
                if state.completed.load(Ordering::SeqCst) {
                    completed_ids.push(model_id.clone());
                }
            }
        }

        // Handle completed downloads
        for model_id in completed_ids {
            self.handle_download_complete(cx, &model_id);
        }

        if any_downloading {
            // Request next frame to keep updating progress
            cx.new_next_frame();
            // Request redraw to update progress bars
            self.view.redraw(cx);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Get dark mode from scope
        let dark_mode = scope
            .data
            .get::<moly_data::Store>()
            .map(|s| if s.is_dark_mode() { 1.0 } else { 0.0 })
            .unwrap_or(0.0);

        // Apply dark mode to all components
        self.apply_dark_mode(cx, dark_mode);

        // Update progress bar if downloading
        self.update_progress_bar(cx, dark_mode);

        // Update right panel with selected model BEFORE drawing
        if let Some(config) = &self.config {
            if let Some(idx) = self.selected_model_index {
                if idx < config.models.len() {
                    let model = config.models[idx].clone();
                    self.update_model_details(cx, &model, dark_mode);
                }
            }
        }

        // Get PortalList widget UID for step pattern
        let models_list = self.view.portal_list(ids!(models_list));
        let models_list_uid = models_list.widget_uid();

        // Draw with PortalList handling
        while let Some(widget) = self.view.draw_walk(cx, scope, walk).step() {
            if widget.widget_uid() == models_list_uid {
                self.draw_models_list(cx, scope, widget, dark_mode);
            }
        }

        DrawStep::done()
    }
}

impl LocalModelsApp {
    /// Start downloading a model (V2 - per-model progress)
    fn start_download(&mut self, cx: &mut Cx, model_index: usize) {
        let Some(config) = &mut self.config else { return };
        if model_index >= config.models.len() { return; }

        let model = &config.models[model_index];
        let model_id = model.id.clone();
        let model_name = model.name.clone();
        let url = model.source.primary_url.clone();
        let backup_urls = model.source.backup_urls.clone();
        let dest_path = model.expanded_path();
        let source_type = model.source.source_type;

        ::log::info!("Starting download for model: {} from {}", model_name, url);

        // Update model status to Downloading
        config.models[model_index].status.state = ModelState::Downloading;
        config.save();

        // Create or reset download state for this model
        let state = ModelDownloadState::new(&model_id);
        state.is_downloading.store(true, Ordering::SeqCst);
        self.download_states.insert(model_id.clone(), state.clone());

        // Update UI
        self.view.label(ids!(status_message)).set_text(
            cx, &format!("Downloading {}...", model_name)
        );
        self.view.redraw(cx);

        // Spawn download thread
        std::thread::spawn(move || {
            // Try primary URL first, then backups
            let all_urls: Vec<String> = std::iter::once(url.clone())
                .chain(backup_urls.into_iter())
                .collect();

            let mut last_error = String::new();

            for (url_index, download_url) in all_urls.iter().enumerate() {
                if url_index > 0 {
                    ::log::info!("Trying backup URL {}: {}", url_index, download_url);
                }

                let result = Self::download_model_blocking(&state, download_url, &dest_path, source_type);

                match result {
                    Ok(_) => {
                        state.completed.store(true, Ordering::SeqCst);
                        return;
                    }
                    Err(e) => {
                        // Check if cancelled
                        if state.cancel_requested.load(Ordering::SeqCst) {
                            *state.error.lock().unwrap() = Some("Download cancelled".to_string());
                            state.completed.store(true, Ordering::SeqCst);
                            return;
                        }
                        last_error = e;
                        ::log::warn!("Download failed from {}: {}", download_url, last_error);
                    }
                }
            }

            // All URLs failed
            *state.error.lock().unwrap() = Some(last_error);
            state.completed.store(true, Ordering::SeqCst);
        });
    }

    /// Blocking download function that runs in a separate thread
    fn download_model_blocking(
        state: &ModelDownloadState,
        url: &str,
        dest_path: &str,
        source_type: SourceType,
    ) -> Result<(), String> {
        // Create HTTP client
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(3600)) // 1 hour timeout for large files
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

        // Create destination directory
        std::fs::create_dir_all(dest_path)
            .map_err(|e| format!("Failed to create model directory: {}", e))?;

        // Determine source and download based on source_type
        match source_type {
            SourceType::Local => {
                Err("This model requires manual installation. See the model description for instructions.".to_string())
            }
            SourceType::Modelscope => Self::download_from_modelscope(&client, state, url, dest_path),
            SourceType::Huggingface | SourceType::DirectUrl => {
                Self::download_from_huggingface(&client, state, url, dest_path)
            }
        }
    }

    /// Download from HuggingFace
    fn download_from_huggingface(
        client: &reqwest::blocking::Client,
        state: &ModelDownloadState,
        url: &str,
        dest_path: &str,
    ) -> Result<(), String> {
        let repo_id = Self::parse_huggingface_repo_id(url)?;
        let token = Self::read_hf_token();
        ::log::info!("Downloading HuggingFace repo: {} to {} (auth: {})", repo_id, dest_path, token.is_some());

        // Get list of files from HuggingFace API (with token for private repos)
        let files = Self::list_huggingface_files(client, &repo_id, "", token.as_deref())?;

        // Calculate total size and set file count
        let total_size: u64 = files.iter().map(|(_, size)| *size).sum();
        state.total_bytes.store(total_size, Ordering::SeqCst);
        state.total_files.store(files.len() as u64, Ordering::SeqCst);
        ::log::info!("Total download size: {} bytes ({} files)", total_size, files.len());

        // Download each file
        let mut downloaded_bytes: u64 = 0;
        for (file_index, (file_path, _file_size)) in files.iter().enumerate() {
            if state.cancel_requested.load(Ordering::SeqCst) {
                let _ = std::fs::remove_dir_all(dest_path);
                return Err("Download cancelled".to_string());
            }

            // Update current file info
            state.current_file_index.store(file_index as u64, Ordering::SeqCst);
            *state.current_file.lock().unwrap() = Some(file_path.clone());

            let local_path = std::path::Path::new(dest_path).join(file_path);
            if let Some(parent) = local_path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create directory: {}", e))?;
            }

            let download_url = format!(
                "https://huggingface.co/{}/resolve/main/{}",
                repo_id, file_path
            );
            ::log::info!("Downloading [{}/{}]: {}", file_index + 1, files.len(), file_path);
            Self::download_file_streaming(client, &download_url, &local_path, state, &mut downloaded_bytes, token.as_deref())?;
        }

        ::log::info!("Download complete: {}", dest_path);
        Ok(())
    }

    /// Download from ModelScope (with automatic PyTorch to MLX conversion for Paraformer)
    fn download_from_modelscope(
        client: &reqwest::blocking::Client,
        state: &ModelDownloadState,
        url: &str,
        dest_path: &str,
    ) -> Result<(), String> {
        let model_id = Self::parse_modelscope_model_id(url)?;
        let is_paraformer = url.contains("paraformer");

        // For Paraformer, download to temp dir first, then convert
        let download_dir = if is_paraformer {
            let temp_dir = std::env::temp_dir().join("moly-paraformer-download");
            temp_dir.to_string_lossy().to_string()
        } else {
            dest_path.to_string()
        };

        ::log::info!("Downloading ModelScope model: {} to {}", model_id, download_dir);

        // Create download directory
        std::fs::create_dir_all(&download_dir)
            .map_err(|e| format!("Failed to create directory: {}", e))?;

        // Get list of files from ModelScope API
        let files = Self::list_modelscope_files(client, &model_id, "")?;

        // Calculate total size (add 10% for conversion overhead)
        let download_size: u64 = files.iter().map(|(_, size)| *size).sum();
        let total_size = if is_paraformer { download_size + download_size / 10 } else { download_size };
        state.total_bytes.store(total_size, Ordering::SeqCst);
        state.total_files.store(files.len() as u64, Ordering::SeqCst);
        ::log::info!("Total download size: {} bytes ({} files)", download_size, files.len());

        // Download each file
        let mut downloaded_bytes: u64 = 0;
        for (file_index, (file_path, _file_size)) in files.iter().enumerate() {
            if state.cancel_requested.load(Ordering::SeqCst) {
                let _ = std::fs::remove_dir_all(&download_dir);
                return Err("Download cancelled".to_string());
            }

            // Update current file info
            state.current_file_index.store(file_index as u64, Ordering::SeqCst);
            *state.current_file.lock().unwrap() = Some(file_path.clone());

            let local_path = std::path::Path::new(&download_dir).join(file_path);
            if let Some(parent) = local_path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create directory: {}", e))?;
            }

            let download_url = format!(
                "https://modelscope.cn/models/{}/resolve/master/{}",
                model_id, file_path
            );
            ::log::info!("Downloading [{}/{}]: {}", file_index + 1, files.len(), file_path);
            Self::download_file_streaming(client, &download_url, &local_path, state, &mut downloaded_bytes, None)?;
        }

        ::log::info!("Download complete: {}", download_dir);

        // Convert Paraformer model from PyTorch to MLX format
        if is_paraformer {
            ::log::info!("Converting Paraformer model to MLX format...");

            let input_dir = std::path::Path::new(&download_dir);
            let output_dir = std::path::Path::new(dest_path);

            // Create output directory
            std::fs::create_dir_all(output_dir)
                .map_err(|e| format!("Failed to create output directory: {}", e))?;

            // Run conversion
            let (converted, unmapped) = mlx_rs_core::convert::convert_paraformer(input_dir, output_dir)
                .map_err(|e| format!("Conversion failed: {}", e))?;

            ::log::info!("Converted {} tensors ({} unmapped)", converted, unmapped);

            // Update progress to 100%
            state.progress_bytes.store(total_size, Ordering::SeqCst);

            // Clean up temp directory
            let _ = std::fs::remove_dir_all(&download_dir);

            ::log::info!("Paraformer conversion complete: {}", dest_path);
        }

        Ok(())
    }

    /// Parse ModelScope URL to extract model ID
    fn parse_modelscope_model_id(url: &str) -> Result<String, String> {
        // Format: https://modelscope.cn/models/{org}/{model}
        let url = url.trim_end_matches('/');

        if let Some(stripped) = url.strip_prefix("https://modelscope.cn/models/") {
            let parts: Vec<&str> = stripped.split('/').collect();
            if parts.len() >= 2 {
                return Ok(format!("{}/{}", parts[0], parts[1]));
            }
        }

        Err(format!("Invalid ModelScope URL: {}", url))
    }

    /// List files in a ModelScope repository recursively
    fn list_modelscope_files(
        client: &reqwest::blocking::Client,
        model_id: &str,
        path_prefix: &str,
    ) -> Result<Vec<(String, u64)>, String> {
        let api_url = if path_prefix.is_empty() {
            format!("https://modelscope.cn/api/v1/models/{}/repo/files", model_id)
        } else {
            format!("https://modelscope.cn/api/v1/models/{}/repo/files?Root={}", model_id, path_prefix)
        };

        ::log::debug!("Listing ModelScope files from: {}", api_url);

        let response = client.get(&api_url)
            .header("User-Agent", "moly-local-models/1.0")
            .send()
            .map_err(|e| format!("Failed to list files: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Failed to list files: HTTP {}", response.status()));
        }

        let api_response: ModelScopeResponse = response.json()
            .map_err(|e| format!("Failed to parse file list: {}", e))?;

        if api_response.code != 200 {
            return Err(format!("ModelScope API error: code {}", api_response.code));
        }

        let data = api_response.data.ok_or("No data in ModelScope response")?;

        let mut files = Vec::new();
        for item in data.files {
            if item.file_type == "blob" {
                files.push((item.path, item.size));
            } else if item.file_type == "tree" {
                // Recursively list subdirectory
                let sub_files = Self::list_modelscope_files(client, model_id, &item.path)?;
                files.extend(sub_files);
            }
        }

        Ok(files)
    }

    /// Download a file with streaming and progress tracking
    fn download_file_streaming(
        client: &reqwest::blocking::Client,
        url: &str,
        local_path: &std::path::Path,
        state: &ModelDownloadState,
        downloaded_bytes: &mut u64,
        token: Option<&str>,
    ) -> Result<(), String> {
        let mut req = client.get(url)
            .header("User-Agent", "moly-local-models/1.0");
        if let Some(tok) = token {
            req = req.header("Authorization", format!("Bearer {}", tok));
        }
        let response = req.send()
            .map_err(|e| format!("Failed to download: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Failed to download: HTTP {}", response.status()));
        }

        let mut file = std::fs::File::create(local_path)
            .map_err(|e| format!("Failed to create file: {}", e))?;

        use std::io::{Read, Write};
        let mut reader = response;
        let mut buffer = [0u8; 8192];

        loop {
            if state.cancel_requested.load(Ordering::SeqCst) {
                return Err("Download cancelled".to_string());
            }

            let bytes_read = reader.read(&mut buffer)
                .map_err(|e| format!("Failed to read data: {}", e))?;

            if bytes_read == 0 {
                break;
            }

            file.write_all(&buffer[..bytes_read])
                .map_err(|e| format!("Failed to write data: {}", e))?;

            *downloaded_bytes += bytes_read as u64;
            state.progress_bytes.store(*downloaded_bytes, Ordering::SeqCst);
        }

        Ok(())
    }

    /// Read HuggingFace auth token from HF_TOKEN env var or ~/.cache/huggingface/token
    fn read_hf_token() -> Option<String> {
        if let Ok(token) = std::env::var("HF_TOKEN") {
            let t = token.trim().to_string();
            if !t.is_empty() { return Some(t); }
        }
        if let Ok(home) = std::env::var("HOME") {
            let path = std::path::Path::new(&home).join(".cache/huggingface/token");
            if let Ok(tok) = std::fs::read_to_string(&path) {
                let t = tok.trim().to_string();
                if !t.is_empty() { return Some(t); }
            }
        }
        None
    }

    /// Parse HuggingFace URL to extract repo ID (org/repo)
    fn parse_huggingface_repo_id(url: &str) -> Result<String, String> {
        let url = url.trim_end_matches('/');
        if let Some(stripped) = url.strip_prefix("https://huggingface.co/") {
            let parts: Vec<&str> = stripped.split('/').collect();
            if parts.len() >= 2 {
                return Ok(format!("{}/{}", parts[0], parts[1]));
            }
        }
        Err(format!("Invalid HuggingFace URL: {}", url))
    }

    /// List files in a HuggingFace repository recursively
    fn list_huggingface_files(
        client: &reqwest::blocking::Client,
        repo_id: &str,
        path_prefix: &str,
        token: Option<&str>,
    ) -> Result<Vec<(String, u64)>, String> {
        let api_url = if path_prefix.is_empty() {
            format!("https://huggingface.co/api/models/{}/tree/main", repo_id)
        } else {
            format!("https://huggingface.co/api/models/{}/tree/main/{}", repo_id, path_prefix)
        };

        let mut req = client.get(&api_url)
            .header("User-Agent", "moly-local-models/1.0");
        if let Some(tok) = token {
            req = req.header("Authorization", format!("Bearer {}", tok));
        }
        let response = req.send()
            .map_err(|e| format!("Failed to list files: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Failed to list files: HTTP {}", response.status()));
        }

        let items: Vec<HuggingFaceItem> = response.json()
            .map_err(|e| format!("Failed to parse file list: {}", e))?;

        let mut files = Vec::new();
        for item in items {
            if item.item_type == "file" {
                files.push((item.path, item.size.unwrap_or(0)));
            } else if item.item_type == "directory" {
                let sub_files = Self::list_huggingface_files(client, repo_id, &item.path, token)?;
                files.extend(sub_files);
            }
        }

        Ok(files)
    }

    /// Handle download completion for a specific model
    fn handle_download_complete(&mut self, cx: &mut Cx, model_id: &str) {
        let Some(state) = self.download_states.get(model_id) else { return };

        let error = state.error.lock().unwrap().take();
        let was_cancelled = state.cancel_requested.load(Ordering::SeqCst);

        state.is_downloading.store(false, Ordering::SeqCst);

        if let Some(config) = &mut self.config {
            if let Some(model) = config.models.iter_mut().find(|m| m.id == model_id) {
                let model_name = model.name.clone();

                if let Some(err) = error {
                    ::log::error!("Download failed for {}: {}", model_name, err);
                    model.status.state = ModelState::Error;
                    model.status.error_message = Some(err.clone());
                    model.download_progress.fail();
                    config.save();
                    self.view.label(ids!(status_message)).set_text(
                        cx, &format!("Download failed: {}", err)
                    );
                } else if was_cancelled {
                    ::log::info!("Download cancelled for {}", model_name);
                    model.status.state = ModelState::NotAvailable;
                    model.download_progress.fail();
                    config.save();
                    self.view.label(ids!(status_message)).set_text(
                        cx, "Download cancelled"
                    );
                } else {
                    ::log::info!("Download completed for {}", model_name);
                    model.status.state = ModelState::Ready;
                    model.status.last_downloaded = Some(chrono::Utc::now().to_rfc3339());
                    model.download_progress.complete();
                    // Scan to update file counts
                    model.scan_filesystem();
                    config.save();
                    self.view.label(ids!(status_message)).set_text(
                        cx, &format!("Successfully downloaded {}", model_name)
                    );
                }
            }
        }

        self.view.redraw(cx);
    }

    /// Update the progress bar UI for the currently selected model
    fn update_progress_bar(&mut self, cx: &mut Cx2d, dark_mode: f64) {
        // Get selected model's download state
        let (is_downloading, progress, progress_bytes, total_bytes, current_file) = self.config
            .as_ref()
            .and_then(|c| self.selected_model_index.and_then(|idx| c.models.get(idx)))
            .and_then(|model| {
                self.download_states.get(&model.id).map(|state| {
                    let is_dl = state.is_downloading.load(Ordering::SeqCst);
                    let prog = state.progress_percent();
                    let prog_bytes = state.progress_bytes.load(Ordering::SeqCst);
                    let total = state.total_bytes.load(Ordering::SeqCst);
                    let file = state.current_file.lock().unwrap().clone();
                    (is_dl, prog, prog_bytes, total, file)
                })
            })
            .unwrap_or((false, 0.0, 0, 0, None));

        // Show/hide progress section
        self.view.view(ids!(progress_section)).apply_over(cx, live! {
            visible: (is_downloading)
        });

        // Show/hide buttons based on download state
        self.view.button(ids!(download_button)).apply_over(cx, live! {
            visible: (!is_downloading)
        });
        self.view.button(ids!(cancel_button)).apply_over(cx, live! {
            visible: (is_downloading)
        });

        if is_downloading {
            // Update progress bar fill width (as percentage of parent)
            let fill_width = (progress * 300.0) as i64; // Assuming ~300px width
            self.view.view(ids!(progress_bar_fill)).apply_over(cx, live! {
                width: (fill_width)
            });

            // Update progress text with current file info
            let progress_text = if let Some(file) = current_file {
                format!(
                    "{:.1}% ({} / {}) - {}",
                    progress * 100.0,
                    format_bytes(progress_bytes),
                    format_bytes(total_bytes),
                    file
                )
            } else {
                format!(
                    "{:.1}% ({} / {})",
                    progress * 100.0,
                    format_bytes(progress_bytes),
                    format_bytes(total_bytes)
                )
            };
            self.view.label(ids!(progress_text)).set_text(cx, &progress_text);
            self.view.label(ids!(progress_text)).apply_over(cx, live! {
                draw_text: { dark_mode: (dark_mode) }
            });

            // Apply dark mode to progress bar
            self.view.view(ids!(progress_bar_bg)).apply_over(cx, live! {
                draw_bg: { dark_mode: (dark_mode) }
            });
            self.view.view(ids!(progress_bar_fill)).apply_over(cx, live! {
                draw_bg: { dark_mode: (dark_mode) }
            });
        }
    }

    /// Rebuild the flat list of rows (category headers + model indices) used by PortalList.
    /// Must be called whenever the set of models changes.
    fn rebuild_flat_list(&mut self) {
        let Some(config) = &self.config else {
            self.flat_list.clear();
            return;
        };

        let categories = [
            ModelCategory::Llm,
            ModelCategory::Image,
            ModelCategory::Asr,
            ModelCategory::Tts,
        ];

        let mut flat = Vec::new();
        for cat in &categories {
            let indices: Vec<usize> = config.models.iter().enumerate()
                .filter(|(_, m)| &m.category == cat)
                .map(|(i, _)| i)
                .collect();

            if !indices.is_empty() {
                flat.push(ListRow::Header(*cat));
                for idx in indices {
                    flat.push(ListRow::Model(idx));
                }
            }
        }

        self.flat_list = flat;
    }

    /// Handle clicks on model list items
    ///
    /// Event handling strategy:
    /// - ModelListItem has `event_order: Down`, so it receives finger_down events
    ///   BEFORE its children, allowing us to capture clicks anywhere on the item
    /// - Remove button uses `.clicked()` which checks button-specific actions
    /// - We check remove button FIRST to prevent item selection when removing
    fn handle_model_list_clicks(&mut self, cx: &mut Cx, actions: &Actions) {
        let models_list = self.view.portal_list(ids!(models_list));

        for (item_id, item) in models_list.items_with_actions(actions) {
            // Map flat list position to model index; skip header rows
            let model_idx = match self.flat_list.get(item_id) {
                Some(ListRow::Model(idx)) => *idx,
                _ => continue,
            };

            if item.button(ids!(remove_item_button)).clicked(actions) {
                self.remove_model_files(cx, model_idx);
                return;
            }

            if let Some(fd) = item.as_view().finger_down(actions) {
                if fd.tap_count == 1 {
                    self.selected_model_index = Some(model_idx);
                    self.view.redraw(cx);
                }
            }
        }
    }

    /// Remove downloaded model files for a specific model
    fn remove_model_files(&mut self, cx: &mut Cx, model_index: usize) {
        let Some(config) = &mut self.config else { return };
        if model_index >= config.models.len() { return; }

        let model = &config.models[model_index];
        let model_name = model.name.clone();
        let expanded_path = model.expanded_path();

        ::log::info!("Removing model files for: {} at {}", model_name, expanded_path);

        // Try to remove the directory
        let path = std::path::Path::new(&expanded_path);
        if path.exists() {
            match std::fs::remove_dir_all(path) {
                Ok(_) => {
                    ::log::info!("Successfully removed model files: {}", expanded_path);
                    // Update model status
                    let model = &mut config.models[model_index];
                    model.status.state = ModelState::NotAvailable;
                    model.status.downloaded_bytes = 0;
                    model.status.downloaded_files = 0;
                    // Reset file download flags
                    for file in &mut model.files {
                        file.downloaded = false;
                    }
                    config.save();
                    self.view.label(ids!(status_message)).set_text(
                        cx, &format!("Removed {}", model_name)
                    );
                }
                Err(e) => {
                    ::log::error!("Failed to remove model files: {:?}", e);
                    self.view.label(ids!(status_message)).set_text(
                        cx, &format!("Failed to remove {}: {}", model_name, e)
                    );
                }
            }
        } else {
            ::log::warn!("Model path does not exist: {}", expanded_path);
            // Update status anyway since files don't exist
            let model = &mut config.models[model_index];
            model.status.state = ModelState::NotAvailable;
            model.status.downloaded_bytes = 0;
            model.status.downloaded_files = 0;
            config.save();
            self.view.label(ids!(status_message)).set_text(
                cx, &format!("{} was already removed", model_name)
            );
        }

        self.view.redraw(cx);
    }

    /// Draw the models PortalList, grouped by category
    fn draw_models_list(&mut self, cx: &mut Cx2d, scope: &mut Scope, widget: WidgetRef, dark_mode: f64) {
        let Some(config) = &self.config else { return };

        let binding = widget.as_portal_list();
        let Some(mut list) = binding.borrow_mut() else { return };

        list.set_item_range(cx, 0, self.flat_list.len());

        while let Some(item_id) = list.next_visible_item(cx) {
            match self.flat_list.get(item_id) {
                Some(ListRow::Header(cat)) => {
                    let cat = *cat;
                    let item = list.item(cx, item_id, live_id!(CategoryHeader));
                    item.apply_over(cx, live! {
                        draw_bg: { dark_mode: (dark_mode) }
                    });
                    item.label(ids!(category_header_label)).set_text(cx, cat.label());
                    item.label(ids!(category_header_label)).apply_over(cx, live! {
                        draw_text: { dark_mode: (dark_mode) }
                    });
                    item.draw_all(cx, scope);
                }
                Some(ListRow::Model(model_idx)) => {
                    let model_idx = *model_idx;
                    if model_idx >= config.models.len() { continue; }

                    let model = &config.models[model_idx];
                    let is_selected = self.selected_model_index == Some(model_idx);

                    let is_downloading = self.download_states
                        .get(&model.id)
                        .map(|s| s.is_downloading.load(Ordering::SeqCst))
                        .unwrap_or(false);

                    let download_progress = if is_downloading {
                        self.download_states.get(&model.id).map(|s| s.progress_percent()).unwrap_or(0.0)
                    } else {
                        0.0
                    };

                    let item = list.item(cx, item_id, live_id!(ModelItem));

                    item.apply_over(cx, live! {
                        draw_bg: {
                            selected: (if is_selected { 1.0 } else { 0.0 }),
                            dark_mode: (dark_mode)
                        }
                    });

                    let status_value = if is_downloading {
                        ModelState::Downloading.as_f64()
                    } else {
                        model.status.state.as_f64()
                    };
                    item.view(ids!(model_status)).apply_over(cx, live! {
                        draw_bg: {
                            status: (status_value),
                            dark_mode: (dark_mode)
                        }
                    });

                    item.label(ids!(model_name)).set_text(cx, &model.name);
                    item.label(ids!(model_name)).apply_over(cx, live! {
                        draw_text: { dark_mode: (dark_mode) }
                    });

                    // Hide category badge — redundant inside a category group
                    item.view(ids!(model_category)).set_visible(cx, false);

                    item.view(ids!(remove_button_container)).set_visible(cx, false);

                    item.view(ids!(inline_progress)).set_visible(cx, is_downloading);
                    if is_downloading {
                        item.view(ids!(inline_progress)).apply_over(cx, live! {
                            draw_bg: {
                                dark_mode: (dark_mode),
                                progress: (download_progress)
                            }
                        });
                    }

                    item.draw_all(cx, scope);
                }
                None => continue,
            }
        }
    }

    fn apply_dark_mode(&mut self, cx: &mut Cx2d, dark_mode: f64) {
        // Apply to main backgrounds
        self.view.apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode) }
        });

        self.view.view(ids!(models_panel)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode) }
        });

        self.view.view(ids!(model_view)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode) }
        });

        // Apply to labels
        self.view.label(ids!(header_label)).apply_over(cx, live! {
            draw_text: { dark_mode: (dark_mode) }
        });

        self.view.label(ids!(model_title)).apply_over(cx, live! {
            draw_text: { dark_mode: (dark_mode) }
        });

        self.view.label(ids!(model_description)).apply_over(cx, live! {
            draw_text: { dark_mode: (dark_mode) }
        });

        // Apply to info section
        self.view.view(ids!(info_section)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode) }
        });

        // Apply to buttons
        self.view.button(ids!(download_button)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode) }
        });

        self.view.button(ids!(cancel_button)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode) }
        });

        self.view.button(ids!(remove_button)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode) }
        });

        self.view.button(ids!(refresh_button)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode) }
        });
    }

    fn update_model_details(&mut self, cx: &mut Cx2d, model: &LocalModelV2, dark_mode: f64) {
        // Update title
        self.view.label(ids!(model_title)).set_text(cx, &model.name);

        // Update category badge in detail panel header
        // Note: We use explicit path (title_category -> category_label) to avoid
        // confusion with category_label in list items' model_category badges
        let category_value = model.category.as_f64();
        let title_category = self.view.view(ids!(title_category));
        title_category.apply_over(cx, live! {
            draw_bg: {
                category: (category_value),
                dark_mode: (dark_mode)
            }
        });
        // Update the label inside title_category
        title_category.label(ids!(category_label)).set_text(cx, model.category.label());
        title_category.label(ids!(category_label)).apply_over(cx, live! {
            draw_text: {
                category: (category_value),
                dark_mode: (dark_mode)
            }
        });

        // Update description
        self.view.label(ids!(model_description)).set_text(cx, &model.description);

        // Check if downloading to show appropriate status
        let is_downloading = self.download_states
            .get(&model.id)
            .map(|s| s.is_downloading.load(Ordering::SeqCst))
            .unwrap_or(false);

        // Update status row
        let status_row = self.view.view(ids!(status_row));
        let status_text = if is_downloading {
            ModelState::Downloading.label()
        } else {
            model.status.state.label()
        };
        status_row.label(ids!(info_value)).set_text(cx, status_text);
        status_row.label(ids!(info_label)).apply_over(cx, live! {
            draw_text: { dark_mode: (dark_mode) }
        });
        status_row.label(ids!(info_value)).apply_over(cx, live! {
            draw_text: { dark_mode: (dark_mode) }
        });

        // Update size row with more details
        let size_text = if model.status.downloaded_bytes > 0 && model.storage.total_size_bytes > 0 {
            format!(
                "{} / {} ({} files)",
                format_bytes(model.status.downloaded_bytes),
                model.storage.total_size_display,
                model.status.downloaded_files
            )
        } else {
            model.storage.total_size_display.clone()
        };
        let size_row = self.view.view(ids!(size_row));
        size_row.label(ids!(info_value)).set_text(cx, &size_text);
        size_row.label(ids!(info_label)).apply_over(cx, live! {
            draw_text: { dark_mode: (dark_mode) }
        });
        size_row.label(ids!(info_value)).apply_over(cx, live! {
            draw_text: { dark_mode: (dark_mode) }
        });

        // Update memory row
        let memory_text = if model.runtime.memory_required_mb > 0 {
            let mem_gb = model.runtime.memory_required_mb as f64 / 1024.0;
            if let Some(peak) = model.runtime.memory_peak_mb {
                let peak_gb = peak as f64 / 1024.0;
                format!("{:.1} GB required ({:.1} GB peak)", mem_gb, peak_gb)
            } else {
                format!("{:.1} GB required", mem_gb)
            }
        } else {
            "Unknown".to_string()
        };
        let memory_row = self.view.view(ids!(memory_row));
        memory_row.label(ids!(info_value)).set_text(cx, &memory_text);
        memory_row.label(ids!(info_label)).apply_over(cx, live! {
            draw_text: { dark_mode: (dark_mode) }
        });
        memory_row.label(ids!(info_value)).apply_over(cx, live! {
            draw_text: { dark_mode: (dark_mode) }
        });

        // Update path row
        let path_row = self.view.view(ids!(path_row));
        path_row.label(ids!(info_value)).set_text(cx, &model.storage.local_path);
        path_row.label(ids!(info_label)).apply_over(cx, live! {
            draw_text: { dark_mode: (dark_mode) }
        });
        path_row.label(ids!(info_value)).apply_over(cx, live! {
            draw_text: { dark_mode: (dark_mode) }
        });

        // Update url row
        let url_row = self.view.view(ids!(url_row));
        url_row.label(ids!(info_value)).set_text(cx, &model.source.primary_url);
        url_row.label(ids!(info_label)).apply_over(cx, live! {
            draw_text: { dark_mode: (dark_mode) }
        });
        url_row.label(ids!(info_value)).apply_over(cx, live! {
            draw_text: { dark_mode: (dark_mode) }
        });

    }
}

/// Format bytes as human-readable string
fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
