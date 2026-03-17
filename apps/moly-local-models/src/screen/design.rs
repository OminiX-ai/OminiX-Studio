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
            fn get_color(self) -> vec4 {
                return #6b7280;
            }
            text_style: <FONT_MEDIUM>{ font_size: 11.0 }
        }
    }

    // Section title style - using Manrope SemiBold
    SectionTitle = <Label> {
        draw_text: {
            fn get_color(self) -> vec4 {
                return #1f2937;
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
            instance category: 0.0  // 0=LLM, 1=Image, 2=ASR, 3=TTS

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 3.0);

                // Category colors
                let llm_color = #dbeafe;     // Blue
                let image_color = #fce7f3;   // Pink
                let asr_color = #d1fae5;     // Green
                let tts_color = #fef3c7;     // Yellow

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
                instance category: 0.0
                fn get_color(self) -> vec4 {
                    let llm_color = #1e40af;
                    let image_color = #9d174d;
                    let asr_color = #047857;
                    let tts_color = #92401f;

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

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.circle(4.0, 4.0, 4.0);

                // Colors for each status
                let gray = #d1d5db;
                let yellow = #f59e0b;
                let green = #22c55e;
                let orange = #f97316;
                let red = #ef4444;

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
            instance progress: 0.0  // 0.0 to 1.0

            fn pixel(self) -> vec4 {
                // Background color
                let bg_color = #e5e7eb;
                // Progress fill color
                let fill_color = #3b82f6;

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

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.circle(12.0, 12.0, 10.0);

                let base = #00000000;
                let hover_color = #fee2e2;

                sdf.fill(mix(base, hover_color, self.hover));
                return sdf.result;
            }
        }

        draw_text: {
            fn get_color(self) -> vec4 {
                return #ef4444;
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

            fn pixel(self) -> vec4 {
                let base = #ffffff;
                let hover_color = #f1f5f9;
                let selected_color = #dbeafe;

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
                    fn get_color(self) -> vec4 {
                        return #1f2937;
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

    // Category group header shown above each group in the sidebar list
    CategoryGroupHeader = <View> {
        width: Fill, height: Fit
        padding: {left: 16, right: 16, top: 10, bottom: 4}

        show_bg: true
        draw_bg: {
            fn pixel(self) -> vec4 {
                return #ffffff;
            }
        }

        category_header_label = <Label> {
            draw_text: {
                fn get_color(self) -> vec4 {
                    return #9ca3af;
                }
                text_style: <FONT_SEMIBOLD>{ font_size: 10.0 }
            }
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
            instance btn_type: 0.0  // 0=primary, 1=danger

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 5.0);

                let primary = mix(#3b82f6, #2563fa, self.hover);
                let danger = mix(#ef4444, #dc2626, self.hover);

                let color = mix(primary, danger, self.btn_type);

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
                fn get_color(self) -> vec4 {
                    return #374151;
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
            fn pixel(self) -> vec4 {
                return #f8fafc;
            }
        }

        // Left panel: Model list
        models_panel = <View> {
            width: 260, height: Fill
            flow: Down
            show_bg: true
            draw_bg: {
                fn pixel(self) -> vec4 {
                    return #ffffff;
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
                        fn get_color(self) -> vec4 {
                            return #1f2937;
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
                    fn pixel(self) -> vec4 {
                        return #f1f5f9;
                    }
                }
            }

            // Model list
            models_list = <PortalList> {
                width: Fill, height: Fill
                flow: Down

                ModelItem = <ModelListItem> {}
                CategoryHeader = <CategoryGroupHeader> {}
            }
        }

        // Vertical divider
        <View> {
            width: 1, height: Fill
            show_bg: true
            draw_bg: {
                fn pixel(self) -> vec4 {
                    return #f1f5f9;
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
                fn pixel(self) -> vec4 {
                    return #f8fafc;
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
                        fn get_color(self) -> vec4 {
                            return #1f2937;
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
                    fn get_color(self) -> vec4 {
                        return #6b7280;
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
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 6.0);
                        sdf.fill(#ffffff);
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
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 4.0);
                            sdf.fill(#e5e7eb);
                            return sdf.result;
                        }
                    }

                    // Progress bar fill
                    progress_bar_fill = <View> {
                        width: 0, height: Fill
                        show_bg: true
                        draw_bg: {
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 4.0);
                                sdf.fill(#3b82f6);
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
                        fn get_color(self) -> vec4 {
                            return #6b7280;
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
                    fn get_color(self) -> vec4 {
                        return #6b7280;
                    }
                    text_style: <FONT_REGULAR>{ font_size: 11.0 }
                }
            }

            // Spacer
            <View> { width: Fill, height: Fill }
        }
    }
}
