use makepad_widgets::*;
use crate::data::{Store, ProviderId};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use moly_widgets::theme::*;

    // Settings label style
    SettingsLabel = <Label> {
        draw_text: {
            instance dark_mode: 0.0
            fn get_color(self) -> vec4 {
                return mix(#374151, #e2e8f0, self.dark_mode);
            }
            text_style: <THEME_FONT_BOLD>{ font_size: 11.0 }
        }
    }

    // Settings hint/helper text
    SettingsHint = <Label> {
        draw_text: {
            instance dark_mode: 0.0
            fn get_color(self) -> vec4 {
                return mix(#9ca3af, #64748b, self.dark_mode);
            }
            text_style: <THEME_FONT_REGULAR>{ font_size: 10.0 }
        }
    }

    // Text input for settings
    SettingsTextInput = <TextInput> {
        width: Fill, height: 44
        padding: {left: 12, right: 12, top: 10, bottom: 10}

        draw_bg: {
            instance radius: 6.0
            instance border_width: 1.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let sz = self.rect_size - 2.0;
                sdf.box(1.0, 1.0, sz.x, sz.y, max(1.0, self.radius - self.border_width));

                let bg = mix(#ffffff, #1e293b, self.dark_mode);
                let border = mix(#d1d5db, #475569, self.dark_mode);
                sdf.fill(bg);
                sdf.stroke(border, self.border_width);
                return sdf.result;
            }
        }

        draw_text: {
            instance dark_mode: 0.0
            fn get_color(self) -> vec4 {
                return mix(#1f2937, #f1f5f9, self.dark_mode);
            }
            text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
        }
    }

    // Provider list item
    ProviderItem = <View> {
        width: Fill, height: Fit
        padding: {left: 16, right: 16, top: 12, bottom: 12}
        cursor: Hand
        show_bg: true

        draw_bg: {
            instance hover: 0.0
            instance selected: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let base = mix(#ffffff, #1e293b, self.dark_mode);
                let hover_color = mix(#f1f5f9, #334155, self.dark_mode);
                let selected_color = mix(#dbeafe, #1e3a5f, self.dark_mode);
                return mix(mix(base, hover_color, self.hover), selected_color, self.selected);
            }
        }

        flow: Right
        align: {y: 0.5}
        spacing: 12

        provider_name = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                fn get_color(self) -> vec4 {
                    return mix(#1f2937, #f1f5f9, self.dark_mode);
                }
                text_style: <THEME_FONT_REGULAR>{ font_size: 13.0 }
            }
        }
    }

    // Save button
    SaveButton = <Button> {
        width: Fit, height: 40
        padding: {left: 20, right: 20, top: 10, bottom: 10}

        draw_bg: {
            instance hover: 0.0
            instance pressed: 0.0
            instance radius: 6.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let sz = self.rect_size - 2.0;
                // Blue button colors: #3b82f6 -> #2563eb -> #1d4ed8
                let base_color = vec4(0.231, 0.510, 0.965, 1.0);
                let hover_color = vec4(0.145, 0.388, 0.922, 1.0);
                let pressed_color = vec4(0.114, 0.306, 0.847, 1.0);
                let color = mix(
                    mix(base_color, hover_color, self.hover),
                    pressed_color,
                    self.pressed
                );
                sdf.box(1.0, 1.0, sz.x, sz.y, self.radius);
                sdf.fill(color);
                return sdf.result;
            }
        }

        draw_text: {
            color: #ffffff
            text_style: <THEME_FONT_BOLD>{ font_size: 12.0 }
        }

        text: "Save"
    }

    pub SettingsApp = {{SettingsApp}} {
        width: Fill, height: Fill
        flow: Right
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix(#f5f7fa, #0f172a, self.dark_mode);
            }
        }

        // Left panel - provider list
        providers_panel = <View> {
            width: 280, height: Fill
            flow: Down
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 {
                    return mix(#ffffff, #1e293b, self.dark_mode);
                }
            }

            // Header
            <View> {
                width: Fill, height: Fit
                padding: {left: 16, right: 16, top: 16, bottom: 12}

                header_label = <Label> {
                    text: "Providers"
                    draw_text: {
                        instance dark_mode: 0.0
                        fn get_color(self) -> vec4 {
                            return mix(#1f2937, #f1f5f9, self.dark_mode);
                        }
                        text_style: <THEME_FONT_BOLD>{ font_size: 20.0 }
                    }
                }
            }

            // Provider list
            providers_list = <View> {
                width: Fill, height: Fill
                flow: Down

                openai_item = <ProviderItem> {
                    provider_name = { text: "OpenAI" }
                }
                anthropic_item = <ProviderItem> {
                    provider_name = { text: "Anthropic" }
                }
                gemini_item = <ProviderItem> {
                    provider_name = { text: "Google Gemini" }
                }
                ollama_item = <ProviderItem> {
                    provider_name = { text: "Ollama (Local)" }
                }
                groq_item = <ProviderItem> {
                    provider_name = { text: "Groq" }
                }
                deepseek_item = <ProviderItem> {
                    provider_name = { text: "DeepSeek" }
                }
            }
        }

        // Divider
        <View> {
            width: 1, height: Fill
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 {
                    return mix(#e5e7eb, #374151, self.dark_mode);
                }
            }
        }

        // Right panel - provider details
        provider_view = <View> {
            width: Fill, height: Fill
            flow: Down
            padding: 24
            spacing: 20

            // Header with title and enabled checkbox on same row
            provider_header = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 4

                // Title row with checkbox on the right
                title_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {y: 0.5}

                    provider_title = <Label> {
                        text: "OpenAI"
                        draw_text: {
                            instance dark_mode: 0.0
                            fn get_color(self) -> vec4 {
                                return mix(#1f2937, #f1f5f9, self.dark_mode);
                            }
                            text_style: <THEME_FONT_BOLD>{ font_size: 20.0 }
                        }
                    }

                    <View> { width: Fill } // Spacer

                    enabled_checkbox = <CheckBox> {
                        text: "Enabled"
                        draw_check: {
                            instance dark_mode: 0.0
                            fn get_color(self) -> vec4 {
                                return mix(#3b82f6, #60a5fa, self.dark_mode);
                            }
                        }
                        draw_text: {
                            instance dark_mode: 0.0
                            fn get_color(self) -> vec4 {
                                return mix(#374151, #e2e8f0, self.dark_mode);
                            }
                            text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                        }
                    }
                }

                provider_type_label = <Label> {
                    text: "OpenAI Compatible API"
                    draw_text: {
                        instance dark_mode: 0.0
                        fn get_color(self) -> vec4 {
                            return mix(#6b7280, #94a3b8, self.dark_mode);
                        }
                        text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                    }
                }
            }

            // API Host section
            host_section = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 6

                <SettingsLabel> { text: "API Host" }
                api_host_input = <SettingsTextInput> {
                    text: "https://api.openai.com/v1"
                }
                <SettingsHint> { text: "The base URL for API requests" }
            }

            // API Key section
            key_section = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 6

                <SettingsLabel> { text: "API Key" }
                api_key_input = <SettingsTextInput> {
                    is_password: true
                    empty_text: "sk-..."
                }
                <SettingsHint> { text: "Your API key (stored locally)" }
            }

            // Actions
            actions = <View> {
                width: Fill, height: Fit
                flow: Right
                spacing: 12
                margin: {top: 12}

                save_button = <SaveButton> {}
            }

            // Status message
            status_message = <Label> {
                text: ""
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#059669, #10b981, self.dark_mode);
                    }
                    text_style: <THEME_FONT_REGULAR>{ font_size: 11.0 }
                }
            }

            // Spacer
            <View> { width: Fill, height: Fill }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct SettingsApp {
    #[deref]
    pub view: View,

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
    fn select_provider(&mut self, cx: &mut Cx, scope: &mut Scope, id: &str) {
        self.selected_provider_id = Some(id.to_string());
        self.load_provider_data(cx, scope);
        self.view.redraw(cx);
    }

    fn load_provider_data(&mut self, cx: &mut Cx, scope: &mut Scope) {
        let Some(provider_id) = &self.selected_provider_id else { return };

        if let Some(store) = scope.data.get::<Store>() {
            if let Some(provider) = store.preferences.get_provider(provider_id) {
                ::log::info!("Loading provider data for {}: url={}, has_key={}, enabled={}",
                    provider_id, provider.url, provider.api_key.is_some(), provider.enabled);

                // Update title
                self.view.label(ids!(provider_title)).set_text(cx, &provider.name);

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
            draw_check: { dark_mode: (dark_mode) }
            draw_text: { dark_mode: (dark_mode) }
        });
    }
}
