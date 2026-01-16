//! Settings Screen UI Design

use makepad_widgets::*;

use super::SettingsApp;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use moly_widgets::theme::*;

    // Provider icons - registered for dynamic loading
    ICON_OPENAI = dep("crate://self/resources/providers/openai.png")
    ICON_ANTHROPIC = dep("crate://self/resources/providers/anthropic.png")
    ICON_GEMINI = dep("crate://self/resources/providers/gemini.png")
    ICON_OLLAMA = dep("crate://self/resources/providers/ollama.png")
    ICON_DEEPSEEK = dep("crate://self/resources/providers/deepseek.png")

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

        provider_icon = <Image> {
            width: 24, height: 24
            fit: Smallest
        }

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

        // Provider icons for dynamic loading (order: openai, anthropic, gemini, ollama, deepseek)
        provider_icons: [
            (ICON_OPENAI),
            (ICON_ANTHROPIC),
            (ICON_GEMINI),
            (ICON_OLLAMA),
            (ICON_DEEPSEEK),
        ]

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
                    provider_icon = { source: (ICON_OPENAI) }
                    provider_name = { text: "OpenAI" }
                }
                anthropic_item = <ProviderItem> {
                    provider_icon = { source: (ICON_ANTHROPIC) }
                    provider_name = { text: "Anthropic" }
                }
                gemini_item = <ProviderItem> {
                    provider_icon = { source: (ICON_GEMINI) }
                    provider_name = { text: "Google Gemini" }
                }
                ollama_item = <ProviderItem> {
                    provider_icon = { source: (ICON_OLLAMA) }
                    provider_name = { text: "Ollama (Local)" }
                }
                groq_item = <ProviderItem> {
                    provider_name = { text: "Groq" }
                }
                deepseek_item = <ProviderItem> {
                    provider_icon = { source: (ICON_DEEPSEEK) }
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
                    spacing: 12

                    provider_title_icon = <Image> {
                        width: 32, height: 32
                        fit: Smallest
                        source: (ICON_OPENAI)
                    }

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
