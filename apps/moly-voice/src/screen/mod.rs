pub mod design;

use makepad_widgets::*;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use crate::screen::design::*;
}

// ── Data types ────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct VoiceEntry {
    pub name: String,
    pub is_ready: bool,
}

#[derive(Default)]
enum TrainingState {
    #[default]
    Idle,
    Training {
        task_id: String,
        stage: String,
        progress: f32,
    },
    Done,
    Error(String),
}

#[derive(Default)]
enum SynthesisState {
    #[default]
    Idle,
    Generating,
    Done { duration_secs: f32 },
    Error(String),
}

enum TrainingUpdate {
    Progress { stage: String, progress: f32 },
    Done,
    Error(String),
}

enum SynthesisUpdate {
    Done { duration_secs: f32 },
    Error(String),
}

enum VoicesUpdate {
    Loaded(Vec<VoiceEntry>),
    Error(String),
}

// ── Widget ────────────────────────────────────────────────────────────────────

#[derive(Live, LiveHook, Widget)]
pub struct VoiceApp {
    #[deref]
    pub view: View,

    #[rust]
    initialized: bool,

    #[rust]
    voices: Vec<VoiceEntry>,

    #[rust]
    selected_voice_index: Option<usize>,

    #[rust]
    training_state: TrainingState,

    #[rust]
    synthesis_state: SynthesisState,

    // Form config
    #[rust]
    quality: String, // "fast" | "standard" | "high"

    #[rust]
    language: String, // "auto" | "zh" | "en"

    #[rust]
    denoise: bool,

    // Background thread channels
    #[rust]
    training_rx: Option<Receiver<TrainingUpdate>>,

    #[rust]
    synthesis_rx: Option<Receiver<SynthesisUpdate>>,

    #[rust]
    voices_rx: Option<Receiver<VoicesUpdate>>,

    // Cancel flag shared with the training polling thread
    #[rust]
    training_cancel: Option<Arc<AtomicBool>>,

    // Task id of the running training job (used for cancel API call)
    #[rust]
    training_task_id: String,
}

impl Widget for VoiceApp {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // ── One-time initialisation ───────────────────────────────────────
        if !self.initialized {
            self.quality = "standard".to_string();
            self.language = "auto".to_string();
            self.denoise = true;
            self.initialized = true;
            self.fetch_voices();
        }

        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        // ── Left panel ────────────────────────────────────────────────────

        // "+ New" clears the form and focuses the voice name input
        if self.view.button(ids!(new_voice_btn)).clicked(&actions) {
            self.view.text_input(ids!(voice_name_input)).set_text(cx, "");
            self.view.text_input(ids!(audio_path_input)).set_text(cx, "");
            self.view.text_input(ids!(transcript_input)).set_text(cx, "");
            self.selected_voice_index = None;
            self.view.text_input(ids!(voice_name_input)).set_key_focus(cx);
            self.view.redraw(cx);
        }

        // Voice list item clicks (PortalList)
        let voices_list = self.view.portal_list(ids!(voices_list));
        for (item_id, item) in voices_list.items_with_actions(&actions) {
            if let Some(fd) = item.as_view().finger_down(&actions) {
                if fd.tap_count == 1 {
                    self.selected_voice_index = Some(item_id);
                    self.view.redraw(cx);
                }
            }
        }

        // ── Quality buttons ───────────────────────────────────────────────
        if self.view.button(ids!(quality_fast_btn)).clicked(&actions) {
            self.quality = "fast".to_string();
            self.view.redraw(cx);
        }
        if self.view.button(ids!(quality_standard_btn)).clicked(&actions) {
            self.quality = "standard".to_string();
            self.view.redraw(cx);
        }
        if self.view.button(ids!(quality_high_btn)).clicked(&actions) {
            self.quality = "high".to_string();
            self.view.redraw(cx);
        }

        // ── Language buttons ──────────────────────────────────────────────
        if self.view.button(ids!(lang_auto_btn)).clicked(&actions) {
            self.language = "auto".to_string();
            self.view.redraw(cx);
        }
        if self.view.button(ids!(lang_zh_btn)).clicked(&actions) {
            self.language = "zh".to_string();
            self.view.redraw(cx);
        }
        if self.view.button(ids!(lang_en_btn)).clicked(&actions) {
            self.language = "en".to_string();
            self.view.redraw(cx);
        }

        // ── Denoise toggle ────────────────────────────────────────────────
        if self.view.button(ids!(denoise_btn)).clicked(&actions) {
            self.denoise = !self.denoise;
            self.view.redraw(cx);
        }

        // ── Train button ──────────────────────────────────────────────────
        if self.view.button(ids!(train_btn)).clicked(&actions) {
            let voice_name = self.view.text_input(ids!(voice_name_input)).text();
            let audio_path = self.view.text_input(ids!(audio_path_input)).text();
            let transcript = self.view.text_input(ids!(transcript_input)).text();

            if voice_name.trim().is_empty() {
                self.show_train_status(cx, "Please enter a voice name.", true);
            } else if audio_path.trim().is_empty() {
                self.show_train_status(cx, "Please enter the path to a WAV file.", true);
            } else {
                self.start_training(cx, voice_name, audio_path, transcript);
            }
        }

        // ── Cancel training button ────────────────────────────────────────
        if self.view.button(ids!(cancel_train_btn)).clicked(&actions) {
            if let Some(cancel) = &self.training_cancel {
                cancel.store(true, Ordering::SeqCst);
            }
            // POST cancel to API in a fire-and-forget thread
            let task_id = self.training_task_id.clone();
            std::thread::spawn(move || {
                let _ = reqwest::blocking::Client::new()
                    .post("http://localhost:8080/v1/voices/train/cancel")
                    .json(&serde_json::json!({ "task_id": task_id }))
                    .send();
            });
            self.training_state = TrainingState::Idle;
            self.training_rx = None;
            self.view.redraw(cx);
        }

        // ── Generate button ───────────────────────────────────────────────
        if self.view.button(ids!(generate_btn)).clicked(&actions) {
            let synth_text = self.view.text_input(ids!(synth_text_input)).text();
            let speed_str = self.view.text_input(ids!(speed_input)).text();
            let speed: f32 = speed_str.parse().unwrap_or(1.0);

            if synth_text.trim().is_empty() {
                self.show_synth_status(cx, "Please enter text to synthesize.");
            } else if let Some(idx) = self.selected_voice_index {
                if idx < self.voices.len() {
                    let voice_name = self.voices[idx].name.clone();
                    self.start_synthesis(cx, synth_text, voice_name, speed);
                } else {
                    self.show_synth_status(cx, "Selected voice is no longer available.");
                }
            } else {
                self.show_synth_status(cx, "Please select a voice from the left panel.");
            }
        }

        // ── Play button ───────────────────────────────────────────────────
        if self.view.button(ids!(play_btn)).clicked(&actions) {
            std::process::Command::new("afplay")
                .arg("/tmp/ominix-voice-out.wav")
                .spawn()
                .ok();
        }

        // ── Poll background channels ──────────────────────────────────────
        let mut need_next_frame = false;

        // Voices fetch
        if let Some(rx) = &self.voices_rx {
            if let Ok(update) = rx.try_recv() {
                match update {
                    VoicesUpdate::Loaded(voices) => {
                        self.voices = voices;
                        ::log::info!("Voice list refreshed: {} voices", self.voices.len());
                    }
                    VoicesUpdate::Error(e) => {
                        ::log::warn!("Failed to fetch voices: {}", e);
                    }
                }
                self.voices_rx = None;
                self.view.redraw(cx);
            }
        }

        // Training updates
        if let Some(rx) = &self.training_rx {
            match rx.try_recv() {
                Ok(TrainingUpdate::Progress { stage, progress }) => {
                    self.training_state = TrainingState::Training {
                        task_id: self.training_task_id.clone(),
                        stage,
                        progress,
                    };
                    need_next_frame = true;
                    self.view.redraw(cx);
                }
                Ok(TrainingUpdate::Done) => {
                    self.training_state = TrainingState::Done;
                    self.training_rx = None;
                    self.training_cancel = None;
                    self.show_train_status(cx, "Training complete! Voice is ready.", false);
                    self.fetch_voices(); // Refresh voice list
                    self.view.redraw(cx);
                }
                Ok(TrainingUpdate::Error(e)) => {
                    let msg = format!("Training failed: {}", e);
                    self.training_state = TrainingState::Error(e);
                    self.training_rx = None;
                    self.training_cancel = None;
                    self.show_train_status(cx, &msg, true);
                    self.view.redraw(cx);
                }
                Err(mpsc::TryRecvError::Empty) => {
                    need_next_frame = true;
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.training_rx = None;
                }
            }
        }

        // Synthesis updates
        if let Some(rx) = &self.synthesis_rx {
            match rx.try_recv() {
                Ok(SynthesisUpdate::Done { duration_secs }) => {
                    self.synthesis_state = SynthesisState::Done { duration_secs };
                    self.synthesis_rx = None;
                    let msg = format!("Ready — {:.1}s generated", duration_secs);
                    self.show_synth_status(cx, &msg);
                    self.view.redraw(cx);
                }
                Ok(SynthesisUpdate::Error(e)) => {
                    let msg = format!("Synthesis failed: {}", e);
                    self.synthesis_state = SynthesisState::Error(e);
                    self.synthesis_rx = None;
                    self.show_synth_status(cx, &msg);
                    self.view.redraw(cx);
                }
                Err(mpsc::TryRecvError::Empty) => {
                    need_next_frame = true;
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.synthesis_rx = None;
                }
            }
        }

        if need_next_frame {
            cx.new_next_frame();
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Update dynamic UI before drawing
        self.update_button_states(cx);
        self.update_training_ui(cx);
        self.update_synthesis_ui(cx);
        self.update_synth_voice_label(cx);

        // Get PortalList UID for step pattern
        let voices_list = self.view.portal_list(ids!(voices_list));
        let voices_list_uid = voices_list.widget_uid();

        while let Some(widget) = self.view.draw_walk(cx, scope, walk).step() {
            if widget.widget_uid() == voices_list_uid {
                self.draw_voices_list(cx, scope, widget);
            }
        }

        DrawStep::done()
    }
}

impl VoiceApp {
    // ── Helpers for updating UI ───────────────────────────────────────────────

    fn update_button_states(&mut self, cx: &mut Cx2d) {
        // Quality buttons
        let q = if self.quality.is_empty() { "standard" } else { &self.quality };
        let q_fast = if q == "fast" { 1.0_f64 } else { 0.0_f64 };
        let q_std  = if q == "standard" { 1.0_f64 } else { 0.0_f64 };
        let q_high = if q == "high" { 1.0_f64 } else { 0.0_f64 };
        self.view.button(ids!(quality_fast_btn)).apply_over(cx, live! { draw_bg: { selected: (q_fast) } });
        self.view.button(ids!(quality_standard_btn)).apply_over(cx, live! { draw_bg: { selected: (q_std) } });
        self.view.button(ids!(quality_high_btn)).apply_over(cx, live! { draw_bg: { selected: (q_high) } });

        // Language buttons
        let l = if self.language.is_empty() { "auto" } else { &self.language };
        let l_auto = if l == "auto" { 1.0_f64 } else { 0.0_f64 };
        let l_zh   = if l == "zh"   { 1.0_f64 } else { 0.0_f64 };
        let l_en   = if l == "en"   { 1.0_f64 } else { 0.0_f64 };
        self.view.button(ids!(lang_auto_btn)).apply_over(cx, live! { draw_bg: { selected: (l_auto) } });
        self.view.button(ids!(lang_zh_btn)).apply_over(cx, live! { draw_bg: { selected: (l_zh) } });
        self.view.button(ids!(lang_en_btn)).apply_over(cx, live! { draw_bg: { selected: (l_en) } });

        // Denoise button
        let denoise_active = if self.denoise || !self.initialized { 1.0_f64 } else { 0.0_f64 };
        let denoise_text = if self.denoise || !self.initialized { "✓ Denoise" } else { "Denoise" };
        self.view.button(ids!(denoise_btn)).apply_over(cx, live! {
            draw_bg: { active: (denoise_active) }
        });
        self.view.button(ids!(denoise_btn)).set_text(cx, denoise_text);
    }

    fn update_training_ui(&mut self, cx: &mut Cx2d) {
        let is_training = matches!(self.training_state, TrainingState::Training { .. });

        self.view.button(ids!(train_btn)).apply_over(cx, live! { visible: (!is_training) });
        self.view.button(ids!(cancel_train_btn)).apply_over(cx, live! { visible: (is_training) });
        self.view.view(ids!(progress_section)).apply_over(cx, live! { visible: (is_training) });

        if let TrainingState::Training { stage, progress, .. } = &self.training_state {
            let pct = (progress * 100.0) as u32;
            self.view.label(ids!(progress_stage_label)).set_text(cx, stage);
            self.view.label(ids!(progress_pct_label)).set_text(cx, &format!("{}%", pct));

            // Update progress bar fill width proportionally
            // We use apply_over with a pixel width; the bar is inside progress_bar_bg
            // Approximate bar fill: assume right panel ~900px wide minus padding ~852px
            let approx_bar_width = 852.0_f64;
            let fill_w = (*progress as f64 * approx_bar_width) as i64;
            self.view.view(ids!(progress_fill)).apply_over(cx, live! {
                width: (fill_w)
            });
        }
    }

    fn update_synthesis_ui(&mut self, cx: &mut Cx2d) {
        let is_generating = matches!(self.synthesis_state, SynthesisState::Generating);
        let has_output = matches!(self.synthesis_state, SynthesisState::Done { .. });

        self.view.button(ids!(generate_btn)).apply_over(cx, live! { visible: (!is_generating) });
        self.view.button(ids!(play_btn)).apply_over(cx, live! { visible: (has_output) });
    }

    fn update_synth_voice_label(&mut self, cx: &mut Cx2d) {
        let text = if let Some(idx) = self.selected_voice_index {
            self.voices.get(idx).map(|v| v.name.as_str()).unwrap_or("(none)").to_string()
        } else {
            "(select a voice from the list)".to_string()
        };
        self.view.label(ids!(synth_voice_label)).set_text(cx, &text);
    }

    fn show_train_status(&mut self, cx: &mut Cx, msg: &str, is_error: bool) {
        let err_val: f64 = if is_error { 1.0 } else { 0.0 };
        self.view.label(ids!(train_status_label)).set_text(cx, msg);
        self.view.label(ids!(train_status_label)).apply_over(cx, live! {
            visible: (true)
        });
        self.view.label(ids!(train_status_label)).apply_over(cx, live! {
            draw_text: { is_error: (err_val) }
        });
    }

    fn show_synth_status(&mut self, cx: &mut Cx, msg: &str) {
        self.view.label(ids!(synth_status_label)).set_text(cx, msg);
        self.view.redraw(cx);
    }

    // ── Voice list drawing ────────────────────────────────────────────────────

    fn draw_voices_list(&self, cx: &mut Cx2d, scope: &mut Scope, widget: WidgetRef) {
        let binding = widget.as_portal_list();
        let Some(mut list) = binding.borrow_mut() else { return };

        if self.voices.is_empty() {
            list.set_item_range(cx, 0, 1);
            if let Some(0) = list.next_visible_item(cx) {
                let item = list.item(cx, 0, live_id!(VoiceEmptyItem));
                item.draw_all(cx, scope);
            }
            return;
        }

        list.set_item_range(cx, 0, self.voices.len());
        while let Some(item_id) = list.next_visible_item(cx) {
            if item_id >= self.voices.len() { continue; }
            let voice = &self.voices[item_id];
            let is_selected = self.selected_voice_index == Some(item_id);

            let item = list.item(cx, item_id, live_id!(VoiceListItem));
            item.apply_over(cx, live! {
                draw_bg: { selected: (if is_selected { 1.0 } else { 0.0 }) }
            });
            let ready = if voice.is_ready { 1.0_f64 } else { 0.0_f64 };
            item.view(ids!(voice_status)).apply_over(cx, live! {
                draw_bg: { ready: (ready) }
            });
            item.label(ids!(voice_name)).set_text(cx, &voice.name);
            item.draw_all(cx, scope);
        }
    }

    // ── Background operations ─────────────────────────────────────────────────

    /// Fetch the voice list from the API in a background thread.
    fn fetch_voices(&mut self) {
        let (tx, rx): (Sender<VoicesUpdate>, Receiver<VoicesUpdate>) = mpsc::channel();
        self.voices_rx = Some(rx);

        std::thread::spawn(move || {
            match Self::fetch_voices_blocking() {
                Ok(voices) => { let _ = tx.send(VoicesUpdate::Loaded(voices)); }
                Err(e)     => { let _ = tx.send(VoicesUpdate::Error(e)); }
            }
        });
    }

    fn fetch_voices_blocking() -> Result<Vec<VoiceEntry>, String> {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .map_err(|e| e.to_string())?;

        let resp = client
            .get("http://localhost:8080/v1/voices")
            .send()
            .map_err(|e| format!("GET /v1/voices failed: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("GET /v1/voices HTTP {}", resp.status()));
        }

        let value: serde_json::Value = resp.json().map_err(|e| e.to_string())?;

        // Handle both {"data": [...]} and {"voices": [...]} and plain [...]
        let names: Vec<String> = if let Some(arr) = value.get("data").and_then(|d| d.as_array()) {
            arr.iter()
                .filter_map(|v| {
                    v.get("voice_id").or_else(|| v.get("name"))
                        .and_then(|n| n.as_str())
                        .map(String::from)
                })
                .collect()
        } else if let Some(arr) = value.get("voices").and_then(|d| d.as_array()) {
            arr.iter().filter_map(|v| v.as_str().map(String::from)).collect()
        } else if let Some(arr) = value.as_array() {
            arr.iter().filter_map(|v| v.as_str().map(String::from)).collect()
        } else {
            vec![]
        };

        Ok(names.into_iter().map(|name| VoiceEntry { name, is_ready: true }).collect())
    }

    /// Start training a new voice.
    fn start_training(
        &mut self,
        cx: &mut Cx,
        voice_name: String,
        audio_path: String,
        transcript: String,
    ) {
        let quality  = if self.quality.is_empty() { "standard".to_string() } else { self.quality.clone() };
        let language = if self.language.is_empty() { "auto".to_string() } else { self.language.clone() };
        let denoise  = self.denoise;

        let cancel = Arc::new(AtomicBool::new(false));
        self.training_cancel = Some(cancel.clone());

        let (tx, rx): (Sender<TrainingUpdate>, Receiver<TrainingUpdate>) = mpsc::channel();
        self.training_rx = Some(rx);
        self.training_state = TrainingState::Training {
            task_id: String::new(),
            stage: "Starting…".to_string(),
            progress: 0.0,
        };

        // Add training-voice entry to the list immediately (is_ready=false)
        let voice_entry_name = voice_name.clone();
        if !self.voices.iter().any(|v| v.name == voice_entry_name) {
            self.voices.push(VoiceEntry { name: voice_entry_name, is_ready: false });
        }

        self.view.label(ids!(train_status_label)).apply_over(cx, live! { visible: (false) });
        cx.new_next_frame();

        std::thread::spawn(move || {
            if let Err(e) = Self::run_training_thread(
                tx.clone(), cancel, voice_name, audio_path, transcript,
                quality, language, denoise,
            ) {
                let _ = tx.send(TrainingUpdate::Error(e));
            }
        });
    }

    fn run_training_thread(
        tx: Sender<TrainingUpdate>,
        cancel: Arc<AtomicBool>,
        voice_name: String,
        audio_path: String,
        transcript: String,
        quality: String,
        language: String,
        denoise: bool,
    ) -> Result<(), String> {
        use base64::Engine as _;

        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(600))
            .build()
            .map_err(|e| e.to_string())?;

        // 1. Read and base64-encode the audio file
        let audio_bytes = std::fs::read(&audio_path)
            .map_err(|e| format!("Cannot read audio file '{}': {}", audio_path, e))?;
        let audio_b64 = base64::engine::general_purpose::STANDARD.encode(&audio_bytes);

        // 2. POST /v1/voices/train
        let train_body = serde_json::json!({
            "voice_name": voice_name,
            "audio": audio_b64,
            "transcript": transcript,
            "quality": quality,
            "language": language,
            "denoise": denoise,
        });

        let resp = client
            .post("http://localhost:8080/v1/voices/train")
            .json(&train_body)
            .send()
            .map_err(|e| format!("POST /v1/voices/train failed: {}", e))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().unwrap_or_default();
            return Err(format!("Training request failed (HTTP {}): {}", status, body));
        }

        let resp_val: serde_json::Value = resp.json().map_err(|e| e.to_string())?;
        let task_id = resp_val
            .get("task_id")
            .and_then(|v| v.as_str())
            .ok_or("No task_id in training response")?
            .to_string();

        let _ = tx.send(TrainingUpdate::Progress {
            stage: "Uploading…".to_string(),
            progress: 0.05,
        });

        // 3. Poll for status
        loop {
            if cancel.load(Ordering::SeqCst) {
                return Ok(());
            }

            std::thread::sleep(std::time::Duration::from_millis(500));

            if cancel.load(Ordering::SeqCst) {
                return Ok(());
            }

            let status_url = format!(
                "http://localhost:8080/v1/voices/train/status?task_id={}",
                task_id
            );
            let status_resp = client
                .get(&status_url)
                .send()
                .map_err(|e| format!("Status poll failed: {}", e))?;

            if !status_resp.status().is_success() {
                return Err(format!("Status poll HTTP {}", status_resp.status()));
            }

            let sv: serde_json::Value = status_resp.json().map_err(|e| e.to_string())?;
            let stage    = sv.get("stage").and_then(|v| v.as_str()).unwrap_or("Processing").to_string();
            let progress = sv.get("progress").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
            let done     = sv.get("done").and_then(|v| v.as_bool()).unwrap_or(false)
                        || progress >= 1.0;
            let error    = sv.get("error").and_then(|v| v.as_str()).map(String::from);

            if let Some(err) = error {
                let _ = tx.send(TrainingUpdate::Error(err));
                return Ok(());
            }

            let _ = tx.send(TrainingUpdate::Progress { stage, progress });

            if done {
                let _ = tx.send(TrainingUpdate::Done);
                return Ok(());
            }
        }
    }

    /// Start speech synthesis in a background thread.
    fn start_synthesis(&mut self, cx: &mut Cx, text: String, voice: String, speed: f32) {
        let (tx, rx): (Sender<SynthesisUpdate>, Receiver<SynthesisUpdate>) = mpsc::channel();
        self.synthesis_rx = Some(rx);
        self.synthesis_state = SynthesisState::Generating;
        self.show_synth_status(cx, "Generating…");
        cx.new_next_frame();

        std::thread::spawn(move || {
            match Self::run_synthesis_thread(text, voice, speed) {
                Ok(duration) => { let _ = tx.send(SynthesisUpdate::Done { duration_secs: duration }); }
                Err(e)       => { let _ = tx.send(SynthesisUpdate::Error(e)); }
            }
        });
    }

    fn run_synthesis_thread(text: String, voice: String, speed: f32) -> Result<f32, String> {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .map_err(|e| e.to_string())?;

        let body = serde_json::json!({
            "input": text,
            "voice": voice,
            "speed": speed,
            "response_format": "wav",
        });

        let resp = client
            .post("http://localhost:8080/v1/audio/speech")
            .json(&body)
            .send()
            .map_err(|e| format!("POST /v1/audio/speech failed: {}", e))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body_txt = resp.text().unwrap_or_default();
            return Err(format!("Synthesis failed (HTTP {}): {}", status, body_txt));
        }

        let wav_bytes = resp.bytes().map_err(|e| e.to_string())?;
        let byte_count = wav_bytes.len();

        std::fs::write("/tmp/ominix-voice-out.wav", &wav_bytes)
            .map_err(|e| format!("Failed to write WAV: {}", e))?;

        // Approximate duration: WAV 44100 Hz, 16-bit mono = 88200 bytes/sec
        let duration_secs = byte_count.saturating_sub(44) as f32 / 88200.0;
        Ok(duration_secs)
    }
}
