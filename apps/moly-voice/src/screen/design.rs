use makepad_widgets::*;

use super::VoiceApp;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use moly_widgets::theme::*;

    // Voice status indicator (green = ready, gray = not trained)
    VoiceStatusDot = <View> {
        width: 8, height: 8
        margin: {right: 10}
        draw_bg: {
            instance ready: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.circle(4.0, 4.0, 4.0);
                let color = mix(#d1d5db, #22c55e, self.ready);
                sdf.fill(color);
                return sdf.result;
            }
        }
    }

    // Voice list item template for PortalList
    VoiceListItem = <View> {
        width: Fill, height: 44
        padding: {left: 12, right: 12, top: 8, bottom: 8}
        cursor: Hand
        event_order: Down
        flow: Right
        align: {y: 0.5}
        show_bg: true
        draw_bg: {
            instance selected: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 4.0);
                let normal = #ffffff;
                let selected_color = #eff6ff;
                sdf.fill(mix(normal, selected_color, self.selected));
                return sdf.result;
            }
        }
        voice_status = <VoiceStatusDot> {}
        voice_name = <Label> {
            width: Fill
            draw_text: {
                color: #1f2937
                text_style: <FONT_MEDIUM>{ font_size: 13.0 }
            }
        }
    }

    // Empty state item for PortalList when no voices exist
    VoiceEmptyItem = <View> {
        width: Fill, height: Fit
        padding: {left: 12, right: 12, top: 20, bottom: 20}
        align: {x: 0.5}
        <Label> {
            text: "No voices yet.\nClick + New to train one."
            draw_text: {
                color: #9ca3af
                text_style: { font_size: 12.0 }
                wrap: Word
            }
        }
    }

    // Form field label (fixed width for alignment)
    FieldLabel = <Label> {
        width: 90, height: Fit
        margin: {right: 8}
        draw_text: {
            color: #374151
            text_style: <FONT_MEDIUM>{ font_size: 12.0 }
        }
    }

    // Text input container with border
    InputContainer = <View> {
        width: Fill, height: 32
        show_bg: true
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, 4.0);
                sdf.fill(#f9fafb);
                sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, 4.0);
                sdf.stroke(#d1d5db, 1.0);
                return sdf.result;
            }
        }
        padding: {left: 8, right: 8}
        align: {y: 0.5}
    }

    // Option button for quality/language selection
    OptionButton = <Button> {
        width: Fit, height: 28
        padding: {left: 10, right: 10, top: 4, bottom: 4}
        margin: {right: 4}
        animator: {
            hover = {
                default: off,
                off = { from: {all: Forward {duration: 0.15}} apply: { draw_bg: {hover: 0.0} } }
                on  = { from: {all: Forward {duration: 0.15}} apply: { draw_bg: {hover: 1.0} } }
            }
        }
        draw_bg: {
            instance hover: 0.0
            instance selected: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 4.0);
                let normal = mix(#f3f4f6, #e5e7eb, self.hover);
                let sel = #dbeafe;
                sdf.fill(mix(normal, sel, self.selected));
                return sdf.result;
            }
        }
        draw_text: {
            text_style: <FONT_MEDIUM>{ font_size: 11.0 }
            color: #374151
        }
    }

    // Primary (blue) action button
    PrimaryButton = <Button> {
        width: Fit, height: 34
        padding: {left: 16, right: 16, top: 8, bottom: 8}
        margin: {right: 8}
        animator: {
            hover = {
                default: off,
                off = { from: {all: Forward {duration: 0.15}} apply: { draw_bg: {hover: 0.0} } }
                on  = { from: {all: Forward {duration: 0.15}} apply: { draw_bg: {hover: 1.0} } }
            }
        }
        draw_bg: {
            instance hover: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 6.0);
                sdf.fill(mix(#3b82f6, #1d4fd8, self.hover));
                return sdf.result;
            }
        }
        draw_text: {
            text_style: <FONT_MEDIUM>{ font_size: 13.0 }
            color: #ffffff
        }
    }

    // Secondary (gray) action button
    SecondaryButton = <Button> {
        width: Fit, height: 34
        padding: {left: 16, right: 16, top: 8, bottom: 8}
        margin: {right: 8}
        animator: {
            hover = {
                default: off,
                off = { from: {all: Forward {duration: 0.15}} apply: { draw_bg: {hover: 0.0} } }
                on  = { from: {all: Forward {duration: 0.15}} apply: { draw_bg: {hover: 1.0} } }
            }
        }
        draw_bg: {
            instance hover: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 6.0);
                sdf.fill(mix(#f3f4f6, #e5e7eb, self.hover));
                return sdf.result;
            }
        }
        draw_text: {
            text_style: <FONT_MEDIUM>{ font_size: 13.0 }
            color: #374151
        }
    }

    // Denoise toggle button
    DenoiseToggleButton = <Button> {
        width: Fit, height: 28
        padding: {left: 10, right: 10, top: 4, bottom: 4}
        margin: {left: 16}
        text: "Denoise"
        animator: {
            hover = {
                default: off,
                off = { from: {all: Forward {duration: 0.15}} apply: { draw_bg: {hover: 0.0} } }
                on  = { from: {all: Forward {duration: 0.15}} apply: { draw_bg: {hover: 1.0} } }
            }
        }
        draw_bg: {
            instance hover: 0.0
            instance active: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 4.0);
                let off_c = mix(#f3f4f6, #e5e7eb, self.hover);
                let on_c  = mix(#dcfce7, #bbf7d0, self.hover);
                sdf.fill(mix(off_c, on_c, self.active));
                return sdf.result;
            }
        }
        draw_text: {
            text_style: <FONT_MEDIUM>{ font_size: 11.0 }
            color: #374151
        }
    }

    // Section divider with centered label
    SectionDivider = <View> {
        width: Fill, height: Fit
        flow: Right
        align: {y: 0.5}
        margin: {bottom: 16}

        divider_left = <View> {
            width: Fill, height: 1
            show_bg: true
            draw_bg: { color: #e5e7eb }
            margin: {right: 10, top: 8, bottom: 8}
        }
        divider_label = <Label> {
            draw_text: {
                color: #9ca3af
                text_style: <FONT_SEMIBOLD>{ font_size: 10.0 }
            }
        }
        <View> {
            width: Fill, height: 1
            show_bg: true
            draw_bg: { color: #e5e7eb }
            margin: {left: 10, top: 8, bottom: 8}
        }
    }

    pub VoiceApp = {{VoiceApp}} {
        width: Fill, height: Fill
        flow: Right

        // ── Left panel: voice list (260px) ─────────────────────────────
        voices_panel = <View> {
            width: 260, height: Fill
            show_bg: true
            draw_bg: { color: #ffffff }
            flow: Down

            // Header with title and + New button
            panel_header = <View> {
                width: Fill, height: 52
                padding: {left: 12, right: 12, top: 10, bottom: 10}
                show_bg: true
                draw_bg: { color: #f8fafc }
                flow: Right
                align: {y: 0.5}

                <Label> {
                    width: Fill
                    text: "Voices"
                    draw_text: {
                        color: #1f2937
                        text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                    }
                }

                new_voice_btn = <Button> {
                    width: Fit, height: Fit
                    padding: {left: 10, right: 10, top: 5, bottom: 5}
                    text: "+ New"
                    animator: {
                        hover = {
                            default: off,
                            off = { from: {all: Forward {duration: 0.15}} apply: { draw_bg: {hover: 0.0} } }
                            on  = { from: {all: Forward {duration: 0.15}} apply: { draw_bg: {hover: 1.0} } }
                        }
                    }
                    draw_bg: {
                        instance hover: 0.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 4.0);
                            sdf.fill(mix(#3b82f6, #1d4fd8, self.hover));
                            return sdf.result;
                        }
                    }
                    draw_text: {
                        text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                        color: #ffffff
                    }
                }
            }

            // Separator line
            <View> {
                width: Fill, height: 1
                show_bg: true
                draw_bg: { color: #f1f5f9 }
            }

            // Scrollable voice list
            voices_list = <PortalList> {
                width: Fill, height: Fill
                flow: Down
                VoiceListItem = <VoiceListItem> {}
                VoiceEmptyItem = <VoiceEmptyItem> {}
            }
        }

        // Vertical separator between panels
        <View> {
            width: 1, height: Fill
            show_bg: true
            draw_bg: { color: #e5e7eb }
        }

        // ── Right panel: training + synthesis (fill) ────────────────────
        voice_details = <ScrollYView> {
            width: Fill, height: Fill
            flow: Down
            padding: {left: 24, right: 24, top: 20, bottom: 24}

            // ── TRAIN NEW VOICE ──
            train_divider = <SectionDivider> {
                divider_label = { text: "TRAIN NEW VOICE" }
            }

            // Voice Name
            name_row = <View> {
                width: Fill, height: Fit
                flow: Right
                align: {y: 0.5}
                margin: {bottom: 10}

                <FieldLabel> { text: "Voice Name" }
                name_input_container = <InputContainer> {
                    voice_name_input = <TextInput> {
                        width: Fill, height: Fill
                        empty_text: "e.g. my-voice"
                        draw_text: {
                            color: #1f2937
                            color_focus: #1f2937
                            color_empty: #9ca3af
                            color_empty_focus: #9ca3af
                            text_style: { font_size: 13.0 }
                        }
                        draw_bg: {
                            fn pixel(self) -> vec4 { return vec4(0.0, 0.0, 0.0, 0.0); }
                        }
                        draw_selection: { color: #bfdbfe color_focus: #bfdbfe }
                        draw_cursor: { color: #1f2937 }
                    }
                }
            }

            // Audio File
            audio_row = <View> {
                width: Fill, height: Fit
                flow: Right
                align: {y: 0.5}
                margin: {bottom: 10}

                <FieldLabel> { text: "Audio File" }
                audio_input_container = <InputContainer> {
                    audio_path_input = <TextInput> {
                        width: Fill, height: Fill
                        empty_text: "/path/to/reference.wav"
                        draw_text: {
                            color: #1f2937
                            color_focus: #1f2937
                            color_empty: #9ca3af
                            color_empty_focus: #9ca3af
                            text_style: { font_size: 13.0 }
                        }
                        draw_bg: {
                            fn pixel(self) -> vec4 { return vec4(0.0, 0.0, 0.0, 0.0); }
                        }
                        draw_selection: { color: #bfdbfe color_focus: #bfdbfe }
                        draw_cursor: { color: #1f2937 }
                    }
                }
            }

            // Transcript
            transcript_row = <View> {
                width: Fill, height: Fit
                flow: Right
                align: {y: 0.5}
                margin: {bottom: 10}

                <FieldLabel> { text: "Transcript" }
                transcript_input_container = <InputContainer> {
                    transcript_input = <TextInput> {
                        width: Fill, height: Fill
                        empty_text: "Transcription of the reference audio"
                        draw_text: {
                            color: #1f2937
                            color_focus: #1f2937
                            color_empty: #9ca3af
                            color_empty_focus: #9ca3af
                            text_style: { font_size: 13.0 }
                        }
                        draw_bg: {
                            fn pixel(self) -> vec4 { return vec4(0.0, 0.0, 0.0, 0.0); }
                        }
                        draw_selection: { color: #bfdbfe color_focus: #bfdbfe }
                        draw_cursor: { color: #1f2937 }
                    }
                }
            }

            // Quality selection
            quality_row = <View> {
                width: Fill, height: Fit
                flow: Right
                align: {y: 0.5}
                margin: {bottom: 10}

                <FieldLabel> { text: "Quality" }
                quality_fast_btn     = <OptionButton> { text: "Fast" }
                quality_standard_btn = <OptionButton> { text: "Standard" }
                quality_high_btn     = <OptionButton> { text: "High" }
            }

            // Language + Denoise
            lang_row = <View> {
                width: Fill, height: Fit
                flow: Right
                align: {y: 0.5}
                margin: {bottom: 18}

                <FieldLabel> { text: "Language" }
                lang_auto_btn = <OptionButton> { text: "Auto" }
                lang_zh_btn   = <OptionButton> { text: "ZH" }
                lang_en_btn   = <OptionButton> { text: "EN" }
                denoise_btn   = <DenoiseToggleButton> {}
            }

            // Train / Cancel buttons
            train_buttons_row = <View> {
                width: Fill, height: Fit
                flow: Right
                margin: {bottom: 14}

                train_btn        = <PrimaryButton>   { text: "Upload & Train" }
                cancel_train_btn = <SecondaryButton>  { text: "Cancel" }
            }

            // Progress section (hidden until training starts; visibility controlled by apply_over)
            progress_section = <View> {
                width: Fill, height: Fit
                flow: Down
                margin: {bottom: 14}

                progress_info_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {y: 0.5}
                    margin: {bottom: 6}

                    progress_stage_label = <Label> {
                        width: Fill
                        text: "Initializing..."
                        draw_text: {
                            color: #374151
                            text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                        }
                    }
                    progress_pct_label = <Label> {
                        text: "0%"
                        draw_text: {
                            color: #6b7280
                            text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                        }
                    }
                }

                // Progress bar
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
                    progress_fill = <View> {
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
            }

            // Training status / error message (is_error: 1.0=red, 0.0=green)
            train_status_label = <Label> {
                width: Fill
                margin: {bottom: 14}
                draw_text: {
                    instance is_error: 1.0
                    fn get_color(self) -> vec4 {
                        let green = vec4(0.086, 0.639, 0.29, 1.0);
                        let red   = vec4(0.937, 0.267, 0.267, 1.0);
                        return mix(green, red, self.is_error);
                    }
                    text_style: { font_size: 12.0 }
                    wrap: Word
                }
            }

            // ── SYNTHESIZE ──
            synth_divider = <SectionDivider> {
                margin: {top: 8, bottom: 16}
                divider_label = { text: "SYNTHESIZE" }
            }

            // Voice + Speed selection row
            synth_config_row = <View> {
                width: Fill, height: Fit
                flow: Right
                align: {y: 0.5}
                margin: {bottom: 10}

                <Label> {
                    text: "Voice:"
                    margin: {right: 8}
                    draw_text: {
                        color: #374151
                        text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                    }
                }
                synth_voice_label = <Label> {
                    width: Fill
                    text: "(select a voice from the list)"
                    draw_text: {
                        color: #9ca3af
                        text_style: { font_size: 12.0 }
                    }
                }
                <Label> {
                    text: "Speed:"
                    margin: {right: 8}
                    draw_text: {
                        color: #374151
                        text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                    }
                }
                speed_input_container = <View> {
                    width: 64, height: 28
                    show_bg: true
                    draw_bg: {
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, 4.0);
                            sdf.fill(#f9fafb);
                            sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, 4.0);
                            sdf.stroke(#d1d5db, 1.0);
                            return sdf.result;
                        }
                    }
                    padding: {left: 6, right: 6}
                    align: {y: 0.5}
                    speed_input = <TextInput> {
                        width: Fill, height: Fill
                        text: "1.0"
                        draw_text: {
                            color: #1f2937
                            color_focus: #1f2937
                            text_style: { font_size: 12.0 }
                        }
                        draw_bg: {
                            fn pixel(self) -> vec4 { return vec4(0.0, 0.0, 0.0, 0.0); }
                        }
                        draw_selection: { color: #bfdbfe color_focus: #bfdbfe }
                        draw_cursor: { color: #1f2937 }
                    }
                }
            }

            // Synthesis text area
            synth_text_container = <View> {
                width: Fill, height: 96
                show_bg: true
                draw_bg: {
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, 4.0);
                        sdf.fill(#f9fafb);
                        sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, 4.0);
                        sdf.stroke(#d1d5db, 1.0);
                        return sdf.result;
                    }
                }
                padding: {left: 8, right: 8, top: 8, bottom: 8}
                margin: {bottom: 10}

                synth_text_input = <TextInput> {
                    width: Fill, height: Fill
                    empty_text: "Enter text to synthesize..."
                    draw_text: {
                        color: #1f2937
                        color_focus: #1f2937
                        color_empty: #9ca3af
                        color_empty_focus: #9ca3af
                        text_style: { font_size: 13.0 }
                    }
                    draw_bg: {
                        fn pixel(self) -> vec4 { return vec4(0.0, 0.0, 0.0, 0.0); }
                    }
                    draw_selection: { color: #bfdbfe color_focus: #bfdbfe }
                    draw_cursor: { color: #1f2937 }
                }
            }

            // Generate / Play buttons
            synth_buttons_row = <View> {
                width: Fill, height: Fit
                flow: Right
                margin: {bottom: 8}

                generate_btn = <PrimaryButton>   { text: "Generate" }
                play_btn     = <SecondaryButton> { text: "▶ Play" }
            }

            // Synthesis status
            synth_status_label = <Label> {
                width: Fill
                text: ""
                draw_text: {
                    color: #6b7280
                    text_style: { font_size: 12.0 }
                }
            }
        }
    }
}

