//! Chat Screen UI Design
//!
//! Contains the live_design! DSL block defining the UI layout and styling.

use makepad_widgets::*;

use super::{ChatApp, ChatHistoryItem, ChatHistoryPanel};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use moly_widgets::theme::*;
    use moly_kit::widgets::chat::Chat;
    use moly_kit::widgets::prompt_input::PromptInput;

    // Provider icons - registered so they can be loaded at runtime
    ICON_OPENAI = dep("crate://self/resources/providers/openai.png")
    ICON_ANTHROPIC = dep("crate://self/resources/providers/anthropic.png")
    ICON_GEMINI = dep("crate://self/resources/providers/gemini.png")
    ICON_OLLAMA = dep("crate://self/resources/providers/ollama.png")
    ICON_DEEPSEEK = dep("crate://self/resources/providers/deepseek.png")
    ICON_OPENROUTER = dep("crate://self/resources/providers/openrouter.png")
    ICON_SILICONFLOW = dep("crate://self/resources/providers/siliconflow.png")
    ICON_NVIDIA = dep("crate://self/resources/providers/nvidia.png")
    ICON_GROQ = dep("crate://self/resources/providers/groq.png")
    ICON_ZHIPU = dep("crate://self/resources/providers/zhipu.png")

    // Delete icon for chat history
    ICON_TRASH = dep("crate://self/resources/icons/trash.svg")

    // Individual chat history item - Widget with proper event handling
    pub ChatHistoryItem = {{ChatHistoryItem}} {
        width: Fill, height: Fit
        padding: {left: 12, right: 8, top: 8, bottom: 8}
        cursor: Hand
        show_bg: true
        draw_bg: {
            instance selected: 0.0
            instance hover: 0.0
            instance down: 0.0
            fn pixel(self) -> vec4 {
                let base = #ffffff;
                let selected_color = #eaecf0;
                let hover_color = #eaecf0;
                let color = mix(base, selected_color, self.selected);
                return mix(color, hover_color, self.hover * (1.0 - self.selected));
            }
        }

        // Animator enables finger event handling
        animator: {
            hover = {
                default: off
                off = {
                    from: {all: Forward {duration: 0.15}}
                    apply: {
                        draw_bg: {hover: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: 0.15}}
                    apply: {
                        draw_bg: {hover: 1.0}
                    }
                }
            }
            down = {
                default: off
                off = {
                    from: {all: Forward {duration: 0.2}}
                    apply: {
                        draw_bg: {down: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {down: 1.0}
                    }
                }
            }
        }

        flow: Right
        spacing: 4
        align: {y: 0.5}

        // Left side: title and date
        content = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 2

            title_label = <Label> {
                width: Fill
                draw_text: {
                    color: #1f2937
                    text_style: { font_size: 12.0 }
                    wrap: Ellipsis
                }
                text: "New Session"
            }

            date_label = <Label> {
                width: Fill
                draw_text: {
                    color: #6b7280
                    text_style: { font_size: 10.0 }
                }
                text: ""
            }
        }

        // Right side: delete button (visible on hover)
        delete_button = <View> {
            width: 24, height: 24
            align: {x: 0.5, y: 0.5}
            cursor: Hand
            show_bg: true
            draw_bg: {
                instance hover: 0.0
                fn pixel(self) -> vec4 {
                    let hover_color = #fee2e2;
                    return mix(vec4(0.0, 0.0, 0.0, 0.0), hover_color, self.hover);
                }
            }

            animator: {
                hover = {
                    default: off
                    off = {
                        from: {all: Forward {duration: 0.1}}
                        apply: { draw_bg: {hover: 0.0} }
                    }
                    on = {
                        from: {all: Forward {duration: 0.1}}
                        apply: { draw_bg: {hover: 1.0} }
                    }
                }
            }

            delete_icon = <Icon> {
                draw_icon: {
                    svg_file: (ICON_TRASH)
                    color: #9ca3af
                }
                icon_walk: { width: 18, height: 18 }
            }
        }
    }

    // Template alias for PortalList
    ChatHistoryItemTemplate = <ChatHistoryItem> {}

    // Chat history panel as a separate widget
    pub ChatHistoryPanel = {{ChatHistoryPanel}} {
        width: 220, height: Fill
        flow: Down
        show_bg: true
        draw_bg: {
            fn pixel(self) -> vec4 {
                return #f8fafc;
            }
        }

        // New chat button
        new_chat_container = <View> {
            width: Fill, height: Fit
            padding: 12

            new_chat_button = <Button> {
                width: Fill, height: Fit
                padding: {left: 12, right: 12, top: 10, bottom: 10}
                text: "+ New Session"
                draw_text: {
                    text_style: { font_size: 12.0 }
                    color: #ffffff
                }
                draw_bg: {
                    instance hover: 0.0
                    instance pressed: 0.0
                    fn pixel(self) -> vec4 {
                        let base = #3b82f6;
                        let hover_color = #2055ff;
                        let pressed_color = #1045cc;
                        let color = mix(base, hover_color, self.hover);
                        return mix(color, pressed_color, self.pressed);
                    }
                }
            }
        }

        // History header
        history_header = <View> {
            width: Fill, height: Fit
            padding: {left: 12, right: 12, top: 8, bottom: 8}

            history_title = <Label> {
                text: "History"
                draw_text: {
                    color: #6b7280
                    text_style: { font_size: 11.0 }
                }
            }
        }

        // Chat history list
        history_list = <PortalList> {
            width: Fill, height: Fill
            flow: Down

            ChatHistoryItem = <ChatHistoryItem> {}
        }
    }

    pub ChatApp = {{ChatApp}} {
        width: Fill, height: Fill
        flow: Down
        show_bg: true
        draw_bg: {
            fn pixel(self) -> vec4 {
                return #f5f7fa;
            }
        }

        // Provider icons for model selector and chat messages
        // Order: openai, anthropic, gemini, ollama, deepseek, openrouter, siliconflow, nvidia, groq, zhipu
        provider_icons: [
            (ICON_OPENAI),
            (ICON_ANTHROPIC),
            (ICON_GEMINI),
            (ICON_OLLAMA),
            (ICON_DEEPSEEK),
            (ICON_OPENROUTER),
            (ICON_SILICONFLOW),
            (ICON_NVIDIA),
            (ICON_GROQ),
            (ICON_ZHIPU),
        ]

        // Header with provider status
        header = <View> {
            width: Fill, height: Fit
            flow: Down
            padding: 16
            spacing: 4

            title_label = <Label> {
                text: "Session"
                draw_text: {
                    color: #1f2937
                    text_style: <FONT_SEMIBOLD>{ font_size: 20.0 }
                }
            }

            status_label = <Label> {
                text: "No provider configured - Go to Settings to add an API key"
                draw_text: {
                    color: #f59e0b
                    text_style: <FONT_REGULAR>{ font_size: 11.0 }
                }
            }
        }

        // Main content area - full width chat (history moved to shell sidebar)
        main_content = <View> {
            width: Fill, height: Fill
            flow: Overlay

            // Chat widget from moly-kit (always present)
            chat = <Chat> {
                width: Fill, height: Fill
            }

            // Empty chat welcome overlay (shows greeting when no messages)
            welcome_overlay = <View> {
                width: Fill, height: Fill
                flow: Down
                align: {x: 0.5, y: 0.35}
                spacing: 32
                padding: {left: 48, right: 48}
                visible: true

                // Greeting text
                greeting_label = <Label> {
                    width: Fit, height: Fit
                    text: "What can I help you with?"
                    draw_text: {
                        color: #1f2937
                        text_style: <FONT_SEMIBOLD>{ font_size: 28.0 }
                    }
                }

                // Prompt input
                welcome_prompt = <PromptInput> {
                    width: Fill, height: Fit
                }
            }

            // ── ASR welcome overlay ────────────────────────────────────────
            asr_welcome_overlay = <View> {
                width: Fill, height: Fill
                flow: Down
                align: {x: 0.5, y: 0.3}
                spacing: 24
                padding: {left: 64, right: 64}
                visible: false

                <Label> {
                    width: Fit, height: Fit
                    text: "Speech Recognition"
                    draw_text: { color: #1f2937, text_style: <FONT_SEMIBOLD>{ font_size: 28.0 } }
                }
                <Label> {
                    width: Fit, height: Fit
                    text: "Drop an audio file or click Browse to transcribe"
                    draw_text: { color: #6b7280, text_style: <FONT_REGULAR>{ font_size: 14.0 } }
                }

                // Drop zone
                asr_drop_zone = <View> {
                    width: 480, height: 160
                    align: {x: 0.5, y: 0.5}
                    flow: Down
                    spacing: 12
                    show_bg: true
                    draw_bg: {
                        instance hover: 0.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.box(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0, 12.0);
                            sdf.fill(mix(#f9fafb, #f0f9ff, self.hover));
                            // Dashed border effect
                            let bw = 1.5;
                            sdf.box(bw, bw, self.rect_size.x - 2.0*bw, self.rect_size.y - 2.0*bw, 12.0);
                            sdf.stroke(mix(#d1d5db, #3b82f6, self.hover), 1.5);
                            return sdf.result;
                        }
                    }

                    <Label> {
                        width: Fit, height: Fit
                        margin: {top: 40}
                        text: "Drop audio file here"
                        draw_text: { color: #9ca3af, text_style: <FONT_MEDIUM>{ font_size: 14.0 } }
                        align: {x: 0.5}
                    }
                    <Label> {
                        width: Fit, height: Fit
                        text: "MP3, WAV, M4A, FLAC"
                        draw_text: { color: #d1d5db, text_style: { font_size: 11.0 } }
                        align: {x: 0.5}
                    }

                    asr_browse_btn = <View> {
                        width: Fit, height: 36
                        cursor: Hand
                        align: {x: 0.5, y: 0.5}
                        padding: {left: 20, right: 20}
                        show_bg: true
                        draw_bg: {
                            instance hover: 0.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0, 8.0);
                                sdf.fill(mix(#3b82f6, #2563db, self.hover));
                                return sdf.result;
                            }
                        }
                        animator: {
                            hover = {
                                default: off
                                off = { from: {all: Forward{duration: 0.15}}, apply: {draw_bg: {hover: 0.0}} }
                                on  = { from: {all: Forward{duration: 0.15}}, apply: {draw_bg: {hover: 1.0}} }
                            }
                        }
                        <Label> {
                            text: "Browse Files"
                            draw_text: { color: #ffffff, text_style: <FONT_MEDIUM>{ font_size: 13.0 } }
                        }
                    }
                }

                // File name display (after selection)
                asr_file_label = <View> {
                    width: Fit, height: Fit
                    visible: false
                    label = <Label> {
                        width: Fit, height: Fit
                        text: ""
                        draw_text: { color: #374151, text_style: <FONT_MEDIUM>{ font_size: 13.0 } }
                    }
                }

                // Transcribe button
                asr_transcribe_btn = <View> {
                    width: Fit, height: 44
                    cursor: Hand
                    visible: false
                    align: {x: 0.5, y: 0.5}
                    padding: {left: 28, right: 28}
                    show_bg: true
                    draw_bg: {
                        instance hover: 0.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.box(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0, 10.0);
                            sdf.fill(mix(#10b981, #059669, self.hover));
                            return sdf.result;
                        }
                    }
                    animator: {
                        hover = {
                            default: off
                            off = { from: {all: Forward{duration: 0.15}}, apply: {draw_bg: {hover: 0.0}} }
                            on  = { from: {all: Forward{duration: 0.15}}, apply: {draw_bg: {hover: 1.0}} }
                        }
                    }
                    <Label> {
                        text: "Transcribe"
                        draw_text: { color: #ffffff, text_style: <FONT_SEMIBOLD>{ font_size: 14.0 } }
                    }
                }

                // Result area
                asr_result_label = <View> {
                    width: Fill, height: Fit
                    visible: false
                    label = <Label> {
                        width: Fill, height: Fit
                        text: ""
                        draw_text: { color: #1f2937, text_style: <FONT_REGULAR>{ font_size: 14.0 }, wrap: Word }
                    }
                }
            }

            // ── TTS welcome overlay ────────────────────────────────────────
            tts_welcome_overlay = <View> {
                width: Fill, height: Fill
                flow: Down
                align: {x: 0.5, y: 0.3}
                spacing: 24
                padding: {left: 64, right: 64}
                visible: false

                <Label> {
                    width: Fit, height: Fit
                    text: "Text to Speech"
                    draw_text: { color: #1f2937, text_style: <FONT_SEMIBOLD>{ font_size: 28.0 } }
                }
                <Label> {
                    width: Fit, height: Fit
                    text: "Enter text to convert to speech"
                    draw_text: { color: #6b7280, text_style: <FONT_REGULAR>{ font_size: 14.0 } }
                }

                // Text input area
                tts_input = <TextInput> {
                    width: 520, height: 120
                    empty_text: "Type your text here..."
                    draw_bg: {
                        color: #ffffff
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, 10.0);
                            sdf.fill(self.color);
                            sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, 10.0);
                            sdf.stroke(#d1d5db, 1.0);
                            return sdf.result;
                        }
                    }
                    draw_text: {
                        color: #1f2937
                        text_style: <FONT_REGULAR>{ font_size: 14.0 }
                        wrap: Word
                    }
                }

                // Generate button
                tts_generate_btn = <View> {
                    width: Fit, height: 44
                    cursor: Hand
                    align: {x: 0.5, y: 0.5}
                    padding: {left: 28, right: 28}
                    show_bg: true
                    draw_bg: {
                        instance hover: 0.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.box(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0, 10.0);
                            sdf.fill(mix(#f59e0b, #d97706, self.hover));
                            return sdf.result;
                        }
                    }
                    animator: {
                        hover = {
                            default: off
                            off = { from: {all: Forward{duration: 0.15}}, apply: {draw_bg: {hover: 0.0}} }
                            on  = { from: {all: Forward{duration: 0.15}}, apply: {draw_bg: {hover: 1.0}} }
                        }
                    }
                    <Label> {
                        text: "Generate Speech"
                        draw_text: { color: #ffffff, text_style: <FONT_SEMIBOLD>{ font_size: 14.0 } }
                    }
                }

                // Status label
                tts_status_label = <View> {
                    width: Fit, height: Fit
                    visible: false
                    label = <Label> {
                        width: Fit, height: Fit
                        text: ""
                        draw_text: { color: #6b7280, text_style: <FONT_REGULAR>{ font_size: 13.0 } }
                    }
                }
            }

            // ── Image Gen welcome overlay ──────────────────────────────────
            image_welcome_overlay = <View> {
                width: Fill, height: Fill
                flow: Down
                align: {x: 0.5, y: 0.25}
                spacing: 20
                padding: {left: 64, right: 64}
                visible: false

                <Label> {
                    width: Fit, height: Fit
                    text: "Image Generation"
                    draw_text: { color: #1f2937, text_style: <FONT_SEMIBOLD>{ font_size: 28.0 } }
                }
                <Label> {
                    width: Fit, height: Fit
                    text: "Describe the image you want to create"
                    draw_text: { color: #6b7280, text_style: <FONT_REGULAR>{ font_size: 14.0 } }
                }

                // Prompt input
                image_prompt_input = <TextInput> {
                    width: 520, height: 80
                    empty_text: "A photorealistic landscape of..."
                    draw_bg: {
                        color: #ffffff
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, 10.0);
                            sdf.fill(self.color);
                            sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, 10.0);
                            sdf.stroke(#d1d5db, 1.0);
                            return sdf.result;
                        }
                    }
                    draw_text: {
                        color: #1f2937
                        text_style: <FONT_REGULAR>{ font_size: 14.0 }
                        wrap: Word
                    }
                }

                // Negative prompt (optional)
                image_neg_prompt_input = <TextInput> {
                    width: 520, height: 48
                    empty_text: "Negative prompt (optional)"
                    draw_bg: {
                        color: #ffffff
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, 10.0);
                            sdf.fill(self.color);
                            sdf.box(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0, 10.0);
                            sdf.stroke(#d1d5db, 1.0);
                            return sdf.result;
                        }
                    }
                    draw_text: {
                        color: #1f2937
                        text_style: <FONT_REGULAR>{ font_size: 13.0 }
                        wrap: Word
                    }
                }

                // Generate button
                image_generate_btn = <View> {
                    width: Fit, height: 44
                    cursor: Hand
                    align: {x: 0.5, y: 0.5}
                    padding: {left: 28, right: 28}
                    show_bg: true
                    draw_bg: {
                        instance hover: 0.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.box(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0, 10.0);
                            sdf.fill(mix(#8b5cf6, #7c3aed, self.hover));
                            return sdf.result;
                        }
                    }
                    animator: {
                        hover = {
                            default: off
                            off = { from: {all: Forward{duration: 0.15}}, apply: {draw_bg: {hover: 0.0}} }
                            on  = { from: {all: Forward{duration: 0.15}}, apply: {draw_bg: {hover: 1.0}} }
                        }
                    }
                    <Label> {
                        text: "Generate Image"
                        draw_text: { color: #ffffff, text_style: <FONT_SEMIBOLD>{ font_size: 14.0 } }
                    }
                }

                // Status label
                image_status_label = <View> {
                    width: Fit, height: Fit
                    visible: false
                    label = <Label> {
                        width: Fit, height: Fit
                        text: ""
                        draw_text: { color: #6b7280, text_style: <FONT_REGULAR>{ font_size: 13.0 } }
                    }
                }

                // Result image display
                image_result = <Image> {
                    width: 512, height: 512
                    visible: false
                }
            }
        }
    }
}
