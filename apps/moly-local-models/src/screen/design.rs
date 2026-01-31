use makepad_widgets::*;

use super::LocalModelsApp;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use moly_widgets::theme::*;

    // Local label style - using Manrope Medium
    LocalModelsLabel = <Label> {
        draw_text: {
            instance dark_mode: 0.0
            fn get_color(self) -> vec4 {
                return mix(#6b7280, #94a3b8, self.dark_mode);
            }
            text_style: <FONT_MEDIUM>{ font_size: 11.0 }
        }
    }

    // Section title style - using Manrope SemiBold
    SectionTitle = <Label> {
        draw_text: {
            instance dark_mode: 0.0
            fn get_color(self) -> vec4 {
                return mix(#1f2937, #f1f5f9, self.dark_mode);
            }
            text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
        }
    }

    // Model category badge
    CategoryBadge = <View> {
        width: Fit, height: Fit
        padding: {left: 6, right: 6, top: 2, bottom: 2}
        margin: {left: 8}

        draw_bg: {
            instance dark_mode: 0.0
            instance category: 0.0  // 0=LLM, 1=Image, 2=ASR, 3=TTS

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 3.0);

                // Category colors
                let llm_color = mix(#dbeafe, #1e3a5f, self.dark_mode);     // Blue
                let image_color = mix(#fce7f3, #5b2c4a, self.dark_mode);   // Pink
                let asr_color = mix(#d1fae5, #1a4d3a, self.dark_mode);     // Green
                let tts_color = mix(#fef3c7, #5c4a1f, self.dark_mode);     // Yellow

                let color = mix(
                    mix(llm_color, image_color, clamp(self.category, 0.0, 1.0)),
                    mix(asr_color, tts_color, clamp(self.category - 2.0, 0.0, 1.0)),
                    step(1.5, self.category)
                );

                sdf.fill(color);
                return sdf.result;
            }
        }

        category_label = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                instance category: 0.0
                fn get_color(self) -> vec4 {
                    let llm_color = mix(#1e40af, #93c5fd, self.dark_mode);
                    let image_color = mix(#9d174d, #f9a8d4, self.dark_mode);
                    let asr_color = mix(#047857, #6de8b7, self.dark_mode);
                    let tts_color = mix(#92401f, #fcd34d, self.dark_mode);

                    return mix(
                        mix(llm_color, image_color, clamp(self.category, 0.0, 1.0)),
                        mix(asr_color, tts_color, clamp(self.category - 2.0, 0.0, 1.0)),
                        step(1.5, self.category)
                    );
                }
                text_style: <FONT_MEDIUM>{ font_size: 9.0 }
            }
        }
    }

    // Status indicator - supports 6 states:
    // 0=not_available (gray), 1=downloading (yellow pulse), 2=ready (green)
    // 3=partial (orange), 4=error (red), 5=verifying (blue pulse)
    ModelStatusDot = <View> {
        width: 8, height: 8
        margin: {right: 10}
        draw_bg: {
            instance status: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.circle(4.0, 4.0, 4.0);

                // Colors for each status
                let gray = mix(#d1d5db, #64748b, self.dark_mode);
                let yellow = mix(#f59e0b, #fbbf24, self.dark_mode);
                let green = mix(#22c55e, #4ade80, self.dark_mode);
                let orange = mix(#f97316, #fb923c, self.dark_mode);
                let red = mix(#ef4444, #f87171, self.dark_mode);

                // Select color based on status (simplified - no animation)
                // 0=gray, 1=yellow, 2=green, 3=orange, 4+=red
                let color = mix(gray, yellow, clamp(self.status, 0.0, 1.0));
                let color = mix(color, green, clamp(self.status - 1.0, 0.0, 1.0));
                let color = mix(color, orange, clamp(self.status - 2.0, 0.0, 1.0));
                let color = mix(color, red, clamp(self.status - 3.0, 0.0, 1.0));

                sdf.fill(color);
                return sdf.result;
            }
        }
    }

    // Inline mini progress bar for list items
    InlineProgressBar = <View> {
        width: Fill, height: 3
        margin: {top: 4}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance progress: 0.0  // 0.0 to 1.0

            fn pixel(self) -> vec4 {
                // Background color
                let bg_color = mix(#e5e7eb, #374151, self.dark_mode);
                // Progress fill color
                let fill_color = mix(#3b82f6, #60a5fa, self.dark_mode);

                // Calculate if current pixel is in progress area
                // progress is 0.0-1.0, pos.x is 0.0-1.0
                let in_progress = step(self.pos.x, self.progress);

                return mix(bg_color, fill_color, in_progress);
            }
        }
    }

    // Small remove button for list items
    RemoveItemButton = <Button> {
        width: 24, height: 24
        margin: {left: 4}

        draw_bg: {
            instance hover: 0.0
            instance pressed: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.circle(12.0, 12.0, 10.0);

                let base = mix(#00000000, #00000000, self.dark_mode);
                let hover_color = mix(#fee2e2, #7f1d1d, self.dark_mode);

                sdf.fill(mix(base, hover_color, self.hover));
                return sdf.result;
            }
        }

        draw_text: {
            instance dark_mode: 0.0
            fn get_color(self) -> vec4 {
                return mix(#ef4444, #fca5a5, self.dark_mode);
            }
            text_style: <FONT_MEDIUM>{ font_size: 14.0 }
        }

        text: "×"
    }

    // Model list item in left panel - with inline progress bar
    // Widget hierarchy:
    //   ModelListItem (captures all clicks via event_order: Down)
    //   ├── item_content (transparent container for layout)
    //   │   ├── model_status (status dot)
    //   │   ├── model_name (label)
    //   │   ├── model_category (badge with category_label inside)
    //   │   └── remove_button_container → remove_item_button
    //   └── inline_progress (progress bar, visible when downloading)
    ModelListItem = <View> {
        width: Fill, height: Fit
        padding: {left: 16, right: 8, top: 10, bottom: 10}
        flow: Down
        cursor: Hand
        grab_key_focus: false
        // event_order: Down means parent handles events BEFORE children
        // This ensures we capture clicks on the entire item area
        event_order: Down

        show_bg: true
        draw_bg: {
            instance hover: 0.0
            instance selected: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let base = mix(#ffffff, #1e293b, self.dark_mode);
                let hover_color = mix(#f1f5f9, #334155, self.dark_mode);
                let selected_color = mix(#dbeafe, #1e3a5f, self.dark_mode);

                return mix(
                    mix(base, hover_color, self.hover),
                    selected_color,
                    self.selected
                );
            }
        }

        // Content row: status dot, name, category, remove button
        // This is a passive layout container - no event handling
        item_content = <View> {
            width: Fill, height: Fit
            flow: Right
            align: {y: 0.5}
            spacing: 8

            model_status = <ModelStatusDot> {}

            model_name = <Label> {
                width: Fill
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#1f2937, #f1f5f9, self.dark_mode);
                    }
                    text_style: <FONT_REGULAR>{ font_size: 11.3 }
                }
            }

            model_category = <CategoryBadge> { visible: false }

            // Remove button - needs to capture clicks before parent
            remove_button_container = <View> {
                width: Fit, height: Fit
                visible: false

                remove_item_button = <RemoveItemButton> {}
            }
        }

        // Inline progress bar (only visible when downloading)
        inline_progress = <InlineProgressBar> {
            visible: false
        }
    }

    // Action button style
    ActionButton = <Button> {
        width: Fit, height: 32
        padding: {left: 14, right: 14}
        margin: {right: 8}

        draw_bg: {
            instance hover: 0.0
            instance pressed: 0.0
            instance dark_mode: 0.0
            instance btn_type: 0.0  // 0=primary, 1=danger

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 5.0);

                let primary = mix(#3b82f6, #2563fa, self.hover);
                let primary_dark = mix(#2563fa, #1d4fd9, self.hover);
                let danger = mix(#ef4444, #dc2626, self.hover);
                let danger_dark = mix(#dc2626, #b91c1c, self.hover);

                let color = mix(
                    mix(primary, primary_dark, self.dark_mode),
                    mix(danger, danger_dark, self.dark_mode),
                    self.btn_type
                );

                sdf.fill(mix(color, color * 0.9, self.pressed));
                return sdf.result;
            }
        }

        draw_text: {
            fn get_color(self) -> vec4 {
                return #ffffff;
            }
            text_style: <FONT_MEDIUM>{ font_size: 11.0 }
        }
    }

    // Info row
    InfoRow = <View> {
        width: Fill, height: Fit
        flow: Right
        padding: {top: 6, bottom: 6}
        align: {y: 0.5}

        info_label = <LocalModelsLabel> {
            width: 100
        }

        info_value = <Label> {
            width: Fill
            draw_text: {
                instance dark_mode: 0.0
                fn get_color(self) -> vec4 {
                    return mix(#374151, #cbd5e1, self.dark_mode);
                }
                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                wrap: Word
            }
        }
    }

    // Main widget
    pub LocalModelsApp = {{LocalModelsApp}} {
        width: Fill, height: Fill
        flow: Right

        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix(#f8fafc, #0f172a, self.dark_mode);
            }
        }

        // Left panel: Model list
        models_panel = <View> {
            width: 260, height: Fill
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
                width: Fill, height: 48
                padding: {left: 16, right: 16}
                align: {y: 0.5}

                header_label = <Label> {
                    text: "Local Models"
                    draw_text: {
                        instance dark_mode: 0.0
                        fn get_color(self) -> vec4 {
                            return mix(#1f2937, #f1f5f9, self.dark_mode);
                        }
                        text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                    }
                }
            }

            // Divider
            <View> {
                width: Fill, height: 1
                show_bg: true
                draw_bg: {
                    instance dark_mode: 0.0
                    fn pixel(self) -> vec4 {
                        return mix(#f1f5f9, #334155, self.dark_mode);
                    }
                }
            }

            // Model list
            models_list = <PortalList> {
                width: Fill, height: Fill
                flow: Down

                ModelItem = <ModelListItem> {}
            }
        }

        // Vertical divider
        <View> {
            width: 1, height: Fill
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 {
                    return mix(#f1f5f9, #334155, self.dark_mode);
                }
            }
        }

        // Right panel: Model details
        model_view = <View> {
            width: Fill, height: Fill
            flow: Down
            padding: {left: 24, right: 24, top: 24, bottom: 24}

            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 {
                    return mix(#f8fafc, #0f172a, self.dark_mode);
                }
            }

            // Model header - smaller title
            model_header = <View> {
                width: Fill, height: Fit
                flow: Right
                align: {y: 0.5}
                margin: {bottom: 16}

                model_title = <Label> {
                    draw_text: {
                        instance dark_mode: 0.0
                        fn get_color(self) -> vec4 {
                            return mix(#1f2937, #f1f5f9, self.dark_mode);
                        }
                        text_style: <FONT_SEMIBOLD>{ font_size: 18.0 }
                    }
                }

                title_category = <CategoryBadge> { visible: false }
            }

            // Model description
            model_description = <Label> {
                width: Fill, height: Fit
                margin: {bottom: 20}
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#6b7280, #94a3b8, self.dark_mode);
                    }
                    text_style: <FONT_REGULAR>{ font_size: 12.0 }
                    wrap: Word
                }
            }

            // Info section
            info_section = <View> {
                width: Fill, height: Fit
                flow: Down
                padding: {top: 14, bottom: 14, left: 14, right: 14}
                margin: {bottom: 20}

                show_bg: true
                draw_bg: {
                    instance dark_mode: 0.0
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 6.0);
                        sdf.fill(mix(#ffffff, #1e293b, self.dark_mode));
                        return sdf.result;
                    }
                }

                status_row = <InfoRow> {
                    info_label = { text: "Status" }
                    info_value = { text: "Not Downloaded" }
                }

                size_row = <InfoRow> {
                    info_label = { text: "Size" }
                    info_value = { text: "~4.5 GB" }
                }

                memory_row = <InfoRow> {
                    info_label = { text: "Memory" }
                    info_value = { text: "16 GB required" }
                }

                path_row = <InfoRow> {
                    info_label = { text: "Path" }
                    info_value = { text: "~/.cache/huggingface/hub/..." }
                }

                url_row = <InfoRow> {
                    info_label = { text: "URL" }
                    info_value = { text: "https://huggingface.co/..." }
                }
            }

            // Action buttons
            actions = <View> {
                width: Fill, height: Fit
                flow: Right

                download_button = <ActionButton> {
                    text: "Download"
                    draw_bg: { btn_type: 0.0 }
                }

                cancel_button = <ActionButton> {
                    text: "Cancel"
                    visible: false
                    draw_bg: { btn_type: 1.0 }
                }

                remove_button = <ActionButton> {
                    text: "Remove"
                    draw_bg: { btn_type: 1.0 }
                }

                refresh_button = <ActionButton> {
                    text: "Refresh"
                    draw_bg: { btn_type: 0.0 }
                }
            }

            // Download progress section
            progress_section = <View> {
                width: Fill, height: Fit
                flow: Down
                margin: {top: 12}
                visible: false

                // Progress bar background
                progress_bar_bg = <View> {
                    width: Fill, height: 8
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 4.0);
                            sdf.fill(mix(#e5e7eb, #374151, self.dark_mode));
                            return sdf.result;
                        }
                    }

                    // Progress bar fill
                    progress_bar_fill = <View> {
                        width: 0, height: Fill
                        show_bg: true
                        draw_bg: {
                            instance dark_mode: 0.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 4.0);
                                sdf.fill(mix(#3b82f6, #60a5fa, self.dark_mode));
                                return sdf.result;
                            }
                        }
                    }
                }

                // Progress text
                progress_text = <Label> {
                    width: Fill, height: Fit
                    margin: {top: 6}
                    draw_text: {
                        instance dark_mode: 0.0
                        fn get_color(self) -> vec4 {
                            return mix(#6b7280, #94a3b8, self.dark_mode);
                        }
                        text_style: <FONT_REGULAR>{ font_size: 11.0 }
                    }
                }
            }

            // Status message
            status_message = <Label> {
                width: Fill, height: Fit
                margin: {top: 12}
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#6b7280, #94a3b8, self.dark_mode);
                    }
                    text_style: <FONT_REGULAR>{ font_size: 11.0 }
                }
            }

            // Spacer
            <View> { width: Fill, height: Fill }
        }
    }
}
