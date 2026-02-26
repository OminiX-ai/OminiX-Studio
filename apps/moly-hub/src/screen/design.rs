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
                let yellow = mix(#f59b0b, #fbbf24, self.dark_mode); // amber
                let green  = mix(#22c55a, #4ade80, self.dark_mode); // downloaded
                let blue   = mix(#3b82f6, #60a5fa, self.dark_mode); // loaded in API
                let red    = mix(#b91c1c, #f87171, self.dark_mode); // error (status=5)
                let color = mix(gray,   yellow, clamp(self.status - 0.0, 0.0, 1.0));
                let color = mix(color,  green,  clamp(self.status - 1.0, 0.0, 1.0));
                let color = mix(color,  blue,   clamp(self.status - 2.0, 0.0, 1.0));
                let color = mix(color,  red,    clamp(self.status - 4.0, 0.0, 1.0));
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
    // Panel helper widgets
    // ─────────────────────────────────────────────────────────────────────────

    HubInputLabel = <Label> {
        width: Fill, height: Fit
        margin: {bottom: 4, top: 12}
        draw_text: {
            instance dark_mode: 0.0
            fn get_color(self) -> vec4 {
                return mix(#6b7280, #94a3b8, self.dark_mode);
            }
            text_style: <FONT_SEMIBOLD>{ font_size: 10.0 }
        }
    }

    HubPanelInput = <TextInput> {
        width: Fill, height: 36
        margin: {bottom: 4}
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
            instance dark_mode: 0.0
            fn get_color(self) -> vec4 { return mix(#374151, #f1f5f9, self.dark_mode); }
            color: #374151
            color_empty: #9ca3af
            text_style: { font_size: 12.0 }
        }
    }

    HubPanelOutput = <View> {
        width: Fill, height: Fit
        padding: {left: 12, right: 12, top: 10, bottom: 10}
        margin: {top: 4, bottom: 16}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 6.0);
                sdf.fill(mix(#f1f5f9, #111927, self.dark_mode));
                return sdf.result;
            }
        }
        output_label = <Label> {
            width: Fill
            draw_text: {
                instance dark_mode: 0.0
                fn get_color(self) -> vec4 {
                    return mix(#1f2937, #d1d5db, self.dark_mode);
                }
                text_style: { font_size: 12.0 }
                wrap: Word
            }
        }
    }

    HubPanelStatus = <Label> {
        width: Fill, height: Fit
        margin: {top: 6}
        draw_text: {
            instance dark_mode: 0.0
            fn get_color(self) -> vec4 {
                return mix(#6b7280, #94a3b8, self.dark_mode);
            }
            text_style: { font_size: 11.0 }
            wrap: Word
        }
    }

    // Shared model detail header included in each type panel
    HubPanelHeader = <View> {
        width: Fill, height: Fit
        flow: Down
        padding: {left: 28, right: 28, top: 22, bottom: 16}

        // Model name
        <View> {
            width: Fill, height: Fit
            flow: Right
            align: {y: 0.5}
            margin: {bottom: 6}
            panel_model_name = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#1f2937, #f1f5f9, self.dark_mode);
                    }
                    text_style: <FONT_SEMIBOLD>{ font_size: 20.0 }
                }
            }
        }

        // Description
        panel_model_desc = <Label> {
            width: Fill, height: Fit
            margin: {bottom: 10}
            draw_text: {
                instance dark_mode: 0.0
                fn get_color(self) -> vec4 {
                    return mix(#6b7280, #94a3b8, self.dark_mode);
                }
                text_style: { font_size: 12.0 }
                wrap: Word
            }
        }

        // Status + size + memory info row
        <View> {
            width: Fill, height: Fit
            flow: Right
            align: {y: 0.5}
            margin: {bottom: 12}
            panel_status_dot = <HubStatusDot> {}
            panel_status_text = <Label> {
                margin: {right: 8}
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#374151, #cbd5a0, self.dark_mode);
                    }
                    text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                }
            }
            panel_sep1 = <Label> {
                text: "·"
                margin: {right: 8}
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#9ca3af, #4b5563, self.dark_mode);
                    }
                    text_style: { font_size: 11.0 }
                }
            }
            panel_size_text = <Label> {
                margin: {right: 8}
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#6b7280, #94a3b8, self.dark_mode);
                    }
                    text_style: { font_size: 11.0 }
                }
            }
            panel_sep2 = <Label> {
                text: "·"
                margin: {right: 8}
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#9ca3af, #4b5563, self.dark_mode);
                    }
                    text_style: { font_size: 11.0 }
                }
            }
            panel_mem_text = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#6b7280, #94a3b8, self.dark_mode);
                    }
                    text_style: { font_size: 11.0 }
                }
            }
        }

        // Action buttons
        <View> {
            width: Fill, height: Fit
            flow: Right
            margin: {bottom: 10}
            panel_download_btn = <HubActionButton> { text: "Download" }
            panel_cancel_btn = <HubActionButton> {
                text: "Cancel"
                visible: false
                draw_bg: { danger: 1.0 }
            }
            panel_remove_btn = <HubActionButton> {
                text: "Remove"
                visible: false
                draw_bg: { danger: 1.0 }
            }
        }

        // Runtime controls: Load / Unload (shown when model is downloaded)
        <View> {
            width: Fill, height: Fit
            flow: Right
            align: {y: 0.5}
            margin: {bottom: 8}
            panel_load_btn = <HubActionButton> {
                text: "Load"
                visible: false
            }
            panel_unload_btn = <HubActionButton> {
                text: "Unload"
                visible: false
            }
            panel_loading_label = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#6366f1, #818cf8, self.dark_mode);
                    }
                    text_style: <FONT_MEDIUM>{ font_size: 11.5 }
                }
                text: "Loading model..."
            }
            panel_chat_btn = <HubActionButton> {
                text: "Open in Chat"
            }
        }

        // Progress bar (visible while downloading)
        panel_progress_section = <View> {
            visible: false
            width: Fill, height: Fit
            flow: Down
            margin: {bottom: 8}
            panel_progress_bg = <View> {
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
                panel_progress_fill = <HubProgressFill> {}
            }
            panel_progress_text = <Label> {
                width: Fill, height: Fit
                margin: {top: 5}
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#6b7280, #94a3b8, self.dark_mode);
                    }
                    text_style: { font_size: 11.0 }
                }
            }
        }

        // Manual install / error message
        panel_status_msg = <Label> {
            width: Fill, height: Fit
            draw_text: {
                instance dark_mode: 0.0
                fn get_color(self) -> vec4 {
                    return mix(#6b7280, #94a3b8, self.dark_mode);
                }
                text_style: { font_size: 11.5 }
                wrap: Word
            }
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Main ModelHubApp widget
    // ─────────────────────────────────────────────────────────────────────────

    // ── Voice Studio footer item in model list ──
    HubVoiceStudioItem = <View> {
        width: Fill, height: Fit
        padding: {left: 14, right: 8, top: 12, bottom: 12}
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
        flow: Right
        align: {y: 0.5}
        <Label> {
            width: Fit
            margin: {right: 8}
            text: "🎙"
            draw_text: {
                text_style: { font_size: 14.0 }
            }
        }
        voice_studio_label = <Label> {
            width: Fill
            text: "Voice Studio"
            draw_text: {
                instance dark_mode: 0.0
                fn get_color(self) -> vec4 {
                    return mix(#1f2937, #f1f5f9, self.dark_mode);
                }
                text_style: <FONT_MEDIUM>{ font_size: 11.3 }
            }
        }
    }

    // ── Voice list item inside Voice Studio panel ──
    HubVoiceListItem = <View> {
        width: Fill, height: 40
        padding: {left: 12, right: 12, top: 8, bottom: 8}
        cursor: Hand
        event_order: Down
        flow: Right
        align: {y: 0.5}
        show_bg: true
        draw_bg: {
            instance selected: 0.0
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 4.0);
                let normal   = mix(#ffffff, #1a2535, self.dark_mode);
                let sel_col  = mix(#dbeafe, #1a3a5a, self.dark_mode);
                sdf.fill(mix(normal, sel_col, self.selected));
                return sdf.result;
            }
        }
        voice_status_dot = <View> {
            width: 8, height: 8
            margin: {right: 8}
            draw_bg: {
                instance ready: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.circle(4.0, 4.0, 4.0);
                    sdf.fill(mix(#d1d5db, #22c55a, self.ready));
                    return sdf.result;
                }
            }
        }
        voice_item_name = <Label> {
            width: Fill
            draw_text: {
                instance dark_mode: 0.0
                fn get_color(self) -> vec4 { return mix(#1f2937, #f1f5f9, self.dark_mode); }
                text_style: <FONT_REGULAR>{ font_size: 11.5 }
                wrap: Ellipsis
            }
        }
    }

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
                hub_title_label = <Label> {
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
            hub_header_divider = <View> {
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
                        instance dark_mode: 0.0
                        fn get_color(self) -> vec4 { return mix(#374151, #f1f5f9, self.dark_mode); }
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
                flow: RightWrap
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
                HubModelItem        = <HubModelListItem> {}
                HubCategoryHeader   = <HubCategoryGroupHeader> {}
                HubVoiceStudioItem  = <HubVoiceStudioItem> {}
            }
        }

        // Vertical divider – 8 px wide for easy dragging, visually 1px center line
        hub_main_divider = <View> {
            width: 8, height: Fill
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 {
                    // 1px opaque line in center, transparent on either side
                    let dist = abs(self.pos.x - 0.5) * self.rect_size.x;
                    let col  = mix(#e2e8f0, #374151, self.dark_mode);
                    return vec4(col.r, col.g, col.b, 1.0 - step(0.5, dist));
                }
            }
        }

        // ── Right panel: type-aware Overlay ──────────────────────────────────
        hub_right_panel = <View> {
            width: Fill, height: Fill
            flow: Overlay
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 { return mix(#f8fafc, #0c1221, self.dark_mode); }
            }

            // Empty state (default)
            hub_empty_state = <View> {
                width: Fill, height: Fill
                align: {x: 0.5, y: 0.4}
                visible: true
                hub_empty_label = <Label> {
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

            // ── LLM panel ────────────────────────────────────────────────────
            hub_llm_panel = <ScrollYView> {
                width: Fill, height: Fill
                visible: false
                flow: Down

                hub_panel_header = <HubPanelHeader> {}

                hub_llm_divider = <View> {
                    width: Fill, height: 1
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 { return mix(#f1f5f9, #263347, self.dark_mode); }
                    }
                }

                <View> {
                    width: Fill, height: Fit
                    flow: Down
                    padding: {left: 28, right: 28, top: 16, bottom: 32}

                    <HubInputLabel> { text: "SYSTEM PROMPT" }
                    llm_system = <HubPanelInput> {
                        height: 72
                        empty_text: "You are a helpful assistant..."
                    }

                    <HubInputLabel> { text: "USER MESSAGE" }
                    llm_user = <HubPanelInput> {
                        height: 60
                        empty_text: "Type your message here..."
                    }

                    <View> {
                        width: Fill, height: Fit
                        flow: Right
                        margin: {top: 10, bottom: 16}
                        llm_generate_btn = <HubActionButton> { text: "Generate" }
                    }

                    <HubInputLabel> { text: "RESPONSE" }
                    llm_response = <HubPanelOutput> {}
                    llm_status = <HubPanelStatus> {}
                }
            }

            // ── VLM panel ────────────────────────────────────────────────────
            hub_vlm_panel = <ScrollYView> {
                width: Fill, height: Fill
                visible: false
                flow: Down

                hub_panel_header = <HubPanelHeader> {}

                hub_vlm_divider = <View> {
                    width: Fill, height: 1
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 { return mix(#f1f5f9, #263347, self.dark_mode); }
                    }
                }

                <View> {
                    width: Fill, height: Fit
                    flow: Down
                    padding: {left: 28, right: 28, top: 16, bottom: 32}

                    <HubInputLabel> { text: "IMAGE FILE" }
                    <View> {
                        width: Fill, height: Fit
                        flow: Right
                        align: {y: 0.5}
                        margin: {bottom: 4}
                        vlm_image_path = <HubPanelInput> {
                            width: Fill, height: 36
                            margin: {right: 6, bottom: 0}
                        }
                        vlm_browse_btn = <HubActionButton> { text: "Browse..." margin: {right: 0} }
                    }

                    <HubInputLabel> { text: "USER MESSAGE" }
                    vlm_user = <HubPanelInput> {
                        height: 60
                        empty_text: "Describe this image..."
                    }

                    <View> {
                        width: Fill, height: Fit
                        flow: Right
                        margin: {top: 10, bottom: 16}
                        vlm_generate_btn = <HubActionButton> { text: "Generate" }
                    }

                    <HubInputLabel> { text: "RESPONSE" }
                    vlm_response = <HubPanelOutput> {}
                    vlm_status = <HubPanelStatus> {}
                }
            }

            // ── ASR panel ────────────────────────────────────────────────────
            hub_asr_panel = <ScrollYView> {
                width: Fill, height: Fill
                visible: false
                flow: Down

                hub_panel_header = <HubPanelHeader> {}

                hub_asr_divider = <View> {
                    width: Fill, height: 1
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 { return mix(#f1f5f9, #263347, self.dark_mode); }
                    }
                }

                <View> {
                    width: Fill, height: Fit
                    flow: Down
                    padding: {left: 28, right: 28, top: 16, bottom: 32}

                    <HubInputLabel> { text: "AUDIO FILE" }
                    <View> {
                        width: Fill, height: Fit
                        flow: Right
                        align: {y: 0.5}
                        margin: {bottom: 4}
                        asr_audio_path = <HubPanelInput> {
                            width: Fill, height: 36
                            margin: {right: 6, bottom: 0}
                        }
                        asr_browse_btn = <HubActionButton> { text: "Browse..." margin: {right: 0} }
                    }

                    <View> {
                        width: Fill, height: Fit
                        flow: Right
                        margin: {top: 10, bottom: 16}
                        asr_transcribe_btn = <HubActionButton> { text: "Transcribe" }
                    }

                    <HubInputLabel> { text: "TRANSCRIPT" }
                    asr_transcript = <HubPanelOutput> {}
                    asr_status = <HubPanelStatus> {}
                }
            }

            // ── TTS panel ────────────────────────────────────────────────────
            hub_tts_panel = <ScrollYView> {
                width: Fill, height: Fill
                visible: false
                flow: Down

                hub_panel_header = <HubPanelHeader> {}

                hub_tts_divider = <View> {
                    width: Fill, height: 1
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 { return mix(#f1f5f9, #263347, self.dark_mode); }
                    }
                }

                <View> {
                    width: Fill, height: Fit
                    flow: Down
                    padding: {left: 28, right: 28, top: 16, bottom: 32}

                    <HubInputLabel> { text: "VOICE ID" }
                    tts_voice_input = <HubPanelInput> {
                        empty_text: "default"
                    }

                    tts_voices_hint = <Label> {
                        width: Fill, height: Fit
                        margin: {top: 4, bottom: 8}
                        draw_text: {
                            instance dark_mode: 0.0
                            fn get_color(self) -> vec4 {
                                return mix(#9ca3af, #64748b, self.dark_mode);
                            }
                            text_style: { font_size: 10.5 }
                            wrap: Word
                        }
                    }

                    <HubInputLabel> { text: "TEXT TO SPEAK" }
                    tts_text_input = <HubPanelInput> {
                        height: 80
                        empty_text: "Enter text to synthesize..."
                    }

                    <View> {
                        width: Fill, height: Fit
                        flow: Right
                        margin: {top: 10, bottom: 16}
                        tts_generate_btn = <HubActionButton> { text: "Generate & Play" }
                    }

                    tts_status = <HubPanelStatus> {}
                }
            }

            // ── Image panel ──────────────────────────────────────────────────
            hub_image_panel = <ScrollYView> {
                width: Fill, height: Fill
                visible: false
                flow: Down

                hub_panel_header = <HubPanelHeader> {}

                hub_image_divider = <View> {
                    width: Fill, height: 1
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 { return mix(#f1f5f9, #263347, self.dark_mode); }
                    }
                }

                <View> {
                    width: Fill, height: Fit
                    flow: Down
                    padding: {left: 28, right: 28, top: 16, bottom: 32}

                    <HubInputLabel> { text: "PROMPT" }
                    img_prompt = <HubPanelInput> {
                        height: 72
                        empty_text: "A beautiful landscape..."
                    }

                    <HubInputLabel> { text: "NEGATIVE PROMPT (OPTIONAL)" }
                    img_neg_prompt = <HubPanelInput> {
                        height: 48
                        empty_text: "blurry, low quality..."
                    }

                    <View> {
                        width: Fill, height: Fit
                        flow: Right
                        margin: {top: 10, bottom: 16}
                        img_generate_btn = <HubActionButton> { text: "Generate Image" }
                    }

                    img_output_path = <Label> {
                        width: Fill, height: Fit
                        margin: {bottom: 8}
                        draw_text: {
                            instance dark_mode: 0.0
                            fn get_color(self) -> vec4 {
                                return mix(#374151, #94a3b8, self.dark_mode);
                            }
                            text_style: { font_size: 11.0 }
                            wrap: Word
                        }
                    }

                    img_status = <HubPanelStatus> {}
                }
            }

            // ── Voice Studio Panel ──────────────────────────────────────────────
            hub_voice_panel = <View> {
            width: Fill, height: Fill
            visible: false
            flow: Right

            // Left sub-panel: voice list + actions
            <View> {
                width: 240, height: Fill
                flow: Down
                show_bg: true
                draw_bg: {
                    instance dark_mode: 0.0
                    fn pixel(self) -> vec4 { return mix(#ffffff, #111927, self.dark_mode); }
                }

                // Header + New button
                <View> {
                    width: Fill, height: 48
                    padding: {left: 16, right: 8}
                    align: {y: 0.5}
                    flow: Right
                    voice_list_title = <Label> {
                        width: Fill
                        text: "Voices"
                        draw_text: {
                            instance dark_mode: 0.0
                            fn get_color(self) -> vec4 {
                                return mix(#1f2937, #f1f5f9, self.dark_mode);
                            }
                            text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                        }
                    }
                    voice_new_btn = <HubActionButton> {
                        text: "+ New"
                        padding: {left: 8, right: 8}
                        height: 28
                    }
                }

                voice_left_divider = <View> {
                    width: Fill, height: 1
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 { return mix(#f1f5f9, #263347, self.dark_mode); }
                    }
                }

                // Voice list
                voice_list = <PortalList> {
                    width: Fill, height: Fill
                    flow: Down
                    HubVoiceListItem = <HubVoiceListItem> {}
                }
            }

            // Vertical divider
            voice_panel_divider = <View> {
                width: 1, height: Fill
                show_bg: true
                draw_bg: {
                    instance dark_mode: 0.0
                    fn pixel(self) -> vec4 { return mix(#f1f5f9, #263347, self.dark_mode); }
                }
            }

            // Right sub-panel: training form + synthesis
            <ScrollYView> {
                width: Fill, height: Fill
                flow: Down
                padding: {left: 28, right: 28, top: 20, bottom: 32}

                // Training section header
                voice_training_title = <Label> {
                    width: Fill
                    margin: {bottom: 12}
                    text: "VOICE TRAINING"
                    draw_text: {
                        instance dark_mode: 0.0
                        fn get_color(self) -> vec4 {
                            return mix(#6b7280, #64748b, self.dark_mode);
                        }
                        text_style: <FONT_SEMIBOLD>{ font_size: 10.5 }
                    }
                }

                <HubInputLabel> { text: "VOICE NAME" }
                voice_name_input = <HubPanelInput> {
                    height: 36
                    empty_text: "My Voice"
                }

                <HubInputLabel> { text: "AUDIO FILE (.wav)" }
                <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {y: 0.5}
                    margin: {bottom: 4}
                    voice_audio_path_input = <HubPanelInput> {
                        width: Fill, height: 36
                        margin: {right: 6, bottom: 0}
                    }
                    voice_audio_browse_btn = <HubActionButton> { text: "Browse..." margin: {right: 0} }
                }

                <HubInputLabel> { text: "TRANSCRIPT (OPTIONAL)" }
                voice_transcript_input = <HubPanelInput> {
                    height: 60
                    empty_text: "Text spoken in the audio file..."
                }

                // Quality selector
                <HubInputLabel> { text: "QUALITY" }
                <View> {
                    width: Fill, height: Fit
                    flow: Right
                    margin: {bottom: 12}
                    voice_quality_fast     = <HubActionButton> { text: "Fast",     margin: {right: 6} }
                    voice_quality_standard = <HubActionButton> { text: "Standard", margin: {right: 6} }
                    voice_quality_high     = <HubActionButton> { text: "High" }
                }

                <View> {
                    width: Fill, height: Fit
                    flow: Right
                    margin: {bottom: 8}
                    voice_train_btn        = <HubActionButton> { text: "Train Voice", margin: {right: 8} }
                    voice_cancel_train_btn = <HubActionButton> {
                        text: "Cancel"
                        visible: false
                        draw_bg: { danger: 1.0 }
                    }
                }

                voice_train_status = <HubPanelStatus> {}

                // Divider
                voice_synth_divider = <View> {
                    width: Fill, height: 1
                    margin: {top: 20, bottom: 20}
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 { return mix(#f1f5f9, #263347, self.dark_mode); }
                    }
                }

                // Synthesis section header
                voice_synthesis_title = <Label> {
                    width: Fill
                    margin: {bottom: 12}
                    text: "VOICE SYNTHESIS"
                    draw_text: {
                        instance dark_mode: 0.0
                        fn get_color(self) -> vec4 {
                            return mix(#6b7280, #64748b, self.dark_mode);
                        }
                        text_style: <FONT_SEMIBOLD>{ font_size: 10.5 }
                    }
                }

                <HubInputLabel> { text: "TEXT TO SYNTHESIZE" }
                voice_synth_text = <HubPanelInput> {
                    height: 72
                    empty_text: "Enter text to synthesize..."
                }

                <HubInputLabel> { text: "SPEED (0.5 – 2.0)" }
                voice_speed_input = <HubPanelInput> {
                    height: 36
                    empty_text: "1.0"
                }

                <View> {
                    width: Fill, height: Fit
                    flow: Right
                    margin: {top: 10, bottom: 8}
                    voice_generate_btn = <HubActionButton> { text: "Synthesize", margin: {right: 8} }
                    voice_play_btn     = <HubActionButton> { text: "▶  Play" }
                }

                voice_synth_status = <HubPanelStatus> {}
            }
        }
    }
}
}
