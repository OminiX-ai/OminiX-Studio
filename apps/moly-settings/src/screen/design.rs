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
    ICON_NVIDIA = dep("crate://self/resources/providers/nvidia.png")
    ICON_GROQ = dep("crate://self/resources/providers/groq.png")

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

    // Custom toggle switch - bigger and green when enabled
    EnableToggle = <CheckBoxFlat> {
        width: 50, height: 26
        margin: 0
        padding: 0

        draw_bg: {
            uniform size: 28.0
            uniform border_size: 0.0
            uniform border_radius: 14.0

            // Off state colors (gray)
            uniform color: #9ca3af
            uniform color_hover: #9ca3af
            uniform color_down: #9ca3af
            // On state colors (green)
            uniform color_active: #22c55e
            uniform color_focus: #22c55e
            uniform color_disabled: #d1d5db

            // No border
            uniform border_color: #00000000
            uniform border_color_hover: #00000000
            uniform border_color_down: #00000000
            uniform border_color_active: #00000000
            uniform border_color_focus: #00000000
            uniform border_color_disabled: #00000000

            // Checkmark color (white circle/thumb)
            uniform mark_color: #ffffff
            uniform mark_color_hover: #ffffff
            uniform mark_color_down: #ffffff
            uniform mark_color_active: #ffffff
            uniform mark_color_active_hover: #ffffff
            uniform mark_color_focus: #ffffff
            uniform mark_color_disabled: #f3f4f6

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Track dimensions - pill/capsule shape with half-circle ends
                let track_width = 50.0;
                let track_height = 26.0;
                let half_height = track_height / 2.0;

                // Draw pill shape: rectangle in middle + two half circles on ends
                // Left half-circle
                sdf.circle(half_height, half_height, half_height);
                // Right half-circle
                sdf.circle(track_width - half_height, half_height, half_height);
                // Middle rectangle
                sdf.rect(half_height, 0.0, track_width - track_height, track_height);

                // Use active state for on/off color
                let off_color = self.color;
                let on_color = self.color_active;
                let track_color = mix(off_color, on_color, self.active);
                sdf.fill(track_color);

                // Thumb (circle) - moves based on active state
                let thumb_size = 20.0;
                let thumb_margin = 3.0;
                let thumb_travel = track_width - thumb_size - (thumb_margin * 2.0);
                let thumb_x = thumb_margin + (thumb_travel * self.active) + (thumb_size / 2.0);
                let thumb_y = track_height / 2.0;

                sdf.circle(thumb_x, thumb_y, thumb_size / 2.0);
                sdf.fill(self.mark_color);

                return sdf.result;
            }
        }

        draw_text: {
            text_style: <THEME_FONT_REGULAR>{ font_size: 0.0 }
            fn get_color(self) -> vec4 {
                return vec4(0.0, 0.0, 0.0, 0.0);
            }
        }
    }

    // Status indicator dot
    StatusDot = <View> {
        width: 8, height: 8
        show_bg: true
        draw_bg: {
            // status: 0=not_connected (gray), 1=connecting (yellow), 2=connected (green), 3=error (red)
            instance status: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let center = self.rect_size / 2.0;
                let radius = min(center.x, center.y);
                sdf.circle(center.x, center.y, radius);

                // Color based on status
                let gray = mix(#9ca3af, #64748b, self.dark_mode);
                let yellow = mix(#f59e0b, #fbbf24, self.dark_mode);
                let green = mix(#22c55e, #4ade80, self.dark_mode);
                let red = mix(#ef4444, #f87171, self.dark_mode);

                // Select color based on status value
                let color = mix(
                    mix(gray, yellow, clamp(self.status, 0.0, 1.0)),
                    mix(green, red, clamp(self.status - 2.0, 0.0, 1.0)),
                    step(1.5, self.status)
                );

                sdf.fill(color);
                return sdf.result;
            }
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

        // Status indicator
        status_dot = <StatusDot> {}

        provider_name = <Label> {
            width: Fill
            draw_text: {
                instance dark_mode: 0.0
                fn get_color(self) -> vec4 {
                    return mix(#1f2937, #f1f5f9, self.dark_mode);
                }
                text_style: <THEME_FONT_REGULAR>{ font_size: 13.0 }
            }
        }

        // Enabled toggle on the right
        provider_enabled = <EnableToggle> {}
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

    // Test Connection button (secondary style)
    TestButton = <Button> {
        width: Fit, height: 40
        padding: {left: 20, right: 20, top: 10, bottom: 10}

        draw_bg: {
            instance hover: 0.0
            instance pressed: 0.0
            instance radius: 6.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let sz = self.rect_size - 2.0;
                // Secondary button: gray outline style
                let bg = mix(#ffffff, #1e293b, self.dark_mode);
                let border = mix(#d1d5db, #475569, self.dark_mode);
                let hover_bg = mix(#f3f4f6, #334155, self.dark_mode);
                let bg_color = mix(bg, hover_bg, self.hover);
                sdf.box(1.0, 1.0, sz.x, sz.y, self.radius);
                sdf.fill(bg_color);
                sdf.stroke(border, 1.0);
                return sdf.result;
            }
        }

        draw_text: {
            instance dark_mode: 0.0
            fn get_color(self) -> vec4 {
                return mix(#374151, #e2e8f0, self.dark_mode);
            }
            text_style: <THEME_FONT_BOLD>{ font_size: 12.0 }
        }

        text: "Test Connection"
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

        // Provider icons for dynamic loading (order: openai, anthropic, gemini, ollama, deepseek, nvidia, groq)
        provider_icons: [
            (ICON_OPENAI),
            (ICON_ANTHROPIC),
            (ICON_GEMINI),
            (ICON_OLLAMA),
            (ICON_DEEPSEEK),
            (ICON_NVIDIA),
            (ICON_GROQ),
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

            // Header with Add button
            <View> {
                width: Fill, height: Fit
                flow: Right
                padding: {left: 16, right: 16, top: 16, bottom: 12}
                align: {y: 0.5}

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

                <View> { width: Fill } // Spacer

                add_provider_button = <Button> {
                    width: 28, height: 28
                    padding: 0
                    draw_bg: {
                        instance hover: 0.0
                        instance pressed: 0.0
                        instance radius: 4.0
                        instance dark_mode: 0.0

                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            let sz = self.rect_size - 2.0;
                            let hover_color = mix(#e5e7eb, #374151, self.dark_mode);
                            let color = mix(vec4(0.0), hover_color, self.hover);
                            sdf.box(1.0, 1.0, sz.x, sz.y, self.radius);
                            sdf.fill(color);
                            return sdf.result;
                        }
                    }
                    draw_text: {
                        instance dark_mode: 0.0
                        fn get_color(self) -> vec4 {
                            return mix(#374151, #e2e8f0, self.dark_mode);
                        }
                        text_style: <THEME_FONT_BOLD>{ font_size: 16.0 }
                    }
                    text: "+"
                }
            }

            // Provider list (dynamic)
            providers_list = <PortalList> {
                width: Fill, height: Fill
                drag_scrolling: false

                ProviderListItem = <ProviderItem> {}
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
                test_button = <TestButton> {}

                <View> { width: Fill } // Spacer

                delete_provider_button = <Button> {
                    width: Fit, height: 40
                    padding: {left: 20, right: 20, top: 10, bottom: 10}
                    visible: false

                    draw_bg: {
                        instance hover: 0.0
                        instance pressed: 0.0
                        instance radius: 6.0

                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            let sz = self.rect_size - 2.0;
                            // Red button colors: #ef4444 -> #dc2626 -> #b91c1c
                            let base_color = vec4(0.937, 0.267, 0.267, 1.0);
                            let hover_color = vec4(0.863, 0.149, 0.149, 1.0);
                            let pressed_color = vec4(0.725, 0.110, 0.110, 1.0);
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

                    text: "Delete"
                }
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

            // Models section (shown after successful connection test)
            models_section = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 8
                margin: {top: 16}
                visible: false

                // Header row with label and Select All toggle
                models_header_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {y: 0.5}
                    spacing: 12

                    models_header = <Label> {
                        text: "Available Models"
                        draw_text: {
                            instance dark_mode: 0.0
                            fn get_color(self) -> vec4 {
                                return mix(#374151, #e2e8f0, self.dark_mode);
                            }
                            text_style: <THEME_FONT_BOLD>{ font_size: 13.0 }
                        }
                    }

                    <View> { width: Fill } // Spacer

                    select_all_label = <Label> {
                        text: "Select All"
                        draw_text: {
                            instance dark_mode: 0.0
                            fn get_color(self) -> vec4 {
                                return mix(#6b7280, #94a3b8, self.dark_mode);
                            }
                            text_style: <THEME_FONT_REGULAR>{ font_size: 11.0 }
                        }
                    }

                    select_all_toggle = <EnableToggle> {}
                }

                models_scroll = <View> {
                    width: Fill, height: 200
                    flow: Down
                    show_bg: true
                    draw_bg: {
                        instance radius: 6.0
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            let sz = self.rect_size - 2.0;
                            sdf.box(1.0, 1.0, sz.x, sz.y, self.radius);
                            let bg = mix(#f9fafb, #1e293b, self.dark_mode);
                            let border = mix(#e5e7eb, #374151, self.dark_mode);
                            sdf.fill(bg);
                            sdf.stroke(border, 1.0);
                            return sdf.result;
                        }
                    }

                    models_list = <PortalList> {
                        width: Fill, height: Fill
                        drag_scrolling: false

                        ModelItem = <View> {
                            width: Fill, height: Fit
                            padding: {left: 12, right: 12, top: 8, bottom: 8}
                            flow: Right
                            align: {y: 0.5}
                            spacing: 12

                            model_enabled = <EnableToggle> {}

                            model_name = <Label> {
                                width: Fill
                                draw_text: {
                                    instance dark_mode: 0.0
                                    fn get_color(self) -> vec4 {
                                        return mix(#374151, #e2e8f0, self.dark_mode);
                                    }
                                    text_style: <THEME_FONT_REGULAR>{ font_size: 11.0 }
                                }
                            }
                        }
                    }
                }
            }

            // Spacer
            <View> { width: Fill, height: Fill }
        }

        // Add Provider Modal (overlay)
        add_provider_modal = <View> {
            width: Fill, height: Fill
            flow: Overlay
            visible: false
            show_bg: true
            draw_bg: {
                fn pixel(self) -> vec4 {
                    return vec4(0.0, 0.0, 0.0, 0.5); // Semi-transparent backdrop
                }
            }

            // Center the modal content
            <View> {
                width: Fill, height: Fill
                align: {x: 0.5, y: 0.5}

                modal_content = <View> {
                    width: 400, height: Fit
                    flow: Down
                    padding: 24
                    spacing: 16
                    show_bg: true
                    draw_bg: {
                        instance radius: 8.0
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            let sz = self.rect_size - 2.0;
                            sdf.box(1.0, 1.0, sz.x, sz.y, self.radius);
                            // Slightly gray background so white inputs stand out
                            let bg = mix(#f3f4f6, #0f172a, self.dark_mode);
                            let border = mix(#d1d5db, #334155, self.dark_mode);
                            sdf.fill(bg);
                            sdf.stroke(border, 1.0);
                            return sdf.result;
                        }
                    }

                    // Modal header
                    modal_header = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        align: {y: 0.5}

                        modal_title = <Label> {
                            text: "Add Provider"
                            draw_text: {
                                instance dark_mode: 0.0
                                fn get_color(self) -> vec4 {
                                    return mix(#1f2937, #f1f5f9, self.dark_mode);
                                }
                                text_style: <THEME_FONT_BOLD>{ font_size: 18.0 }
                            }
                        }

                        <View> { width: Fill } // Spacer

                        close_modal_button = <Button> {
                            width: 24, height: 24
                            padding: 0
                            draw_bg: {
                                instance hover: 0.0
                                instance pressed: 0.0
                                instance radius: 4.0
                                instance dark_mode: 0.0

                                fn pixel(self) -> vec4 {
                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                    let sz = self.rect_size - 2.0;
                                    let hover_color = mix(#e5e7eb, #374151, self.dark_mode);
                                    let color = mix(vec4(0.0), hover_color, self.hover);
                                    sdf.box(1.0, 1.0, sz.x, sz.y, self.radius);
                                    sdf.fill(color);
                                    return sdf.result;
                                }
                            }
                            draw_text: {
                                instance dark_mode: 0.0
                                fn get_color(self) -> vec4 {
                                    return mix(#6b7280, #9ca3af, self.dark_mode);
                                }
                                text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                            }
                            text: "×"
                        }
                    }

                    // Provider name input
                    name_section = <View> {
                        width: Fill, height: Fit
                        flow: Down
                        spacing: 6

                        <SettingsLabel> { text: "Provider Name" }
                        new_provider_name = <SettingsTextInput> {
                            empty_text: "My Provider"
                        }
                    }

                    // API URL input
                    url_section = <View> {
                        width: Fill, height: Fit
                        flow: Down
                        spacing: 6

                        <SettingsLabel> { text: "API URL" }
                        new_provider_url = <SettingsTextInput> {
                            text: "https://api.example.com/v1"
                            empty_text: "https://api.example.com/v1"
                        }
                        <SettingsHint> { text: "OpenAI-compatible API endpoint" }
                    }

                    // API Key input
                    key_section = <View> {
                        width: Fill, height: Fit
                        flow: Down
                        spacing: 6

                        <SettingsLabel> { text: "API Key (optional)" }
                        new_provider_key = <SettingsTextInput> {
                            is_password: true
                            empty_text: "sk-..."
                        }
                    }

                    // Modal actions
                    modal_actions = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 12
                        margin: {top: 8}
                        align: {x: 1.0}

                        cancel_modal_button = <TestButton> {
                            text: "Cancel"
                        }
                        save_new_provider_button = <SaveButton> {
                            text: "Add Provider"
                        }
                    }
                }
            }
        }
    }
}
