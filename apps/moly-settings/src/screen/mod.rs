//! Settings Screen Widget Implementation

pub mod design;

use makepad_widgets::*;
use moly_data::{Store, ProviderId};

#[derive(Live, LiveHook, Widget)]
pub struct SettingsApp {
    #[deref]
    pub view: View,

    /// Provider icons loaded from live_design
    #[live]
    provider_icons: Vec<LiveDependency>,

    #[rust]
    selected_provider_id: Option<ProviderId>,
}

impl Widget for SettingsApp {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // Initialize with first provider selected (before handling events)
        if self.selected_provider_id.is_none() {
            self.selected_provider_id = Some("openai".to_string());
            self.load_provider_data(cx, scope);
            self.view.redraw(cx);
        }

        // Handle provider item clicks
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        // Provider selection
        if self.view.view(ids!(openai_item)).finger_down(&actions).is_some() {
            self.select_provider(cx, scope, "openai");
        }
        if self.view.view(ids!(anthropic_item)).finger_down(&actions).is_some() {
            self.select_provider(cx, scope, "anthropic");
        }
        if self.view.view(ids!(gemini_item)).finger_down(&actions).is_some() {
            self.select_provider(cx, scope, "gemini");
        }
        if self.view.view(ids!(ollama_item)).finger_down(&actions).is_some() {
            self.select_provider(cx, scope, "ollama");
        }
        if self.view.view(ids!(groq_item)).finger_down(&actions).is_some() {
            self.select_provider(cx, scope, "groq");
        }
        if self.view.view(ids!(deepseek_item)).finger_down(&actions).is_some() {
            self.select_provider(cx, scope, "deepseek");
        }

        // Save button click
        if self.view.button(ids!(save_button)).clicked(&actions) {
            self.save_provider(cx, scope);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Apply dark mode
        if let Some(store) = scope.data.get::<Store>() {
            let dark_mode_value = if store.is_dark_mode() { 1.0 } else { 0.0 };
            self.apply_dark_mode(cx, dark_mode_value);
        }

        // Update selection highlighting
        self.update_selection(cx);

        self.view.draw_walk(cx, scope, walk)
    }
}

impl SettingsApp {
    /// Get provider icon from the loaded LiveDependency list
    fn get_provider_icon(&self, provider_id: &str) -> Option<&LiveDependency> {
        // Icons are stored in order: openai, anthropic, gemini, ollama, deepseek
        let index = match provider_id {
            "openai" => Some(0),
            "anthropic" => Some(1),
            "gemini" => Some(2),
            "ollama" => Some(3),
            "deepseek" => Some(4),
            _ => None,
        };
        index.and_then(|i| self.provider_icons.get(i))
    }

    fn select_provider(&mut self, cx: &mut Cx, scope: &mut Scope, id: &str) {
        self.selected_provider_id = Some(id.to_string());
        self.load_provider_data(cx, scope);
        self.view.redraw(cx);
    }

    fn load_provider_data(&mut self, cx: &mut Cx, scope: &mut Scope) {
        let Some(provider_id) = self.selected_provider_id.clone() else { return };

        if let Some(store) = scope.data.get::<Store>() {
            if let Some(provider) = store.preferences.get_provider(&provider_id) {
                ::log::info!("Loading provider data for {}: url={}, has_key={}, enabled={}",
                    provider_id, provider.url, provider.api_key.is_some(), provider.enabled);

                // Update title
                self.view.label(ids!(provider_title)).set_text(cx, &provider.name);

                // Update provider title icon using LiveDependency from live_design
                if let Some(icon_dep) = self.get_provider_icon(&provider_id) {
                    let icon_path = icon_dep.as_str();
                    let _ = self.view.image(ids!(provider_title_icon)).load_image_dep_by_path(cx, icon_path);
                }

                // Update URL input
                self.view.text_input(ids!(api_host_input)).set_text(cx, &provider.url);

                // Update API key input - show masked if exists
                let key_text = provider.api_key.clone().unwrap_or_default();
                ::log::info!("Setting API key input: len={}", key_text.len());
                self.view.text_input(ids!(api_key_input)).set_text(cx, &key_text);

                // Update enabled checkbox
                self.view.check_box(ids!(enabled_checkbox)).set_active(cx, provider.enabled);

                // Clear status message
                self.view.label(ids!(status_message)).set_text(cx, "");
            } else {
                ::log::warn!("Provider {} not found in preferences", provider_id);
            }
        } else {
            ::log::warn!("Store not available in scope");
        }
    }

    fn save_provider(&mut self, cx: &mut Cx, scope: &mut Scope) {
        let Some(provider_id) = &self.selected_provider_id else { return };

        // Get values from inputs
        let url = self.view.text_input(ids!(api_host_input)).text();
        let api_key_text = self.view.text_input(ids!(api_key_input)).text();
        let enabled = self.view.check_box(ids!(enabled_checkbox)).active(cx);

        ::log::info!("save_provider: provider={}, url={}, api_key_len={}, enabled={}",
            provider_id, url, api_key_text.len(), enabled);

        // Save to Store
        if let Some(store) = scope.data.get_mut::<Store>() {
            store.preferences.set_provider_url(provider_id, url);
            store.preferences.set_provider_enabled(provider_id, enabled);

            // Only update API key if user entered something, or if explicitly clearing
            // This prevents accidentally clearing the key if text input returns empty
            if !api_key_text.is_empty() {
                ::log::info!("save_provider: saving API key (len={})", api_key_text.len());
                store.preferences.set_provider_api_key(provider_id, Some(api_key_text));
            } else {
                // Check if there was already a key - if so, don't clear it
                let existing_key = store.preferences.get_provider(provider_id)
                    .and_then(|p| p.api_key.clone());
                if existing_key.is_some() {
                    ::log::warn!("save_provider: text input empty but existing key found, NOT clearing");
                } else {
                    ::log::info!("save_provider: no API key to save");
                }
            }

            // Show success message
            self.view.label(ids!(status_message)).set_text(cx, "Settings saved!");

            ::log::info!("Saved provider settings for {}", provider_id);
        }

        self.view.redraw(cx);
    }

    fn update_selection(&mut self, cx: &mut Cx2d) {
        let selected = self.selected_provider_id.as_deref().unwrap_or("");

        // Reset all items
        let items = ["openai_item", "anthropic_item", "gemini_item", "ollama_item", "groq_item", "deepseek_item"];
        let ids = ["openai", "anthropic", "gemini", "ollama", "groq", "deepseek"];

        for (item, id) in items.iter().zip(ids.iter()) {
            let selected_val = if *id == selected { 1.0 } else { 0.0 };

            match *item {
                "openai_item" => {
                    self.view.view(ids!(openai_item)).apply_over(cx, live!{
                        draw_bg: { selected: (selected_val) }
                    });
                }
                "anthropic_item" => {
                    self.view.view(ids!(anthropic_item)).apply_over(cx, live!{
                        draw_bg: { selected: (selected_val) }
                    });
                }
                "gemini_item" => {
                    self.view.view(ids!(gemini_item)).apply_over(cx, live!{
                        draw_bg: { selected: (selected_val) }
                    });
                }
                "ollama_item" => {
                    self.view.view(ids!(ollama_item)).apply_over(cx, live!{
                        draw_bg: { selected: (selected_val) }
                    });
                }
                "groq_item" => {
                    self.view.view(ids!(groq_item)).apply_over(cx, live!{
                        draw_bg: { selected: (selected_val) }
                    });
                }
                "deepseek_item" => {
                    self.view.view(ids!(deepseek_item)).apply_over(cx, live!{
                        draw_bg: { selected: (selected_val) }
                    });
                }
                _ => {}
            }
        }
    }

    fn apply_dark_mode(&mut self, cx: &mut Cx2d, dark_mode: f64) {
        self.view.apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode) }
        });

        // Apply to panels
        self.view.view(ids!(providers_panel)).apply_over(cx, live!{
            draw_bg: { dark_mode: (dark_mode) }
        });

        // Apply to all labels and inputs that have dark_mode
        self.view.label(ids!(header_label)).apply_over(cx, live!{
            draw_text: { dark_mode: (dark_mode) }
        });
        self.view.label(ids!(provider_title)).apply_over(cx, live!{
            draw_text: { dark_mode: (dark_mode) }
        });
        self.view.label(ids!(provider_type_label)).apply_over(cx, live!{
            draw_text: { dark_mode: (dark_mode) }
        });

        // Apply to provider items
        for id in ["openai_item", "anthropic_item", "gemini_item", "ollama_item", "groq_item", "deepseek_item"] {
            match id {
                "openai_item" => {
                    self.view.view(ids!(openai_item)).apply_over(cx, live!{
                        draw_bg: { dark_mode: (dark_mode) }
                        provider_name = { draw_text: { dark_mode: (dark_mode) } }
                    });
                }
                "anthropic_item" => {
                    self.view.view(ids!(anthropic_item)).apply_over(cx, live!{
                        draw_bg: { dark_mode: (dark_mode) }
                        provider_name = { draw_text: { dark_mode: (dark_mode) } }
                    });
                }
                "gemini_item" => {
                    self.view.view(ids!(gemini_item)).apply_over(cx, live!{
                        draw_bg: { dark_mode: (dark_mode) }
                        provider_name = { draw_text: { dark_mode: (dark_mode) } }
                    });
                }
                "ollama_item" => {
                    self.view.view(ids!(ollama_item)).apply_over(cx, live!{
                        draw_bg: { dark_mode: (dark_mode) }
                        provider_name = { draw_text: { dark_mode: (dark_mode) } }
                    });
                }
                "groq_item" => {
                    self.view.view(ids!(groq_item)).apply_over(cx, live!{
                        draw_bg: { dark_mode: (dark_mode) }
                        provider_name = { draw_text: { dark_mode: (dark_mode) } }
                    });
                }
                "deepseek_item" => {
                    self.view.view(ids!(deepseek_item)).apply_over(cx, live!{
                        draw_bg: { dark_mode: (dark_mode) }
                        provider_name = { draw_text: { dark_mode: (dark_mode) } }
                    });
                }
                _ => {}
            }
        }

        // Apply to text inputs
        self.view.text_input(ids!(api_host_input)).apply_over(cx, live!{
            draw_bg: { dark_mode: (dark_mode) }
            draw_text: { dark_mode: (dark_mode) }
        });
        self.view.text_input(ids!(api_key_input)).apply_over(cx, live!{
            draw_bg: { dark_mode: (dark_mode) }
            draw_text: { dark_mode: (dark_mode) }
        });

        // Apply to checkbox
        self.view.check_box(ids!(enabled_checkbox)).apply_over(cx, live!{
            draw_text: { dark_mode: (dark_mode) }
        });
    }
}
