use makepad_widgets::*;
use super::ModelHubApp;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use moly_widgets::theme::*;

    // ── Category badge (5 categories: LLM=0, VLM=1, ASR=2, TTS=3, Image=4) ──

    HubCategoryBadge = <View> {
        width: Fit, height: Fit
        padding: {left: 6, right: 6, top: 2, bottom: 2}
        margin: {left: 8}
        draw_bg: {
            instance cat: 0.0
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 3.0);
                // LLM indigo, VLM violet, ASR green, TTS amber, Image pink
                let c0 = mix(#dbeafe, #1a2f5a, self.dark_mode); // LLM bg
                let c1 = mix(#ede9fe, #2d1a5a, self.dark_mode); // VLM bg
                let c2 = mix(#d1fae5, #1a4d3a, self.dark_mode); // ASR bg
                let c3 = mix(#fef3c7, #5c4a1f, self.dark_mode); // TTS bg (no e in hex)
                let c4 = mix(#fce7f3, #5b1a3c, self.dark_mode); // Image bg
                // Select by integer step
                let w0 = 1.0 - step(0.5, self.cat);
                let w1 = step(0.5, self.cat) * (1.0 - step(1.5, self.cat));
                let w2 = step(1.5, self.cat) * (1.0 - step(2.5, self.cat));
                let w3 = step(2.5, self.cat) * (1.0 - step(3.5, self.cat));
                let w4 = step(3.5, self.cat);
                let color = c0 * w0 + c1 * w1 + c2 * w2 + c3 * w3 + c4 * w4;
                sdf.fill(color);
                return sdf.result;
            }
        }
        cat_label = <Label> {
            draw_text: {
                instance cat: 0.0
                instance dark_mode: 0.0
                fn get_color(self) -> vec4 {
                    let c0 = mix(#1a40af, #93c5fd, self.dark_mode); // LLM
                    let c1 = mix(#5b21b6, #c4b5fd, self.dark_mode); // VLM
                    let c2 = mix(#047857, #6de8b7, self.dark_mode); // ASR
                    let c3 = mix(#92400f, #fcd34d, self.dark_mode); // TTS (no e in 40f or fcd)
                    let c4 = mix(#9d174d, #f9a8d4, self.dark_mode); // Image
                    let w0 = 1.0 - step(0.5, self.cat);
                    let w1 = step(0.5, self.cat) * (1.0 - step(1.5, self.cat));
                    let w2 = step(1.5, self.cat) * (1.0 - step(2.5, self.cat));
                    let w3 = step(2.5, self.cat) * (1.0 - step(3.5, self.cat));
                    let w4 = step(3.5, self.cat);
                    return c0 * w0 + c1 * w1 + c2 * w2 + c3 * w3 + c4 * w4;
                }
                text_style: <FONT_MEDIUM>{ font_size: 9.0 }
            }
        }
    }

    // ── Status dot (0=gray/not-downloaded, 1=yellow/downloading, 2=green/ready, 4=red/error) ──

    HubStatusDot = <View> {
        width: 8, height: 8
        margin: {right: 8}
        draw_bg: {
            instance status: 0.0
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.circle(4.0, 4.0, 4.0);
                let gray   = mix(#d1d5db, #64748b, self.dark_mode);
                let yellow = mix(#f59b0b, #fbbf24, self.dark_mode); // amber, no 9e
                let green  = mix(#22c55a, #4ade80, self.dark_mode); // no 5e
                let red    = mix(#b91c1c, #f87171, self.dark_mode);
                let color = mix(gray,   yellow, clamp(self.status - 0.0, 0.0, 1.0));
                let color = mix(color,  green,  clamp(self.status - 1.0, 0.0, 1.0));
                let color = mix(color,  red,    clamp(self.status - 3.0, 0.0, 1.0));
                sdf.fill(color);
                return sdf.result;
            }
        }
    }

    // ── Inline mini progress bar ──

    HubInlineProgress = <View> {
        width: Fill, height: 3
        margin: {top: 4}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance progress: 0.0
            fn pixel(self) -> vec4 {
                let bg   = mix(#d1d5db, #374151, self.dark_mode);
                let fill = mix(#3b82f6, #60a5f6, self.dark_mode); // no e after digit
                return mix(bg, fill, step(self.pos.x, self.progress));
            }
        }
    }

    // ── Filter tab button ──

    HubFilterTab = <Button> {
        width: Fit, height: 26
        padding: {left: 10, right: 10}
        margin: {right: 4, bottom: 8}
        animator: {
            hover = {
                default: off,
                off = { from: {all: Forward {duration: 0.1}} apply: { draw_bg: {hover: 0.0} } }
                on  = { from: {all: Forward {duration: 0.1}} apply: { draw_bg: {hover: 1.0} } }
            }
            pressed = {
                default: off,
                off = { from: {all: Forward {duration: 0.07}} apply: { draw_bg: {pressed: 0.0} } }
                on  = { from: {all: Forward {duration: 0.07}} apply: { draw_bg: {pressed: 1.0} } }
            }
        }
        draw_bg: {
            instance hover: 0.0
            instance pressed: 0.0
            instance selected: 0.0
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 4.0);
                let base     = mix(#f1f5f9, #1a2636, self.dark_mode);
                let hov      = mix(#dbeafe, #1a3050, self.dark_mode);
                let sel      = mix(#3b82f6, #2563fa, self.dark_mode);
                let color = mix(mix(base, hov, self.hover), sel, self.selected);
                sdf.fill(mix(color, color * 0.9, self.pressed));
                return sdf.result;
            }
        }
        draw_text: {
            instance selected: 0.0
            instance dark_mode: 0.0
            fn get_color(self) -> vec4 {
                let normal = mix(#374151, #94a3b8, self.dark_mode);
                let active = #ffffff;
                return mix(normal, active, self.selected);
            }
            text_style: <FONT_MEDIUM>{ font_size: 11.0 }
        }
    }

    // ── Model list item ──

    HubModelListItem = <View> {
        width: Fill, height: Fit
        padding: {left: 14, right: 8, top: 9, bottom: 9}
        flow: Down
        cursor: Hand
        event_order: Down
        show_bg: true
        draw_bg: {
            instance hover: 0.0
            instance selected: 0.0
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                let base = mix(#ffffff, #1a2535, self.dark_mode);
                let hov  = mix(#f1f5f9, #263347, self.dark_mode);
                let sel  = mix(#dbeafe, #1a3a5a, self.dark_mode);
                return mix(mix(base, hov, self.hover), sel, self.selected);
            }
        }
        item_row = <View> {
            width: Fill, height: Fit
            flow: Right
            align: {y: 0.5}
            model_status = <HubStatusDot> {}
            model_name = <Label> {
                width: Fill
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#1f2937, #f1f5f9, self.dark_mode);
                    }
                    text_style: <FONT_REGULAR>{ font_size: 11.3 }
                    wrap: Ellipsis
                }
            }
        }
        inline_progress = <HubInlineProgress> { visible: false }
    }

    // ── Category group header ──

    HubCategoryGroupHeader = <View> {
        width: Fill, height: Fit
        padding: {left: 14, right: 14, top: 10, bottom: 4}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix(#ffffff, #1a2535, self.dark_mode);
            }
        }
        category_header_label = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                fn get_color(self) -> vec4 {
                    return mix(#9ca3af, #64748b, self.dark_mode);
                }
                text_style: <FONT_SEMIBOLD>{ font_size: 10.0 }
            }
        }
    }

    // ── Action button ──

    HubActionButton = <Button> {
        width: Fit, height: 32
        padding: {left: 14, right: 14}
        margin: {right: 8}
        animator: {
            hover = {
                default: off,
                off = { from: {all: Forward {duration: 0.1}} apply: { draw_bg: {hover: 0.0} } }
                on  = { from: {all: Forward {duration: 0.1}} apply: { draw_bg: {hover: 1.0} } }
            }
            pressed = {
                default: off,
                off = { from: {all: Forward {duration: 0.07}} apply: { draw_bg: {pressed: 0.0} } }
                on  = { from: {all: Forward {duration: 0.07}} apply: { draw_bg: {pressed: 1.0} } }
            }
        }
        draw_bg: {
            instance hover: 0.0
            instance pressed: 0.0
            instance danger: 0.0   // 0=primary blue, 1=danger red
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 5.0);
                let primary = mix(#3b82f6, #2563fa, self.hover);
                let danger  = mix(#b91c1c, #991b1b, self.hover); // no e after digit
                let color = mix(primary, danger, self.danger);
                sdf.fill(mix(color, color * 0.9, self.pressed));
                return sdf.result;
            }
        }
        draw_text: {
            fn get_color(self) -> vec4 { return #ffffff; }
            text_style: <FONT_MEDIUM>{ font_size: 11.0 }
        }
    }

    // ── Info row (label + value) ──

    HubInfoRow = <View> {
        width: Fill, height: Fit
        flow: Right
        padding: {top: 5, bottom: 5}
        align: {y: 0.0}
        info_label = <Label> {
            width: 100
            draw_text: {
                instance dark_mode: 0.0
                fn get_color(self) -> vec4 { return mix(#9ca3af, #64748b, self.dark_mode); }
                text_style: <FONT_MEDIUM>{ font_size: 11.0 }
            }
        }
        info_value = <Label> {
            width: Fill
            draw_text: {
                instance dark_mode: 0.0
                fn get_color(self) -> vec4 { return mix(#374151, #cbd5a0, self.dark_mode); }
                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                wrap: Word
            }
        }
    }

    // ── Progress bar ──

    HubProgressFill = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance progress: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x * self.progress, self.rect_size.y, 4.0);
                sdf.fill(mix(#3b82f6, #60a5f6, self.dark_mode));
                return sdf.result;
            }
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Main ModelHubApp widget
    // ─────────────────────────────────────────────────────────────────────────

    pub ModelHubApp = {{ModelHubApp}} {
        width: Fill, height: Fill
        flow: Right
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix(#f8fafc, #0c1221, self.dark_mode);
            }
        }

        // ── Left panel ──────────────────────────────────────────────────────
        hub_left_panel = <View> {
            width: 270, height: Fill
            flow: Down
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 { return mix(#ffffff, #111927, self.dark_mode); }
            }

            // Header
            <View> {
                width: Fill, height: 52
                padding: {left: 16, right: 16}
                align: {y: 0.5}
                <Label> {
                    text: "Model Hub"
                    draw_text: {
                        instance dark_mode: 0.0
                        fn get_color(self) -> vec4 {
                            return mix(#1f2937, #f1f5f9, self.dark_mode);
                        }
                        text_style: <FONT_SEMIBOLD>{ font_size: 15.0 }
                    }
                }
            }

            // Divider
            <View> {
                width: Fill, height: 1
                show_bg: true
                draw_bg: {
                    instance dark_mode: 0.0
                    fn pixel(self) -> vec4 { return mix(#f1f5f9, #263347, self.dark_mode); }
                }
            }

            // Search
            <View> {
                width: Fill, height: Fit
                padding: {left: 10, right: 10, top: 10, bottom: 4}
                search_input = <TextInput> {
                    width: Fill, height: 32
                    empty_text: "Search models..."
                    draw_bg: {
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 5.0);
                            sdf.fill(mix(#f1f5f9, #1a2535, self.dark_mode));
                            return sdf.result;
                        }
                    }
                    draw_text: {
                        color: #374151
                        color_empty: #9ca3af
                        text_style: { font_size: 12.0 }
                    }
                }
            }

            // Filter tabs
            <View> {
                width: Fill, height: Fit
                padding: {left: 10, right: 10, top: 4, bottom: 0}
                flow: Right
                filter_all   = <HubFilterTab> { text: "All",   draw_bg: { selected: 1.0 } }
                filter_llm   = <HubFilterTab> { text: "LLM" }
                filter_vlm   = <HubFilterTab> { text: "VLM" }
                filter_asr   = <HubFilterTab> { text: "ASR" }
                filter_tts   = <HubFilterTab> { text: "TTS" }
                filter_image = <HubFilterTab> { text: "Image" }
            }

            // Model list
            hub_model_list = <PortalList> {
                width: Fill, height: Fill
                flow: Down
                HubModelItem    = <HubModelListItem> {}
                HubCategoryHeader = <HubCategoryGroupHeader> {}
            }
        }

        // Vertical divider
        <View> {
            width: 1, height: Fill
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 { return mix(#f1f5f9, #263347, self.dark_mode); }
            }
        }

        // ── Right panel ─────────────────────────────────────────────────────
        hub_right_panel = <View> {
            width: Fill, height: Fill
            flow: Down
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 { return mix(#f8fafc, #0c1221, self.dark_mode); }
            }

            // Empty state
            hub_empty_state = <View> {
                width: Fill, height: Fill
                align: {x: 0.5, y: 0.4}
                visible: true
                <Label> {
                    text: "Select a model from the list"
                    draw_text: {
                        instance dark_mode: 0.0
                        fn get_color(self) -> vec4 {
                            return mix(#9ca3af, #64748b, self.dark_mode);
                        }
                        text_style: { font_size: 14.0 }
                    }
                }
            }

            // Model detail view
            model_details = <ScrollYView> {
                width: Fill, height: Fill
                visible: false
                flow: Down
                padding: {left: 32, right: 32, top: 28, bottom: 28}

                // Title row
                <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {y: 0.5}
                    margin: {bottom: 6}
                    hub_model_name = <Label> {
                        draw_text: {
                            instance dark_mode: 0.0
                            fn get_color(self) -> vec4 {
                                return mix(#1f2937, #f1f5f9, self.dark_mode);
                            }
                            text_style: <FONT_SEMIBOLD>{ font_size: 22.0 }
                        }
                    }
                }

                // Tags / meta line
                hub_model_tags = <Label> {
                    width: Fill, height: Fit
                    margin: {bottom: 12}
                    draw_text: {
                        instance dark_mode: 0.0
                        fn get_color(self) -> vec4 { return mix(#9ca3af, #64748b, self.dark_mode); }
                        text_style: { font_size: 11.0 }
                    }
                }

                // Description
                hub_model_desc = <Label> {
                    width: Fill, height: Fit
                    margin: {bottom: 20}
                    draw_text: {
                        instance dark_mode: 0.0
                        fn get_color(self) -> vec4 {
                            return mix(#4b5563, #94a3b8, self.dark_mode);
                        }
                        text_style: { font_size: 13.0 }
                        wrap: Word
                    }
                }

                // Info card
                <View> {
                    width: Fill, height: Fit
                    flow: Down
                    padding: {left: 16, right: 16, top: 14, bottom: 14}
                    margin: {bottom: 20}
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 8.0);
                            sdf.fill(mix(#ffffff, #111927, self.dark_mode));
                            return sdf.result;
                        }
                    }

                    <HubInfoRow> {
                        info_label = { text: "Status" }
                        status_value = <Label> {
                            draw_text: {
                                instance dark_mode: 0.0
                                fn get_color(self) -> vec4 {
                                    return mix(#374151, #cbd5a0, self.dark_mode);
                                }
                                text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                            }
                        }
                    }
                    <HubInfoRow> {
                        info_label = { text: "Category" }
                        category_value = <Label> {
                            draw_text: {
                                instance dark_mode: 0.0
                                fn get_color(self) -> vec4 {
                                    return mix(#374151, #cbd5a0, self.dark_mode);
                                }
                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                            }
                        }
                    }
                    <HubInfoRow> {
                        info_label = { text: "Size" }
                        size_value = <Label> {
                            draw_text: {
                                instance dark_mode: 0.0
                                fn get_color(self) -> vec4 {
                                    return mix(#374151, #cbd5a0, self.dark_mode);
                                }
                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                            }
                        }
                    }
                    <HubInfoRow> {
                        info_label = { text: "Memory" }
                        memory_value = <Label> {
                            draw_text: {
                                instance dark_mode: 0.0
                                fn get_color(self) -> vec4 {
                                    return mix(#374151, #cbd5a0, self.dark_mode);
                                }
                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                            }
                        }
                    }
                    <HubInfoRow> {
                        info_label = { text: "Path" }
                        path_value = <Label> {
                            width: Fill
                            draw_text: {
                                instance dark_mode: 0.0
                                fn get_color(self) -> vec4 {
                                    return mix(#374151, #cbd5a0, self.dark_mode);
                                }
                                text_style: <FONT_REGULAR>{ font_size: 10.0 }
                                wrap: Word
                            }
                        }
                    }
                    <HubInfoRow> {
                        info_label = { text: "API Type" }
                        api_value = <Label> {
                            draw_text: {
                                instance dark_mode: 0.0
                                fn get_color(self) -> vec4 {
                                    return mix(#374151, #cbd5a0, self.dark_mode);
                                }
                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                            }
                        }
                    }
                }

                // Action buttons
                <View> {
                    width: Fill, height: Fit
                    flow: Right
                    margin: {bottom: 16}
                    hub_download_btn = <HubActionButton> {
                        text: "Download"
                        draw_bg: { danger: 0.0 }
                    }
                    hub_cancel_btn = <HubActionButton> {
                        text: "Cancel"
                        visible: false
                        draw_bg: { danger: 1.0 }
                    }
                    hub_remove_btn = <HubActionButton> {
                        text: "Remove"
                        visible: false
                        draw_bg: { danger: 1.0 }
                    }
                }

                // Progress section
                hub_progress_section = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    visible: false
                    margin: {bottom: 12}

                    // Progress bar track
                    <View> {
                        width: Fill, height: 8
                        show_bg: true
                        draw_bg: {
                            instance dark_mode: 0.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 4.0);
                                sdf.fill(mix(#d1d5db, #374151, self.dark_mode));
                                return sdf.result;
                            }
                        }
                        hub_progress_fill = <HubProgressFill> {}
                    }

                    hub_progress_text = <Label> {
                        width: Fill, height: Fit
                        margin: {top: 6}
                        draw_text: {
                            instance dark_mode: 0.0
                            fn get_color(self) -> vec4 {
                                return mix(#6b7280, #94a3b8, self.dark_mode);
                            }
                            text_style: { font_size: 11.0 }
                        }
                    }
                }

                // Status message (manual install / errors)
                hub_status_msg = <Label> {
                    width: Fill, height: Fit
                    draw_text: {
                        instance dark_mode: 0.0
                        fn get_color(self) -> vec4 {
                            return mix(#6b7280, #94a3b8, self.dark_mode);
                        }
                        text_style: { font_size: 12.0 }
                        wrap: Word
                    }
                }

                <View> { width: Fill, height: Fill }
            }
        }
    }
}
