//! Models Screen Widget Implementation

pub mod design;

use makepad_widgets::*;
use moly_data::{Store, Model, ModelFile, FileId, PendingDownload, PendingDownloadsStatus, ServerConnectionStatus};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

/// State of the models list
#[derive(Clone, Debug, Default)]
enum ModelsState {
    #[default]
    Idle,
    Loading,
    Loaded,
    Error(String),
}

/// Download state for a file
#[derive(Clone, Debug)]
#[allow(dead_code)]
struct DownloadState {
    file_id: FileId,
    model_name: String,
    file_name: String,
    progress: f64,
    status: PendingDownloadsStatus,
}

/// Result from async task
#[derive(Clone)]
enum ModelsTaskResult {
    ConnectionResult(Result<(), String>),
    ModelsResult(Result<Vec<Model>, String>),
    DownloadStarted(Result<FileId, String>),
    DownloadsUpdate(Result<Vec<PendingDownload>, String>),
}

/// Shared state for async results
type TaskResultState = Arc<Mutex<Option<ModelsTaskResult>>>;

#[derive(Live, LiveHook, Widget)]
pub struct ModelsApp {
    #[deref]
    pub view: View,

    /// Current state of the models list
    #[rust]
    models_state: ModelsState,

    /// Cached models for display
    #[rust]
    models: Vec<Model>,

    /// Current search query
    #[rust]
    search_query: String,

    /// Whether we're showing search results or featured models
    #[rust]
    is_search_results: bool,

    /// Shared state for async task results
    #[rust]
    task_result: TaskResultState,

    /// Whether we've initialized connection
    #[rust]
    initialized: bool,

    /// Active downloads (file_id -> download state)
    #[rust]
    active_downloads: HashMap<FileId, DownloadState>,

    /// Index of expanded model (for showing files) - reserved for future use
    #[rust]
    #[allow(dead_code)]
    expanded_model_index: Option<usize>,

    /// Timer for polling download progress
    #[rust]
    download_poll_timer: Timer,
}

impl Widget for ModelsApp {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // Initialize task result state
        if Arc::strong_count(&self.task_result) == 0 {
            self.task_result = Arc::new(Mutex::new(None));
        }

        // Initialize on first event
        if !self.initialized {
            self.initialized = true;
            self.test_connection_and_load(cx, scope);
        }

        // Handle timer for download polling
        if self.download_poll_timer.is_event(event).is_some() {
            if !self.active_downloads.is_empty() {
                self.poll_downloads(cx, scope);
            }
        }

        // Check for async task results
        self.check_task_results(cx, scope);

        // Handle events
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        // Handle refresh button click
        if self.view.button(ids!(refresh_btn)).clicked(&actions) {
            self.test_connection_and_load(cx, scope);
        }

        // Handle search input changes
        if let Some(text) = self.view.text_input(ids!(search_input)).changed(&actions) {
            self.handle_search(cx, scope, &text);
        }

        // Handle model card clicks (expand/collapse files)
        self.handle_model_card_clicks(cx, &actions);

        // Handle download button clicks
        self.handle_download_clicks(cx, scope, &actions);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Get dark mode value
        let dark_mode = if let Some(store) = scope.data.get::<Store>() {
            if store.is_dark_mode() { 1.0 } else { 0.0 }
        } else {
            0.0
        };

        // Apply dark mode to main view
        self.view.apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode) }
        });

        // Apply dark mode to header elements
        self.apply_dark_mode(cx, dark_mode);

        // Update connection status badge
        self.update_status_badge(cx, scope);

        // Update results label
        self.update_results_label(cx);

        // Show/hide downloads section
        let has_downloads = !self.active_downloads.is_empty();
        self.view.view(ids!(downloads_section)).set_visible(cx, has_downloads);
        if has_downloads {
            self.update_downloads_section(cx, dark_mode);
        }

        // Show/hide empty state vs model list
        let has_models = !self.models.is_empty();
        let is_loading = matches!(self.models_state, ModelsState::Loading);
        let is_error = matches!(self.models_state, ModelsState::Error(_));

        self.view.view(ids!(models_scroll)).set_visible(cx, has_models && !is_loading);
        self.view.view(ids!(empty_state)).set_visible(cx, !has_models || is_loading || is_error);

        // Update empty state message
        if !has_models || is_loading || is_error {
            let message = match &self.models_state {
                ModelsState::Loading => "Loading models...".to_string(),
                ModelsState::Error(e) => format!("Error: {}", e),
                ModelsState::Idle | ModelsState::Loaded => {
                    if self.is_search_results && self.models.is_empty() {
                        format!("No models found for '{}'", self.search_query)
                    } else {
                        "Start Moly Server to discover models".to_string()
                    }
                }
            };
            self.view.label(ids!(empty_label)).set_text(cx, &message);
            self.view.label(ids!(empty_label)).apply_over(cx, live! {
                draw_text: { dark_mode: (dark_mode) }
            });
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

impl ModelsApp {
    /// Test connection and load featured models
    fn test_connection_and_load(&mut self, cx: &mut Cx, scope: &mut Scope) {
        self.models_state = ModelsState::Loading;
        self.view.redraw(cx);

        // Get MolyClient from store
        let Some(store) = scope.data.get::<Store>() else { return };
        let moly_client = store.moly_client.clone();
        let task_result = self.task_result.clone();

        // Spawn async task to test connection and load models
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            rt.block_on(async {
                // First test connection
                if let Err(e) = moly_client.test_connection().await {
                    if let Ok(mut guard) = task_result.lock() {
                        *guard = Some(ModelsTaskResult::ConnectionResult(Err(e)));
                    }
                    return;
                }

                // Then load featured models
                let result = moly_client.get_featured_models().await;
                if let Ok(mut guard) = task_result.lock() {
                    *guard = Some(ModelsTaskResult::ModelsResult(result));
                }
            });
        });
    }

    /// Handle search input
    fn handle_search(&mut self, cx: &mut Cx, scope: &mut Scope, query: &str) {
        self.search_query = query.to_string();

        if query.trim().is_empty() {
            // If search is cleared, load featured models
            self.is_search_results = false;
            self.test_connection_and_load(cx, scope);
            return;
        }

        self.is_search_results = true;
        self.models_state = ModelsState::Loading;
        self.view.redraw(cx);

        // Get MolyClient from store
        let Some(store) = scope.data.get::<Store>() else { return };
        let moly_client = store.moly_client.clone();
        let task_result = self.task_result.clone();
        let search_query = query.to_string();

        // Spawn async task to search
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            rt.block_on(async {
                let result = moly_client.search_models(&search_query).await;
                if let Ok(mut guard) = task_result.lock() {
                    *guard = Some(ModelsTaskResult::ModelsResult(result));
                }
            });
        });
    }

    /// Check for async task results
    fn check_task_results(&mut self, cx: &mut Cx, _scope: &mut Scope) {
        let result = {
            if let Ok(mut guard) = self.task_result.lock() {
                guard.take()
            } else {
                None
            }
        };

        if let Some(task_result) = result {
            match task_result {
                ModelsTaskResult::ConnectionResult(Err(e)) => {
                    self.models_state = ModelsState::Error(e);
                    self.models.clear();
                }
                ModelsTaskResult::ConnectionResult(Ok(())) => {
                    // Connection successful, will be followed by ModelsResult
                }
                ModelsTaskResult::ModelsResult(Ok(models)) => {
                    ::log::info!("Loaded {} models", models.len());
                    self.models = models;
                    self.models_state = ModelsState::Loaded;
                }
                ModelsTaskResult::ModelsResult(Err(e)) => {
                    self.models_state = ModelsState::Error(e);
                    self.models.clear();
                }
                ModelsTaskResult::DownloadStarted(Ok(file_id)) => {
                    ::log::info!("Download started for file: {}", file_id);
                    // Start polling for updates
                    self.download_poll_timer = cx.start_interval(0.5);
                }
                ModelsTaskResult::DownloadStarted(Err(e)) => {
                    ::log::error!("Failed to start download: {}", e);
                }
                ModelsTaskResult::DownloadsUpdate(Ok(downloads)) => {
                    self.update_downloads_state(downloads);
                }
                ModelsTaskResult::DownloadsUpdate(Err(e)) => {
                    ::log::error!("Failed to get downloads: {}", e);
                }
            }
            self.view.redraw(cx);
        }
    }

    /// Update download state from pending downloads
    fn update_downloads_state(&mut self, downloads: Vec<PendingDownload>) {
        // Update or add downloads
        for download in &downloads {
            let file_id = download.file.id.clone();
            if let Some(state) = self.active_downloads.get_mut(&file_id) {
                state.progress = download.progress;
                state.status = download.status.clone();
            } else {
                self.active_downloads.insert(file_id.clone(), DownloadState {
                    file_id,
                    model_name: download.model.name.clone(),
                    file_name: download.file.name.clone(),
                    progress: download.progress,
                    status: download.status.clone(),
                });
            }
        }

        // Remove completed downloads
        let active_ids: Vec<_> = downloads.iter().map(|d| d.file.id.clone()).collect();
        self.active_downloads.retain(|id, _| active_ids.contains(id));

        // Stop polling if no more downloads
        if self.active_downloads.is_empty() {
            self.download_poll_timer = Timer::default();
        }
    }

    /// Apply dark mode to UI elements
    fn apply_dark_mode(&mut self, cx: &mut Cx2d, dark_mode: f64) {
        // Header
        self.view.label(ids!(title_label)).apply_over(cx, live! {
            draw_text: { dark_mode: (dark_mode) }
        });

        // Search input
        self.view.text_input(ids!(search_input)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode) }
            draw_text: { dark_mode: (dark_mode) }
        });

        // Refresh button
        self.view.button(ids!(refresh_btn)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode) }
            draw_text: { dark_mode: (dark_mode) }
        });

        // Results label
        self.view.label(ids!(results_label)).apply_over(cx, live! {
            draw_text: { dark_mode: (dark_mode) }
        });
    }

    /// Update connection status badge
    fn update_status_badge(&mut self, cx: &mut Cx2d, scope: &mut Scope) {
        let (status_val, status_text) = if let Some(store) = scope.data.get::<Store>() {
            match store.moly_client.connection_status() {
                ServerConnectionStatus::Disconnected => (0.0, "Disconnected"),
                ServerConnectionStatus::Connecting => (1.0, "Connecting..."),
                ServerConnectionStatus::Connected => (2.0, "Connected"),
                ServerConnectionStatus::Error(_) => (3.0, "Error"),
            }
        } else {
            (0.0, "Disconnected")
        };

        self.view.view(ids!(status_badge)).apply_over(cx, live! {
            draw_bg: { status: (status_val) }
        });
        self.view.label(ids!(status_text)).set_text(cx, status_text);
    }

    /// Update results label
    fn update_results_label(&mut self, cx: &mut Cx2d) {
        let label = if self.is_search_results {
            format!("{} results for '{}'", self.models.len(), self.search_query)
        } else {
            format!("Featured Models ({})", self.models.len())
        };
        self.view.label(ids!(results_label)).set_text(cx, &label);
    }

    /// Update downloads section with active download progress
    fn update_downloads_section(&mut self, cx: &mut Cx2d, dark_mode: f64) {
        // Update header
        let download_count = self.active_downloads.len();
        let header_text = if download_count == 1 {
            "Downloading 1 file...".to_string()
        } else {
            format!("Downloading {} files...", download_count)
        };
        self.view.label(ids!(downloads_header)).set_text(cx, &header_text);
        self.view.label(ids!(downloads_header)).apply_over(cx, live! {
            draw_text: { dark_mode: (dark_mode) }
        });

        // For simplicity, we just update labels with download info
        // A more sophisticated implementation would dynamically create DownloadItem widgets
        // For now, show summary of first download in the existing section
        if let Some((_, state)) = self.active_downloads.iter().next() {
            let status_text = match state.status {
                PendingDownloadsStatus::Initializing => "Initializing...".to_string(),
                PendingDownloadsStatus::Downloading => format!("{}% - {}", (state.progress * 100.0) as u32, state.file_name),
                PendingDownloadsStatus::Paused => format!("Paused - {}", state.file_name),
                PendingDownloadsStatus::Error => format!("Error - {}", state.file_name),
            };
            // Update header with more detail
            self.view.label(ids!(downloads_header)).set_text(cx, &status_text);
        }
    }

    /// Draw the models PortalList
    fn draw_models_list(&mut self, cx: &mut Cx2d, scope: &mut Scope, widget: WidgetRef, dark_mode: f64) {
        let binding = widget.as_portal_list();
        let Some(mut list) = binding.borrow_mut() else { return };

        list.set_item_range(cx, 0, self.models.len());

        while let Some(item_id) = list.next_visible_item(cx) {
            if item_id >= self.models.len() {
                continue;
            }

            let model = &self.models[item_id];
            let item_widget = list.item(cx, item_id, live_id!(ModelCardItem));

            // Apply dark mode to card
            item_widget.apply_over(cx, live! {
                draw_bg: { dark_mode: (dark_mode) }
            });

            // Set model name
            item_widget.label(ids!(model_name)).set_text(cx, &model.name);
            item_widget.label(ids!(model_name)).apply_over(cx, live! {
                draw_text: { dark_mode: (dark_mode) }
            });

            // Set model size
            item_widget.label(ids!(model_size)).set_text(cx, &model.size);
            item_widget.label(ids!(model_size)).apply_over(cx, live! {
                draw_text: { dark_mode: (dark_mode) }
            });

            // Set download count
            let download_text = format!("{} downloads", format_count(model.download_count));
            item_widget.label(ids!(download_count)).set_text(cx, &download_text);
            item_widget.label(ids!(download_count)).apply_over(cx, live! {
                draw_text: { dark_mode: (dark_mode) }
            });

            // Set like count
            let like_text = format!("{} likes", format_count(model.like_count));
            item_widget.label(ids!(like_count)).set_text(cx, &like_text);
            item_widget.label(ids!(like_count)).apply_over(cx, live! {
                draw_text: { dark_mode: (dark_mode) }
            });

            // Set summary (truncate if too long)
            let summary = if model.summary.len() > 200 {
                format!("{}...", &model.summary[..197])
            } else {
                model.summary.clone()
            };
            item_widget.label(ids!(model_summary)).set_text(cx, &summary);
            item_widget.label(ids!(model_summary)).apply_over(cx, live! {
                draw_text: { dark_mode: (dark_mode) }
            });

            // Set architecture
            item_widget.label(ids!(architecture)).set_text(cx, &model.architecture);
            item_widget.label(ids!(architecture)).apply_over(cx, live! {
                draw_text: { dark_mode: (dark_mode) }
            });

            // Set author
            item_widget.label(ids!(author)).set_text(cx, &model.author.name);
            item_widget.label(ids!(author)).apply_over(cx, live! {
                draw_text: { dark_mode: (dark_mode) }
            });

            // Show files count and download button for first file
            let has_files = !model.files.is_empty();
            item_widget.view(ids!(files_section)).set_visible(cx, has_files);

            if has_files {
                // Show files count
                let files_text = format!("{} file(s) available", model.files.len());
                item_widget.label(ids!(files_label)).set_text(cx, &files_text);
                item_widget.label(ids!(files_label)).apply_over(cx, live! {
                    draw_text: { dark_mode: (dark_mode) }
                });

                // Check if first file is being downloaded
                let first_file = &model.files[0];
                let is_downloading = self.active_downloads.contains_key(&first_file.id);

                if is_downloading {
                    if let Some(download_state) = self.active_downloads.get(&first_file.id) {
                        let progress_text = format!("{}%", (download_state.progress * 100.0) as u32);
                        item_widget.button(ids!(download_btn)).set_text(cx, &progress_text);
                    }
                } else if first_file.downloaded {
                    item_widget.button(ids!(download_btn)).set_text(cx, "Downloaded");
                } else {
                    item_widget.button(ids!(download_btn)).set_text(cx, "Download");
                }
            }

            item_widget.draw_all(cx, scope);
        }
    }

    /// Handle model card clicks for expanding files section
    fn handle_model_card_clicks(&mut self, cx: &mut Cx, actions: &Actions) {
        // For now, model cards are always expanded to show files
        // Could implement expand/collapse behavior here if needed
        let _ = (cx, actions);
    }

    /// Handle download button clicks
    fn handle_download_clicks(&mut self, cx: &mut Cx, scope: &mut Scope, actions: &Actions) {
        let models_list = self.view.portal_list(ids!(models_list));

        for (item_id, item_widget) in models_list.items_with_actions(actions) {
            if item_widget.button(ids!(download_btn)).clicked(actions) {
                if item_id < self.models.len() {
                    let model = &self.models[item_id];
                    if !model.files.is_empty() {
                        let file = &model.files[0];
                        if !file.downloaded && !self.active_downloads.contains_key(&file.id) {
                            self.start_download(cx, scope, file.clone(), model.name.clone());
                        }
                    }
                }
            }
        }
    }

    /// Start downloading a file
    fn start_download(&mut self, cx: &mut Cx, scope: &mut Scope, file: ModelFile, model_name: String) {
        let Some(store) = scope.data.get::<Store>() else { return };
        let moly_client = store.moly_client.clone();
        let task_result = self.task_result.clone();
        let file_id = file.id.clone();

        // Add to active downloads immediately with initializing status
        self.active_downloads.insert(file_id.clone(), DownloadState {
            file_id: file_id.clone(),
            model_name,
            file_name: file.name.clone(),
            progress: 0.0,
            status: PendingDownloadsStatus::Initializing,
        });

        self.view.redraw(cx);

        // Spawn async task to start download
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            rt.block_on(async {
                let result = moly_client.download_file(&file_id).await;
                if let Ok(mut guard) = task_result.lock() {
                    *guard = Some(ModelsTaskResult::DownloadStarted(
                        result.map(|_| file_id).map_err(|e| e.to_string())
                    ));
                }
            });
        });
    }

    /// Poll for download progress updates
    fn poll_downloads(&mut self, _cx: &mut Cx, scope: &mut Scope) {
        let Some(store) = scope.data.get::<Store>() else { return };
        let moly_client = store.moly_client.clone();
        let task_result = self.task_result.clone();

        // Only poll if we don't have a pending result
        if let Ok(guard) = task_result.lock() {
            if guard.is_some() {
                return;
            }
        }

        // Spawn async task to get downloads
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            rt.block_on(async {
                let result = moly_client.get_pending_downloads().await;
                if let Ok(mut guard) = task_result.lock() {
                    *guard = Some(ModelsTaskResult::DownloadsUpdate(
                        result.map_err(|e| e.to_string())
                    ));
                }
            });
        });
    }
}

/// Format large numbers with K/M suffix
fn format_count(count: u32) -> String {
    if count >= 1_000_000 {
        format!("{:.1}M", count as f64 / 1_000_000.0)
    } else if count >= 1_000 {
        format!("{:.1}K", count as f64 / 1_000.0)
    } else {
        count.to_string()
    }
}
