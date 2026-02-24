pub mod design;

use makepad_widgets::*;
use moly_data::{ModelRegistry, RegistryModel, RegistryCategory, SourceKind};
use serde::Deserialize;
use std::sync::{Arc, atomic::{AtomicBool, AtomicU64, Ordering}};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::io::{Read, Write};

// ─── List row ────────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
enum ListRow {
    Header(RegistryCategory),
    /// Global index into `registry.models`
    Model(usize),
}

// ─── Filter ───────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Default, Debug)]
enum Filter {
    #[default]
    All,
    Cat(RegistryCategory),
}

// ─── UI state per model ───────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Debug)]
enum ModelUiState {
    NotDownloaded,
    Downloading,
    Downloaded,
    Error,
}

impl ModelUiState {
    fn dot_value(self) -> f64 {
        match self {
            Self::NotDownloaded => 0.0,
            Self::Downloading   => 1.0,
            Self::Downloaded    => 2.0,
            Self::Error         => 4.0,
        }
    }
    fn label(self) -> &'static str {
        match self {
            Self::NotDownloaded => "Not Downloaded",
            Self::Downloading   => "Downloading...",
            Self::Downloaded    => "Downloaded",
            Self::Error         => "Error",
        }
    }
}

// ─── Download state ───────────────────────────────────────────────────────────

#[derive(Clone)]
struct ModelDownloadState {
    is_downloading:   Arc<AtomicBool>,
    cancel_requested: Arc<AtomicBool>,
    progress_bytes:   Arc<AtomicU64>,
    total_bytes:      Arc<AtomicU64>,
    current_file:     Arc<std::sync::Mutex<String>>,
    completed:        Arc<AtomicBool>,
    failed:           Arc<AtomicBool>,
    error_msg:        Arc<std::sync::Mutex<String>>,
}

impl ModelDownloadState {
    fn new() -> Self {
        Self {
            is_downloading:   Arc::new(AtomicBool::new(false)),
            cancel_requested: Arc::new(AtomicBool::new(false)),
            progress_bytes:   Arc::new(AtomicU64::new(0)),
            total_bytes:      Arc::new(AtomicU64::new(0)),
            current_file:     Arc::new(std::sync::Mutex::new(String::new())),
            completed:        Arc::new(AtomicBool::new(false)),
            failed:           Arc::new(AtomicBool::new(false)),
            error_msg:        Arc::new(std::sync::Mutex::new(String::new())),
        }
    }

    fn reset(&self) {
        self.is_downloading.store(false, Ordering::SeqCst);
        self.cancel_requested.store(false, Ordering::SeqCst);
        self.progress_bytes.store(0, Ordering::SeqCst);
        self.total_bytes.store(0, Ordering::SeqCst);
        self.completed.store(false, Ordering::SeqCst);
        self.failed.store(false, Ordering::SeqCst);
        *self.current_file.lock().unwrap() = String::new();
        *self.error_msg.lock().unwrap() = String::new();
    }

    fn fraction(&self) -> f64 {
        let total = self.total_bytes.load(Ordering::SeqCst);
        if total == 0 { return 0.0; }
        (self.progress_bytes.load(Ordering::SeqCst) as f64 / total as f64).min(1.0)
    }

    fn progress_text(&self) -> String {
        let done  = self.progress_bytes.load(Ordering::SeqCst);
        let total = self.total_bytes.load(Ordering::SeqCst);
        let file  = self.current_file.lock().unwrap().clone();
        let pct   = self.fraction() * 100.0;
        if file.is_empty() {
            format!("{:.1}%  ({}/{} MB)", pct, done / 1_048_576, total / 1_048_576)
        } else {
            format!("{:.1}%  {}", pct, file)
        }
    }
}

// ─── HuggingFace / ModelScope API structs ────────────────────────────────────

#[derive(Deserialize)]
struct HfItem {
    #[serde(rename = "type")]
    item_type: String,
    path: String,
    size: Option<u64>,
}

#[derive(Deserialize)]
struct MsResponse {
    #[serde(rename = "Code")]
    code: i32,
    #[serde(rename = "Data")]
    data: Option<MsData>,
}

#[derive(Deserialize)]
struct MsData {
    #[serde(rename = "Files")]
    files: Vec<MsFile>,
}

#[derive(Deserialize)]
struct MsFile {
    #[serde(rename = "Path")]
    path: String,
    #[serde(rename = "Size")]
    size: u64,
    #[serde(rename = "Type")]
    file_type: String,
}

// ─── Widget ───────────────────────────────────────────────────────────────────

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use crate::screen::design::*;
}

#[derive(Live, LiveHook, Widget)]
pub struct ModelHubApp {
    #[deref]
    view: View,

    #[rust] registry: Option<ModelRegistry>,
    #[rust] initialized: bool,
    #[rust] filter: Filter,
    #[rust] search_query: String,
    #[rust] selected_id: Option<String>,
    #[rust] model_states: HashMap<String, ModelUiState>,
    #[rust] download_states: HashMap<String, ModelDownloadState>,
    #[rust] flat_list: Vec<ListRow>,
}

impl Widget for ModelHubApp {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.initialized { self.initialize(cx); }

        let actions = cx.capture_actions(|cx| self.view.handle_event(cx, event, scope));

        self.handle_filter_clicks(cx, &actions);
        self.handle_search(&actions, cx);
        self.handle_list_clicks(cx, &actions);
        self.handle_detail_buttons(cx, &actions);
        self.poll_downloads(cx);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.initialized { self.initialized = true; }

        let hub_list = self.view.portal_list(ids!(hub_model_list));
        let hub_list_uid = hub_list.widget_uid();

        while let Some(widget) = self.view.draw_walk(cx, scope, walk).step() {
            if widget.widget_uid() == hub_list_uid {
                self.draw_hub_list(cx, scope, widget);
            }
        }

        DrawStep::done()
    }
}

impl ModelHubApp {
    fn draw_hub_list(&mut self, cx: &mut Cx2d, scope: &mut Scope, widget: WidgetRef) {
        let binding = widget.as_portal_list();
        let Some(mut list) = binding.borrow_mut() else { return };

        list.set_item_range(cx, 0, self.flat_list.len());

        while let Some(item_id) = list.next_visible_item(cx) {
            match self.flat_list.get(item_id).copied() {
                Some(ListRow::Header(cat)) => {
                    let item = list.item(cx, item_id, live_id!(HubCategoryHeader));
                    item.label(ids!(category_header_label)).set_text(cx, cat.label());
                    item.draw_all(cx, scope);
                }
                Some(ListRow::Model(global_idx)) => {
                    let name = self.registry.as_ref()
                        .and_then(|r| r.models.get(global_idx))
                        .map(|m| m.name.as_str())
                        .unwrap_or("");
                    let model_id = self.registry.as_ref()
                        .and_then(|r| r.models.get(global_idx))
                        .map(|m| m.id.as_str())
                        .unwrap_or("");
                    let state = self.model_states.get(model_id).copied()
                        .unwrap_or(ModelUiState::NotDownloaded);
                    let selected = self.selected_id.as_deref() == Some(model_id);
                    let dl_frac = self.download_states.get(model_id).map(|d| d.fraction());

                    let item = list.item(cx, item_id, live_id!(HubModelItem));
                    item.label(ids!(model_name)).set_text(cx, name);
                    item.view(ids!(model_status)).apply_over(cx, live! {
                        draw_bg: { status: (state.dot_value()) }
                    });
                    item.apply_over(cx, live! {
                        draw_bg: { selected: (if selected { 1.0_f64 } else { 0.0_f64 }) }
                    });

                    if let Some(pct) = dl_frac {
                        item.view(ids!(inline_progress)).set_visible(cx, true);
                        item.view(ids!(inline_progress)).apply_over(cx, live! {
                            draw_bg: { progress: (pct) }
                        });
                    } else {
                        item.view(ids!(inline_progress)).set_visible(cx, false);
                    }

                    item.draw_all(cx, scope);
                }
                None => {}
            }
        }
    }

    // ── Init ─────────────────────────────────────────────────────────────────

    fn initialize(&mut self, cx: &mut Cx) {
        self.initialized = true;
        let registry = ModelRegistry::load();
        ModelRegistry::fetch_updates_async();
        for model in &registry.models {
            self.model_states.insert(model.id.clone(), scan_state(model));
        }
        self.registry = Some(registry);
        self.rebuild_list();
        self.view.redraw(cx);
    }

    // ── List building ─────────────────────────────────────────────────────────

    fn rebuild_list(&mut self) {
        let Some(registry) = &self.registry else { return };
        let q = self.search_query.to_lowercase();

        const CATS: [RegistryCategory; 5] = [
            RegistryCategory::Llm,
            RegistryCategory::Vlm,
            RegistryCategory::Asr,
            RegistryCategory::Tts,
            RegistryCategory::ImageGen,
        ];

        self.flat_list.clear();

        for cat in CATS {
            if let Filter::Cat(fc) = self.filter {
                if fc != cat { continue; }
            }
            let models_in_cat: Vec<usize> = registry.models.iter().enumerate()
                .filter(|(_, m)| m.category == cat)
                .filter(|(_, m)| {
                    q.is_empty()
                        || m.name.to_lowercase().contains(&q)
                        || m.description.to_lowercase().contains(&q)
                        || m.tags.iter().any(|t| t.to_lowercase().contains(&q))
                })
                .map(|(i, _)| i)
                .collect();

            if models_in_cat.is_empty() { continue; }

            self.flat_list.push(ListRow::Header(cat));
            for gi in models_in_cat {
                self.flat_list.push(ListRow::Model(gi));
            }
        }
    }

    // ── Event handlers ────────────────────────────────────────────────────────

    fn handle_filter_clicks(&mut self, cx: &mut Cx, actions: &Actions) {
        let mut new_filter = None;
        if self.view.button(ids!(filter_all)).clicked(actions) {
            new_filter = Some(Filter::All);
        } else if self.view.button(ids!(filter_llm)).clicked(actions) {
            new_filter = Some(Filter::Cat(RegistryCategory::Llm));
        } else if self.view.button(ids!(filter_vlm)).clicked(actions) {
            new_filter = Some(Filter::Cat(RegistryCategory::Vlm));
        } else if self.view.button(ids!(filter_asr)).clicked(actions) {
            new_filter = Some(Filter::Cat(RegistryCategory::Asr));
        } else if self.view.button(ids!(filter_tts)).clicked(actions) {
            new_filter = Some(Filter::Cat(RegistryCategory::Tts));
        } else if self.view.button(ids!(filter_image)).clicked(actions) {
            new_filter = Some(Filter::Cat(RegistryCategory::ImageGen));
        }

        if let Some(f) = new_filter {
            self.filter = f;
            self.rebuild_list();
            let sel = |b: bool| if b { 1.0_f64 } else { 0.0_f64 };
            let is_all   = f == Filter::All;
            let is_llm   = f == Filter::Cat(RegistryCategory::Llm);
            let is_vlm   = f == Filter::Cat(RegistryCategory::Vlm);
            let is_asr   = f == Filter::Cat(RegistryCategory::Asr);
            let is_tts   = f == Filter::Cat(RegistryCategory::Tts);
            let is_image = f == Filter::Cat(RegistryCategory::ImageGen);
            self.view.button(ids!(filter_all)).apply_over(cx, live! {   draw_bg: { selected: (sel(is_all))   } });
            self.view.button(ids!(filter_llm)).apply_over(cx, live! {   draw_bg: { selected: (sel(is_llm))   } });
            self.view.button(ids!(filter_vlm)).apply_over(cx, live! {   draw_bg: { selected: (sel(is_vlm))   } });
            self.view.button(ids!(filter_asr)).apply_over(cx, live! {   draw_bg: { selected: (sel(is_asr))   } });
            self.view.button(ids!(filter_tts)).apply_over(cx, live! {   draw_bg: { selected: (sel(is_tts))   } });
            self.view.button(ids!(filter_image)).apply_over(cx, live! { draw_bg: { selected: (sel(is_image)) } });
            self.view.redraw(cx);
        }
    }

    fn handle_search(&mut self, actions: &Actions, cx: &mut Cx) {
        if let Some(txt) = self.view.text_input(ids!(search_input)).changed(actions) {
            self.search_query = txt.to_string();
            self.rebuild_list();
            self.view.redraw(cx);
        }
    }

    fn handle_list_clicks(&mut self, cx: &mut Cx, actions: &Actions) {
        let list = self.view.portal_list(ids!(hub_model_list));
        for (item_id, item) in list.items_with_actions(actions) {
            if let Some(ListRow::Model(global_idx)) = self.flat_list.get(item_id).copied() {
                if let Some(fd) = item.as_view().finger_down(actions) {
                    if fd.tap_count == 1 {
                        if let Some(model) = self.registry.as_ref()
                            .and_then(|r| r.models.get(global_idx))
                        {
                            let model_id = model.id.clone();
                            self.selected_id = Some(model_id.clone());
                            self.refresh_detail(cx, &model_id);
                            self.view.redraw(cx);
                        }
                    }
                }
            }
        }
    }

    fn handle_detail_buttons(&mut self, cx: &mut Cx, actions: &Actions) {
        let sel = match self.selected_id.clone() { Some(s) => s, None => return };

        if self.view.button(ids!(hub_download_btn)).clicked(actions) {
            self.start_download(cx, &sel);
        }
        if self.view.button(ids!(hub_cancel_btn)).clicked(actions) {
            if let Some(ds) = self.download_states.get(&sel) {
                ds.cancel_requested.store(true, Ordering::SeqCst);
                self.view.label(ids!(hub_status_msg)).set_text(cx, "Cancelling...");
                self.view.redraw(cx);
            }
        }
        if self.view.button(ids!(hub_remove_btn)).clicked(actions) {
            if let Some(model) = self.registry.as_ref()
                .and_then(|r| r.models.iter().find(|m| m.id == sel))
            {
                let path = expand_tilde(&model.storage.local_path);
                if std::fs::remove_dir_all(&path).is_ok() {
                    self.model_states.insert(sel.clone(), ModelUiState::NotDownloaded);
                    self.refresh_detail(cx, &sel);
                    self.view.redraw(cx);
                    ::log::info!("Removed model {}", sel);
                }
            }
        }
    }

    // ── Detail panel ──────────────────────────────────────────────────────────

    fn refresh_detail(&mut self, cx: &mut Cx, model_id: &str) {
        let Some(model) = self.registry.as_ref()
            .and_then(|r| r.models.iter().find(|m| m.id == model_id))
            .cloned()
        else { return };

        let state = self.model_states.get(model_id).copied()
            .unwrap_or(ModelUiState::NotDownloaded);

        self.view.widget(ids!(hub_empty_state)).set_visible(cx, false);
        self.view.widget(ids!(model_details)).set_visible(cx, true);

        self.view.label(ids!(hub_model_name)).set_text(cx, &model.name);
        self.view.label(ids!(hub_model_desc)).set_text(cx, &model.description);
        self.view.label(ids!(hub_model_tags)).set_text(cx, &model.tags.join(" · "));

        self.view.label(ids!(status_value)).set_text(cx, state.label());
        self.view.label(ids!(category_value)).set_text(cx, model.category.label());
        self.view.label(ids!(size_value)).set_text(cx, &model.storage.size_display);
        self.view.label(ids!(memory_value))
            .set_text(cx, &format!("{:.1} GB", model.runtime.memory_gb));
        self.view.label(ids!(path_value)).set_text(cx, &model.storage.local_path);
        self.view.label(ids!(api_value))
            .set_text(cx, &format!("{:?}", model.runtime.api_type));

        let is_dl     = state == ModelUiState::Downloading;
        let is_done   = state == ModelUiState::Downloaded;
        let is_manual = model.source.kind == SourceKind::Manual;

        self.view.widget(ids!(hub_download_btn)).set_visible(cx, !is_dl && !is_done && !is_manual);
        self.view.widget(ids!(hub_cancel_btn)).set_visible(cx, is_dl);
        self.view.widget(ids!(hub_remove_btn)).set_visible(cx, is_done);
        self.view.widget(ids!(hub_progress_section)).set_visible(cx, is_dl);

        let msg = if is_manual {
            format!("Manual installation required.\nPlace model files in:\n{}", model.storage.local_path)
        } else {
            String::new()
        };
        self.view.label(ids!(hub_status_msg)).set_text(cx, &msg);

        if is_dl {
            if let Some(ds) = self.download_states.get(model_id) {
                let pct = ds.fraction();
                let txt = ds.progress_text();
                self.view.view(ids!(hub_progress_fill)).apply_over(cx, live! {
                    draw_bg: { progress: (pct) }
                });
                self.view.label(ids!(hub_progress_text)).set_text(cx, &txt);
            }
        }
    }

    // ── Download ──────────────────────────────────────────────────────────────

    fn start_download(&mut self, cx: &mut Cx, model_id: &str) {
        let Some(model) = self.registry.as_ref()
            .and_then(|r| r.models.iter().find(|m| m.id == model_id))
            .cloned()
        else { return };

        if model.source.kind == SourceKind::Manual {
            self.view.label(ids!(hub_status_msg))
                .set_text(cx, "Manual installation required — see path above.");
            return;
        }

        let ds = self.download_states
            .entry(model_id.to_string())
            .or_insert_with(ModelDownloadState::new)
            .clone();
        ds.reset();
        ds.is_downloading.store(true, Ordering::SeqCst);

        self.model_states.insert(model_id.to_string(), ModelUiState::Downloading);
        self.refresh_detail(cx, model_id);
        cx.new_next_frame();

        let model_id_owned = model_id.to_string();
        let local_path     = expand_tilde(&model.storage.local_path);
        let source_kind    = model.source.kind;
        let repo_id        = model.source.repo_id.clone().unwrap_or_default();
        let revision       = model.source.revision.clone();

        std::thread::spawn(move || {
            let client = match reqwest::blocking::Client::builder()
                .timeout(std::time::Duration::from_secs(3600))
                .build()
            {
                Ok(c) => c,
                Err(e) => {
                    *ds.error_msg.lock().unwrap() = e.to_string();
                    ds.failed.store(true, Ordering::SeqCst);
                    ds.is_downloading.store(false, Ordering::SeqCst);
                    return;
                }
            };

            let result = match source_kind {
                SourceKind::HuggingFace => {
                    download_hf(&client, &repo_id, &revision, &local_path, &ds)
                }
                SourceKind::ModelScope => {
                    download_ms(&client, &repo_id, &revision, &local_path, &ds)
                }
                _ => Err("Source not supported for automatic download".to_string()),
            };

            match result {
                Ok(_) => ds.completed.store(true, Ordering::SeqCst),
                Err(e) => {
                    *ds.error_msg.lock().unwrap() = e;
                    ds.failed.store(true, Ordering::SeqCst);
                }
            }
            ds.is_downloading.store(false, Ordering::SeqCst);
            ::log::info!("Download thread finished: {}", model_id_owned);
        });
    }

    fn poll_downloads(&mut self, cx: &mut Cx) {
        let mut keep_going = false;
        let mut complete: Vec<String> = Vec::new();
        let mut failed: Vec<(String, String)> = Vec::new();

        for (id, ds) in &self.download_states {
            if ds.is_downloading.load(Ordering::SeqCst) { keep_going = true; }
            if ds.completed.load(Ordering::SeqCst) {
                complete.push(id.clone());
            } else if ds.failed.load(Ordering::SeqCst) {
                failed.push((id.clone(), ds.error_msg.lock().unwrap().clone()));
            }
        }

        for id in complete {
            self.model_states.insert(id.clone(), ModelUiState::Downloaded);
            self.download_states.remove(&id);
            if self.selected_id.as_deref() == Some(id.as_str()) {
                self.refresh_detail(cx, &id);
            }
        }

        for (id, err) in failed {
            self.model_states.insert(id.clone(), ModelUiState::Error);
            let msg = format!("Download failed: {}", err);
            self.download_states.remove(&id);
            if self.selected_id.as_deref() == Some(id.as_str()) {
                self.view.label(ids!(hub_status_msg)).set_text(cx, &msg);
                self.refresh_detail(cx, &id);
            }
            ::log::error!("Download error for {}: {}", id, err);
        }

        // Live progress update for selected downloading model
        if let Some(sel) = self.selected_id.clone() {
            if let Some(ds) = self.download_states.get(sel.as_str()) {
                if ds.is_downloading.load(Ordering::SeqCst) {
                    let pct = ds.fraction();
                    let txt = ds.progress_text();
                    self.view.view(ids!(hub_progress_fill)).apply_over(cx, live! {
                        draw_bg: { progress: (pct) }
                    });
                    self.view.label(ids!(hub_progress_text)).set_text(cx, &txt);
                    self.view.redraw(cx);
                }
            }
        }

        if keep_going { cx.new_next_frame(); }
    }
}

// ─── Filesystem scan ──────────────────────────────────────────────────────────

fn scan_state(model: &RegistryModel) -> ModelUiState {
    let path_str = expand_tilde(&model.storage.local_path);
    let path = Path::new(&path_str);
    if !path.exists() { return ModelUiState::NotDownloaded; }
    let count = std::fs::read_dir(path)
        .map(|e| e.filter_map(|x| x.ok())
            .filter(|x| {
                let n = x.file_name();
                let s = n.to_string_lossy();
                !s.starts_with('.')
            }).count())
        .unwrap_or(0);
    if count > 0 { ModelUiState::Downloaded } else { ModelUiState::NotDownloaded }
}

// ─── HuggingFace download ─────────────────────────────────────────────────────

fn download_hf(
    client: &reqwest::blocking::Client,
    repo_id: &str,
    revision: &str,
    local_path: &str,
    ds: &ModelDownloadState,
) -> Result<(), String> {
    let url = format!("https://huggingface.co/api/models/{}/tree/{}", repo_id, revision);
    let mut req = client.get(&url);
    if let Some(tok) = hf_token() {
        req = req.header("Authorization", format!("Bearer {}", tok));
    }
    let resp = req.send().map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("HF API {}", resp.status()));
    }
    let items: Vec<HfItem> = resp.json().map_err(|e| e.to_string())?;
    let files: Vec<(String, u64)> = items.into_iter()
        .filter(|i| i.item_type == "blob")
        .map(|i| (i.path, i.size.unwrap_or(0)))
        .collect();
    if files.is_empty() { return Err("No files in repository".to_string()); }

    let total: u64 = files.iter().map(|(_, s)| s).sum();
    ds.total_bytes.store(total, Ordering::SeqCst);

    let mut done = 0u64;
    for (path, _) in &files {
        if ds.cancel_requested.load(Ordering::SeqCst) {
            return Err("Cancelled".to_string());
        }
        let file_url = format!(
            "https://huggingface.co/{}/resolve/{}/{}",
            repo_id, revision, path
        );
        let dest = PathBuf::from(local_path).join(path);
        *ds.current_file.lock().unwrap() = path.clone();

        let bytes = stream_download(client, &file_url, &dest, &ds.cancel_requested)?;
        done += bytes;
        ds.progress_bytes.store(done, Ordering::SeqCst);
    }
    Ok(())
}

// ─── ModelScope download ──────────────────────────────────────────────────────

fn download_ms(
    client: &reqwest::blocking::Client,
    repo_id: &str,
    revision: &str,
    local_path: &str,
    ds: &ModelDownloadState,
) -> Result<(), String> {
    let url = format!(
        "https://modelscope.cn/api/v1/models/{}/repo/files?Revision={}&Recursive=true",
        repo_id, revision
    );
    let resp = client.get(&url).send().map_err(|e| e.to_string())?;
    let ms: MsResponse = resp.json().map_err(|e| e.to_string())?;
    if ms.code != 200 { return Err(format!("ModelScope API code {}", ms.code)); }
    let data = ms.data.ok_or_else(|| "empty data".to_string())?;
    let files: Vec<(String, u64)> = data.files.into_iter()
        .filter(|f| f.file_type == "blob")
        .map(|f| (f.path, f.size))
        .collect();

    let total: u64 = files.iter().map(|(_, s)| s).sum();
    ds.total_bytes.store(total, Ordering::SeqCst);

    let mut done = 0u64;
    for (path, _) in &files {
        if ds.cancel_requested.load(Ordering::SeqCst) {
            return Err("Cancelled".to_string());
        }
        let file_url = format!(
            "https://modelscope.cn/api/v1/models/{}/repo?Revision={}&FilePath={}",
            repo_id, revision, path
        );
        let dest = PathBuf::from(local_path).join(path);
        *ds.current_file.lock().unwrap() = path.clone();

        let bytes = stream_download(client, &file_url, &dest, &ds.cancel_requested)?;
        done += bytes;
        ds.progress_bytes.store(done, Ordering::SeqCst);
    }
    Ok(())
}

// ─── Streaming file download ──────────────────────────────────────────────────

fn stream_download(
    client: &reqwest::blocking::Client,
    url: &str,
    dest: &Path,
    cancel: &Arc<AtomicBool>,
) -> Result<u64, String> {
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let mut req = client.get(url);
    if let Some(tok) = hf_token() {
        req = req.header("Authorization", format!("Bearer {}", tok));
    }
    let mut resp = req.send().map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("HTTP {}", resp.status()));
    }

    let mut file = std::fs::File::create(dest).map_err(|e| e.to_string())?;
    let mut buf = [0u8; 65536];
    let mut total = 0u64;

    loop {
        if cancel.load(Ordering::SeqCst) {
            drop(file);
            let _ = std::fs::remove_file(dest);
            return Err("Cancelled".to_string());
        }
        match resp.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                file.write_all(&buf[..n]).map_err(|e| e.to_string())?;
                total += n as u64;
            }
            Err(e) => return Err(e.to_string()),
        }
    }
    Ok(total)
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

fn hf_token() -> Option<String> {
    let home = dirs::home_dir()?;
    let p = home.join(".huggingface").join("hub").join("token");
    let t = std::fs::read_to_string(p).ok()?.trim().to_string();
    if t.is_empty() { None } else { Some(t) }
}
