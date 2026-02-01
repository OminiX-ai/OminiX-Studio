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

    // Delete icon for chat history
    ICON_TRASH = dep("crate://self/resources/icons/trash.svg")

    // Individual chat history item - Widget with proper event handling
    pub ChatHistoryItem = {{ChatHistoryItem}} {
        width: Fill, height: Fit
        padding: {left: 12, right: 8, top: 8, bottom: 8}
        cursor: Hand
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance selected: 0.0
            instance hover: 0.0
            instance down: 0.0
            fn pixel(self) -> vec4 {
                let base = mix(#ffffff, #1e293b, self.dark_mode);
                let selected_color = mix(#dbeafe, #1e3a8a, self.dark_mode);
                let hover_color = mix(#f1f5f9, #334155, self.dark_mode);
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
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#1f2937, #f1f5f9, self.dark_mode);
                    }
                    text_style: { font_size: 12.0 }
                    wrap: Ellipsis
                }
                text: "New Chat"
            }

            date_label = <Label> {
                width: Fill
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#6b7280, #9ca3af, self.dark_mode);
                    }
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
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 {
                    let hover_color = mix(#fee2e2, #7f1d1d, self.dark_mode);
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
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#9ca3af, #6b7280, self.dark_mode);
                    }
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
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix(#f8fafc, #0f172a, self.dark_mode);
            }
        }

        // New chat button
        new_chat_container = <View> {
            width: Fill, height: Fit
            padding: 12

            new_chat_button = <Button> {
                width: Fill, height: Fit
                padding: {left: 12, right: 12, top: 10, bottom: 10}
                text: "+ New Chat"
                draw_text: {
                    text_style: { font_size: 12.0 }
                    color: #ffffff
                }
                draw_bg: {
                    instance dark_mode: 0.0
                    instance hover: 0.0
                    instance pressed: 0.0
                    fn pixel(self) -> vec4 {
                        let base = mix(#3b82f6, #2055ff, self.dark_mode);
                        let hover_color = mix(#2055ff, #1045cc, self.dark_mode);
                        let pressed_color = mix(#1045cc, #1040a0, self.dark_mode);
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
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#6b7280, #9ca3af, self.dark_mode);
                    }
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
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix(#f5f7fa, #0f172a, self.dark_mode);
            }
        }

        // Provider icons for model selector and chat messages
        // Order: openai, anthropic, gemini, ollama, deepseek, openrouter, siliconflow, nvidia, groq
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
        ]

        // Header with provider status
        header = <View> {
            width: Fill, height: Fit
            flow: Down
            padding: 16
            spacing: 4

            title_label = <Label> {
                text: "Chat"
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#1f2937, #f1f5f9, self.dark_mode);
                    }
                    text_style: <FONT_SEMIBOLD>{ font_size: 20.0 }
                }
            }

            status_label = <Label> {
                text: "No provider configured - Go to Settings to add an API key"
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#f59e0b, #fbbf24, self.dark_mode);
                    }
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
                visible: true

                // Greeting text
                greeting_label = <Label> {
                    width: Fit, height: Fit
                    text: "What can I help you with?"
                    draw_text: {
                        instance dark_mode: 0.0
                        fn get_color(self) -> vec4 {
                            return mix(#1f2937, #f1f5f9, self.dark_mode);
                        }
                        text_style: <FONT_SEMIBOLD>{ font_size: 28.0 }
                    }
                }

                // Centered PromptInput with model selector
                welcome_prompt = <PromptInput> {
                    width: 700, height: Fit
                }
            }
        }
    }
}
