use makepad_widgets::*;

use moly_data::{ChatId, Store, StoreAction, ModelRegistry, RegistryCategory, ModelRuntimeClient, ensure_server_running};
use std::sync::mpsc;
use std::path::Path;
use moly_kit::a2ui::{A2uiSurface, A2uiSurfaceAction};
use moly_kit::widgets::chat::ChatAction;
use moly_kit::widgets::prompt_input::PromptInputAction;
use moly_kit::widgets::take_pending_a2ui_json;
use moly_widgets::{MolyApp, MolyAppData};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use moly_widgets::theme::*;
    use moly_widgets::components::*;
    use moly_kit::a2ui::surface::*;

    // Import app widgets from external app crates
    use moly_chat::screen::design::*;
    use moly_settings::screen::design::*;
    use moly_mcp::screen::design::*;
    use moly_hub::screen::design::*;

    // Icon dependencies
    ICON_HAMBURGER = dep("crate://self/resources/icons/hamburger.svg")
    ICON_MOON = dep("crate://self/resources/icons/moon.svg")
    ICON_CHAT = dep("crate://self/resources/icons/chat.svg")
    ICON_SETTINGS = dep("crate://self/resources/icons/settings.svg")
    ICON_HUB = dep("crate://self/resources/icons/hub.svg")
    ICON_LLM = dep("crate://self/resources/icons/llm.svg")
    ICON_VLM = dep("crate://self/resources/icons/vlm.svg")
    ICON_ASR = dep("crate://self/resources/icons/asr.svg")
    ICON_TTS = dep("crate://self/resources/icons/tts.svg")
    ICON_IMAGE = dep("crate://self/resources/icons/image.svg")
    ICON_NEW_CHAT = dep("crate://self/resources/icons/new-chat.svg")
    ICON_TRASH = dep("crate://self/resources/icons/trash.svg")

    // Logo (light and dark variants)
    IMG_LOGO = dep("crate://self/resources/ominix-studio-logo.png")

    // Provider icons - registered globally so they can be loaded by moly-kit
    ICON_PROVIDER_OPENAI = dep("crate://self/resources/providers/openai.png")
    ICON_PROVIDER_ANTHROPIC = dep("crate://self/resources/providers/anthropic.png")
    ICON_PROVIDER_GEMINI = dep("crate://self/resources/providers/gemini.png")
    ICON_PROVIDER_OLLAMA = dep("crate://self/resources/providers/ollama.png")
    ICON_PROVIDER_DEEPSEEK = dep("crate://self/resources/providers/deepseek.png")
    ICON_PROVIDER_OPENROUTER = dep("crate://self/resources/providers/openrouter.png")
    ICON_PROVIDER_SILICONFLOW = dep("crate://self/resources/providers/siliconflow.png")

    // Reusable chat tile for chat history grid
    ChatTile = <RoundedView> {
        width: Fill, height: 144
        show_bg: true
        draw_bg: {
            border_radius: 12.0
            color: (PANEL_BG)
        }
        flow: Down
        padding: {top: 16, left: 16, right: 16, bottom: 16}
        cursor: Hand
        visible: false
        header = <View> {
            width: Fill, height: Fit
            flow: Right
            align: {y: 0.0}
            title = <Label> {
                width: Fill
                draw_text: { color: (TEXT_PRIMARY), text_style: <FONT_SEMIBOLD>{ font_size: 11.0 }, wrap: Ellipsis }
            }
            delete_btn = <View> {
                width: 28, height: 28
                align: {x: 0.5, y: 0.5}
                cursor: Hand
                <Icon> { draw_icon: { svg_file: (ICON_TRASH), color: (TEXT_MUTED) }, icon_walk: {width: 18, height: 18} }
            }
        }
        <View> { width: Fill, height: Fill }
        date_label = <Label> { draw_text: { color: (TEXT_MUTED), text_style: { font_size: 10.0 } } }
    }

    // Row of 4 chat tiles for grid layout
    TileRow = <View> {
        width: Fill, height: Fit
        flow: Right
        spacing: 20
        visible: false
        tile_0 = <ChatTile> {}
        tile_1 = <ChatTile> {}
        tile_2 = <ChatTile> {}
        tile_3 = <ChatTile> {}
    }

    // Sidebar button using Button directly (like mofa-studio SidebarMenuButton)
    // Button natively supports icon + text with draw_icon and draw_text
    // Note: Button's draw_bg/draw_text/draw_icon don't support custom instance variables,
    // so we use fixed colors for light mode. Theme switching can be done by swapping button styles.
    SidebarButton = <Button> {
        width: Fill, height: Fit
        padding: {top: 12, bottom: 12, left: 12, right: 12}
        margin: {bottom: 4}
        align: {x: 0.0, y: 0.5}
        icon_walk: {width: 24, height: 24, margin: {right: 12}}

        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.15}}
                    apply: {
                        draw_bg: {hover: 0.0}
                        draw_text: {color: (TEXT_PRIMARY)}
                        draw_icon: {color: (GRAY_600)}
                    }
                }
                on = {
                    from: {all: Forward {duration: 0.15}}
                    apply: {
                        draw_bg: {hover: 1.0}
                        draw_text: {color: (TEXT_PRIMARY)}
                        draw_icon: {color: (GRAY_600)}
                    }
                }
            }
            pressed = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: { draw_bg: {pressed: 0.0} }
                }
                on = {
                    from: {all: Forward {duration: 0.1}}
                    apply: { draw_bg: {pressed: 1.0} }
                }
            }
        }

        draw_bg: {
            instance hover: 0.0
            instance pressed: 0.0
            instance selected: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let normal = (PANEL_BG);
                let hover_color = (HOVER_BG);
                let selected_color = (INDIGO_50);
                let color = mix(
                    mix(normal, hover_color, self.hover),
                    selected_color,
                    self.selected
                );
                sdf.box(2.0, 2.0, self.rect_size.x - 4.0, self.rect_size.y - 4.0, 6.0);
                sdf.fill(color);
                return sdf.result;
            }
        }

        draw_text: {
            text_style: <FONT_MEDIUM>{ font_size: 13.0 }
            color: (TEXT_PRIMARY)
        }

        draw_icon: {
            color: (GRAY_600)
        }
    }

    // Accent CTA button for "New Chat" — blue background, white text/icon
    NewChatButton = <Button> {
        width: Fill, height: Fit
        padding: {top: 11, bottom: 11, left: 16, right: 16}
        margin: {bottom: 16}
        align: {x: 0.5, y: 0.5}
        icon_walk: {width: 18, height: 18, margin: {right: 8}}

        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.15}}
                    apply: { draw_bg: {hover: 0.0} }
                }
                on = {
                    from: {all: Forward {duration: 0.15}}
                    apply: { draw_bg: {hover: 1.0} }
                }
            }
            pressed = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: { draw_bg: {pressed: 0.0} }
                }
                on = {
                    from: {all: Forward {duration: 0.1}}
                    apply: { draw_bg: {pressed: 1.0} }
                }
            }
        }

        draw_bg: {
            instance hover: 0.0
            instance pressed: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let base = #16a39c;
                let hover_color = #128c86;
                let pressed_color = #0f7a75;
                let color = mix(
                    mix(base, hover_color, self.hover),
                    pressed_color,
                    self.pressed
                );
                sdf.box(2.0, 2.0, self.rect_size.x - 4.0, self.rect_size.y - 4.0, 8.0);
                sdf.fill(color);
                return sdf.result;
            }
        }

        draw_text: {
            text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
            color: #ffffff
        }

        draw_icon: {
            fn get_color(self) -> vec4 {
                return #ffffff;
            }
        }
    }

    // Small uppercase section header label for sidebar groups
    SidebarSectionLabel = <Label> {
        width: Fill, height: Fit
        margin: {top: 8, bottom: 2, left: 12, right: 8}
        draw_text: {
            color: (TEXT_MUTED)
            text_style: <FONT_MEDIUM>{ font_size: 10.0 }
        }
    }

    // Slot item in the model-selector dropdown
    ModelDropdownSlot = <View> {
        width: Fill, height: 52
        cursor: Hand
        visible: false
        flow: Right
        align: {y: 0.5}
        padding: {left: 16, right: 16}
        spacing: 10
        show_bg: true
        draw_bg: {
            instance hover: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 0.0);
                sdf.fill(mix(#ffffff, #f9fafb, self.hover));
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

        // Category color dot
        category_dot = <View> {
            width: 8, height: 8
            show_bg: true
            draw_bg: {
                instance cat: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.circle(4.0, 4.0, 3.5);
                    let llm_c = vec4(0.388, 0.400, 0.945, 1.0);
                    let vlm_c = vec4(0.545, 0.361, 0.965, 1.0);
                    let asr_c = vec4(0.063, 0.725, 0.506, 1.0);
                    let tts_c = vec4(0.961, 0.620, 0.043, 1.0);
                    let img_c = vec4(0.925, 0.286, 0.600, 1.0);
                    let c = mix(mix(mix(mix(
                        llm_c,
                        vlm_c, step(0.5, self.cat)),
                        asr_c, step(1.5, self.cat)),
                        tts_c, step(2.5, self.cat)),
                        img_c, step(3.5, self.cat));
                    sdf.fill(c);
                    return sdf.result;
                }
            }
        }

        // Model name
        slot_name = <Label> {
            width: Fill
            draw_text: {
                color: #1f2937
                text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                wrap: Ellipsis
            }
        }

        // Size · Category tag on the right
        slot_meta = <Label> {
            draw_text: {
                color: #9ca3af
                text_style: { font_size: 11.0 }
            }
        }

        // Green dot when loaded
        slot_loaded_dot = <View> {
            width: 8, height: 8
            show_bg: true
            draw_bg: {
                instance loaded: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.circle(4.0, 4.0, 3.5);
                    sdf.fill(mix(#e5e7eb, #22c55e, self.loaded));
                    return sdf.result;
                }
            }
        }
    }

    App = {{App}} {
        ui: <Window> {
            window: { title: "OminiX Studio", inner_size: vec2(1400, 900) }
            pass: {
                clear_color: #f5f7fa
            }

            body = <View> {
                width: Fill, height: Fill
                flow: Overlay
                show_bg: true
                draw_bg: {
                    color: #f5f7fa
                }

                // ── Normal app layout (header + sidebar + content) ──────────
                body_layout = <View> {
                    width: Fill, height: Fill
                    flow: Down

                // Header
                header = <View> {
                    width: Fill, height: 72
                    flow: Right
                    align: {y: 0.5}
                    padding: {left: 20, right: 20, top: 16}
                    show_bg: true
                    draw_bg: {
                        color: #ffffff
                    }

                    // Hamburger menu button
                    hamburger_btn = <View> {
                        width: 40, height: Fit
                        margin: {right: 12}
                        align: {x: 0.5, y: 0.5}
                        cursor: Hand
                        event_order: Down
                        show_bg: false

                        hamburger_icon = <Icon> {
                            draw_icon: {
                                svg_file: (ICON_HAMBURGER)
                                color: #6b7280
                            }
                            icon_walk: {width: 20, height: 20}
                        }
                    }

                    logo_light = <Image> {
                        source: (IMG_LOGO)
                        width: 280, height: 44
                    }

                    title_label = <Label> {
                        text: ""
                        width: 0
                    }

                    <View> { width: Fill } // Left spacer

                    // ── Model Selector pill (center of header, like LM Studio) ──
                    model_selector_btn = <View> {
                        width: Fit, height: 36
                        cursor: Hand
                        align: {x: 0.5, y: 0.5}
                        padding: {left: 16, right: 12, top: 0, bottom: 0}
                        margin: {right: 4}
                        show_bg: true
                        draw_bg: {
                            instance hover: 0.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0, 8.0);
                                sdf.fill(mix(#f3f4f6, #e5e7eb, self.hover));
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
                        flow: Right
                        spacing: 8

                        selector_dot = <View> {
                            width: 8, height: 8
                            show_bg: true
                            draw_bg: {
                                instance loaded: 0.0
                                fn pixel(self) -> vec4 {
                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                    sdf.circle(4.0, 4.0, 3.5);
                                    sdf.fill(mix(#9ca3af, #22c55e, self.loaded));
                                    return sdf.result;
                                }
                            }
                        }

                        // Category type badge — hidden until a model is loaded
                        category_tag = <View> {
                            width: Fit, height: 20
                            visible: false
                            padding: {left: 6, right: 6}
                            align: {x: 0.5, y: 0.5}
                            show_bg: true
                            draw_bg: {
                                instance cat: 0.0
                                fn pixel(self) -> vec4 {
                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                    sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 4.0);
                                    // LLM indigo-50, VLM violet-50, ASR green-50, TTS amber-50, Image pink-50
                                    let c0 = #dbeafe;
                                    let c1 = #ede9fe;
                                    let c2 = #d1fae5;
                                    let c3 = #fef3c7;
                                    let c4 = #fce7f3;
                                    let w0 = 1.0 - step(0.5, self.cat);
                                    let w1 = step(0.5, self.cat) * (1.0 - step(1.5, self.cat));
                                    let w2 = step(1.5, self.cat) * (1.0 - step(2.5, self.cat));
                                    let w3 = step(2.5, self.cat) * (1.0 - step(3.5, self.cat));
                                    let w4 = step(3.5, self.cat);
                                    sdf.fill(c0 * w0 + c1 * w1 + c2 * w2 + c3 * w3 + c4 * w4);
                                    return sdf.result;
                                }
                            }
                            category_tag_label = <Label> {
                                text: "LLM"
                                draw_text: {
                                    instance cat: 0.0
                                    fn get_color(self) -> vec4 {
                                        let c0 = #1a40af;
                                        let c1 = #5b21b6;
                                        let c2 = #047857;
                                        let c3 = #92400f;
                                        let c4 = #9d174d;
                                        let w0 = 1.0 - step(0.5, self.cat);
                                        let w1 = step(0.5, self.cat) * (1.0 - step(1.5, self.cat));
                                        let w2 = step(1.5, self.cat) * (1.0 - step(2.5, self.cat));
                                        let w3 = step(2.5, self.cat) * (1.0 - step(3.5, self.cat));
                                        let w4 = step(3.5, self.cat);
                                        return c0 * w0 + c1 * w1 + c2 * w2 + c3 * w3 + c4 * w4;
                                    }
                                    text_style: <FONT_SEMIBOLD>{ font_size: 9.5 }
                                }
                            }
                        }

                        selector_label = <Label> {
                            text: "Select a model to load"
                            draw_text: {
                                color: #374151
                                text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                            }
                        }

                        <Label> {
                            text: "▾"
                            margin: {left: 2}
                            draw_text: {
                                color: #6b7280
                                text_style: { font_size: 11.0 }
                            }
                        }
                    }

                    // Eject / unload button (visible only when a model is loaded)
                    eject_btn = <View> {
                        width: 32, height: 32
                        cursor: Hand
                        visible: false
                        align: {x: 0.5, y: 0.5}
                        margin: {right: 8}
                        show_bg: true
                        draw_bg: {
                            instance hover: 0.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0, 6.0);
                                sdf.fill(mix(#f9fafb, #fee2e2, self.hover));
                                return sdf.result;
                            }
                        }
                        animator: {
                            hover = {
                                default: off
                                off = { from: {all: Forward{duration: 0.1}}, apply: {draw_bg: {hover: 0.0}} }
                                on  = { from: {all: Forward{duration: 0.1}}, apply: {draw_bg: {hover: 1.0}} }
                            }
                        }
                        <Label> {
                            text: "⏏"
                            draw_text: {
                                color: #6b7280
                                text_style: { font_size: 15.0 }
                            }
                        }
                    }

                    <View> { width: Fill } // Right spacer
                }

                // Content area
                content = <View> {
                    width: Fill, height: Fill
                    flow: Right

                    // Sidebar
                    sidebar = <View> {
                        width: 250, height: Fill
                        show_bg: true
                        draw_bg: {
                            color: #ffffff
                        }
                        flow: Down, padding: {top: 16, bottom: 16, left: 8, right: 8}

                        // New Chat - primary CTA button (accent blue)
                        new_chat_btn = <NewChatButton> {
                            text: "New Chat"
                            draw_icon: { svg_file: (ICON_NEW_CHAT) }
                        }

                        // CHAT section
                        chat_section_label = <View> {
                            width: Fill, height: Fit
                            padding: {top: 8, bottom: 2, left: 12, right: 8}
                            <Label> {
                                text: "CHAT"
                                draw_text: { color: (TEXT_MUTED), text_style: <FONT_MEDIUM>{ font_size: 10.0 } }
                            }
                        }

                        chat_section = <View> {
                            width: Fill, height: Fit
                            flow: Down
                            margin: {bottom: 8}

                            chat_history_btn = <SidebarButton> {
                                text: "Chat History"
                                draw_icon: { svg_file: (ICON_CHAT) }
                            }

                            // Chat history sublist (collapsible, visible when sidebar expanded)
                            chat_history_visible = <View> {
                                width: Fill, height: Fit
                                flow: Down
                                padding: {left: 32}

                                chat_item_0 = <ChatListItem> {}
                                chat_item_1 = <ChatListItem> {}
                                chat_item_2 = <ChatListItem> {}

                                // Show More button
                                show_more_btn = <View> {
                                    width: Fill, height: 28
                                    padding: {left: 8, right: 8}
                                    align: {y: 0.5}
                                    flow: Right
                                    cursor: Hand
                                    show_bg: true
                                    draw_bg: {
                                        instance hover: 0.0
                                        fn pixel(self) -> vec4 {
                                            let base = (PANEL_BG);
                                            let hover_color = (HOVER_BG);
                                            return mix(base, hover_color, self.hover);
                                        }
                                    }
                                    show_more_label = <Label> {
                                        width: Fill
                                        text: "Show More"
                                        draw_text: {
                                            color: (TEXT_SECONDARY)
                                            text_style: { font_size: 11.0 }
                                        }
                                    }
                                    show_more_arrow = <Label> {
                                        text: ">"
                                        draw_text: {
                                            color: (TEXT_SECONDARY)
                                            text_style: { font_size: 11.0 }
                                        }
                                    }
                                }
                            }

                            // Extra chat history items (hidden by default, shown via Show More)
                            chat_history_more = <View> {
                                width: Fill, height: Fit
                                flow: Down
                                padding: {left: 32}
                                visible: false

                                chat_item_3 = <ChatListItem> { visible: false }
                                chat_item_4 = <ChatListItem> { visible: false }
                                chat_item_5 = <ChatListItem> { visible: false }
                            }
                        }

                        // MODELS section
                        models_section_label = <View> {
                            width: Fill, height: Fit
                            padding: {top: 8, bottom: 2, left: 12, right: 8}
                            <Label> {
                                text: "MODELS"
                                draw_text: { color: (TEXT_MUTED), text_style: <FONT_MEDIUM>{ font_size: 10.0 } }
                            }
                        }

                        llm_btn   = <SidebarButton> { text: "LLM",   draw_icon: { svg_file: (ICON_LLM) } }
                        vlm_btn   = <SidebarButton> { text: "VLM",   draw_icon: { svg_file: (ICON_VLM) } }
                        asr_btn   = <SidebarButton> { text: "ASR",   draw_icon: { svg_file: (ICON_ASR) } }
                        tts_btn   = <SidebarButton> { text: "TTS",   draw_icon: { svg_file: (ICON_TTS) } }
                        image_btn = <SidebarButton> { text: "Image", draw_icon: { svg_file: (ICON_IMAGE) } }

                        // Spacer to push Settings to bottom
                        <View> { width: Fill, height: Fill }

                        settings_btn = <SidebarButton> {
                            text: "Settings"
                            draw_icon: { svg_file: (ICON_SETTINGS) }
                        }
                    }

                    // Main content - app container
                    main_content = <View> {
                        width: Fill, height: Fill
                        flow: Overlay

                        // Chat History page (shown when clicking Chat icon)
                        chat_history_page = <View> {
                            width: Fill, height: Fill
                            flow: Down
                            visible: false
                            show_bg: true
                            draw_bg: {
                                color: #f5f7fa
                            }
                            padding: {top: 40, left: 48, right: 48, bottom: 32}

                            // Header with title
                            <View> {
                                width: Fill, height: Fit
                                margin: {bottom: 32}
                                align: {x: 0.5}
                                history_title = <Label> {
                                    text: "Chat History"
                                    draw_text: {
                                        color: #1f2937
                                        text_style: <FONT_SEMIBOLD>{ font_size: 28.0 }
                                    }
                                }
                            }

                            // Search bar container
                            <View> {
                                width: Fill, height: Fit
                                align: {x: 0.5}
                                margin: {bottom: 40}

                                search_container = <RoundedView> {
                                    width: 500, height: 48
                                    show_bg: true
                                    draw_bg: {
                                        border_radius: 12.0
                                        color: #e5e7eb
                                    }
                                    padding: {left: 20, right: 20}
                                    align: {y: 0.5}
                                    flow: Right

                                    // Search icon
                                    <Icon> {
                                        draw_icon: {
                                            svg_file: (ICON_CHAT)
                                            color: #6b7280
                                        }
                                        icon_walk: {width: 20, height: 20, margin: {right: 12}}
                                    }

                                    // Search input
                                    search_input = <TextInput> {
                                        width: Fill, height: 32
                                        empty_text: "Search chats..."
                                        draw_text: {
                                            color: #1f2937
                                            color_focus: #1f2937
                                            color_empty: #6b7280
                                            color_empty_focus: #6b7280
                                            text_style: { font_size: 14.0 }
                                        }
                                        draw_selection: {
                                            color: #bfdbfe
                                            color_focus: #bfdbfe
                                        }
                                        draw_cursor: {
                                            color: #1f2937
                                        }
                                        draw_bg: {
                                            fn pixel(self) -> vec4 {
                                                return vec4(0.0, 0.0, 0.0, 0.0);
                                            }
                                        }
                                    }
                                }
                            }

                            // Empty state (shown when no chats)
                            empty_state = <View> {
                                width: Fill, height: Fill
                                align: {x: 0.5, y: 0.3}
                                visible: true
                                empty_label = <Label> {
                                    text: "No chat history yet. Click 'New Chat' to start."
                                    draw_text: {
                                        color: #6b7280
                                        text_style: { font_size: 16.0 }
                                    }
                                }
                            }

                            // Chat tiles mosaic grid (scrollable)
                            chat_tiles_scroll = <ScrollYView> {
                                width: Fill, height: Fill
                                visible: false

                                chat_tiles_container = <View> {
                                    width: Fill, height: Fit
                                    flow: Down
                                    spacing: 20

                                    tile_row_0 = <TileRow> {}
                                    tile_row_1 = <TileRow> {}
                                    tile_row_2 = <TileRow> {}
                                }
                            }
                        }

                        // Chat with canvas panel (horizontal layout)
                        chat_with_canvas = <View> {
                            width: Fill, height: Fill
                            flow: Right
                            visible: true

                            // Left: Chat app (fills remaining space)
                            chat_app = <ChatApp> {
                                width: Fill, height: Fill
                            }

                            // Middle: Splitter (resizable divider)
                            canvas_splitter = <View> {
                                width: 0, height: Fill  // 0 when collapsed, 16 when expanded
                                cursor: ColResize
                                show_bg: true
                                draw_bg: {
                                    fn pixel(self) -> vec4 {
                                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                        // Draw thin line in center
                                        sdf.rect(7.0, 16.0, 2.0, self.rect_size.y - 32.0);
                                        sdf.fill(#d1d5db);
                                        return sdf.result;
                                    }
                                }
                            }

                            // Right: Canvas panel (collapsed by default, opens when A2UI is enabled)
                            canvas_section = <View> {
                                width: 500, height: Fill
                                flow: Right
                                visible: false

                                // Collapse strip — always visible when canvas is expanded
                                canvas_toggle_column = <View> {
                                    visible: true
                                    width: 20, height: Fill
                                    cursor: Hand
                                    show_bg: true
                                    draw_bg: { color: #f8fafc }
                                    align: {x: 0.5, y: 0.5}
                                    <Label> {
                                        text: "›"
                                        draw_text: {
                                            color: #9ca3af
                                            text_style: { font_size: 18.0 }
                                        }
                                    }
                                }

                                // Content column
                                canvas_content = <RoundedView> {
                                    width: Fill, height: Fill
                                    visible: true
                                    draw_bg: {
                                        color: #ffffff
                                        border_radius: 8.0
                                    }
                                    flow: Down

                                    // Header
                                    canvas_header = <View> {
                                        width: Fill, height: Fit
                                        padding: {left: 16, right: 16, top: 12, bottom: 12}
                                        show_bg: true
                                        draw_bg: { color: #f8fafc }

                                        canvas_title = <Label> {
                                            text: "Canvas"
                                            draw_text: {
                                                color: #1f2937
                                                text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                                            }
                                        }
                                    }

                                    // Canvas area with A2UI Surface
                                    canvas_area = <ScrollYView> {
                                        width: Fill, height: Fill
                                        padding: 12

                                        a2ui_surface = <A2uiSurface> {
                                            width: Fill
                                            height: Fit
                                        }
                                    }
                                }
                            }

                            // Reopen strip — visible when canvas is collapsed
                            canvas_reopen_btn = <View> {
                                width: 20, height: Fill
                                visible: true
                                cursor: Hand
                                show_bg: true
                                draw_bg: { color: #f8fafc }
                                align: {x: 0.5, y: 0.5}
                                <Label> {
                                    text: "‹"
                                    draw_text: {
                                        color: #9ca3af
                                        text_style: { font_size: 18.0 }
                                    }
                                }
                            }
                        }

                        // Settings app
                        settings_app = <SettingsApp> {
                            visible: false
                        }

                        // Per-category Model Hub instances
                        llm_hub_app = <ModelHubApp> {
                            hub_category: 1.0
                            visible: false
                        }
                        vlm_hub_app = <ModelHubApp> {
                            hub_category: 2.0
                            visible: false
                        }
                        asr_hub_app = <ModelHubApp> {
                            hub_category: 3.0
                            visible: false
                        }
                        tts_hub_app = <ModelHubApp> {
                            hub_category: 4.0
                            visible: false
                        }
                        image_hub_app = <ModelHubApp> {
                            hub_category: 5.0
                            visible: false
                        }

                        // MCP app (desktop only)
                        mcp_app = <McpApp> {
                            visible: false
                        }
                    }
                }
                } // closes body_layout

                // ── Model-selector dropdown overlay ─────────────────────────
                model_selector_dropdown = <View> {
                    abs_pos: vec2(0.0, 0.0)
                    width: Fill, height: Fill
                    flow: Down
                    visible: false

                    // Transparent spacer equal to header height
                    <View> { width: Fill, height: 72 }

                    // Centered dropdown panel row
                    dropdown_wrapper = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        align: {x: 0.5}

                        dropdown_panel = <RoundedView> {
                            width: 440, height: Fit
                            show_bg: true
                            draw_bg: {
                                color: #ffffff
                                border_radius: 12.0
                            }
                            flow: Down

                            // Panel header row
                            dropdown_header = <View> {
                                width: Fill, height: 48
                                flow: Right
                                align: {y: 0.5}
                                padding: {left: 16, right: 16}

                                <Label> {
                                    width: Fill
                                    text: "On-Device Models"
                                    draw_text: {
                                        color: #1f2937
                                        text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                                    }
                                }
                                dropdown_status_label = <Label> {
                                    text: ""
                                    draw_text: {
                                        color: #9ca3af
                                        text_style: { font_size: 11.0 }
                                    }
                                }
                            }

                            <View> { width: Fill, height: 1, show_bg: true, draw_bg: { color: #e5e7eb } }

                            // Empty state
                            empty_state = <View> {
                                width: Fill, height: 80
                                visible: true
                                align: {x: 0.5, y: 0.5}
                                <Label> {
                                    text: "No models downloaded. Visit the Model Hub."
                                    draw_text: { color: #6b7280, text_style: { font_size: 12.0 } }
                                }
                            }

                            // Scrollable model list (hidden when empty)
                            model_scroll = <ScrollYView> {
                                width: Fill, height: Fit
                                flow: Down
                                visible: false

                                slot_0 = <ModelDropdownSlot> {}
                                slot_1 = <ModelDropdownSlot> {}
                                slot_2 = <ModelDropdownSlot> {}
                                slot_3 = <ModelDropdownSlot> {}
                                slot_4 = <ModelDropdownSlot> {}
                                slot_5 = <ModelDropdownSlot> {}
                                slot_6 = <ModelDropdownSlot> {}
                                slot_7 = <ModelDropdownSlot> {}
                                slot_8 = <ModelDropdownSlot> {}
                                slot_9 = <ModelDropdownSlot> {}
                            }
                        }
                    }

                    // Click-anywhere-outside to dismiss
                    dismiss_area = <View> {
                        width: Fill, height: Fill
                        cursor: Arrow
                    }
                }
            }
        }
    }
}

// ── Model selector types ──────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Default, Debug)]
enum ShellModelLoadState {
    #[default]
    Unloaded,
    Loading,
    Loaded,
    Error,
}

#[derive(Clone, Debug)]
struct DownloadedModelEntry {
    registry_id:   String,
    name:          String,
    api_model_id:  String,
    category:      RegistryCategory,
    model_type_str: &'static str,
    size_display:  String,
}

fn category_to_model_type(cat: RegistryCategory) -> &'static str {
    match cat {
        RegistryCategory::Llm      => "llm",
        RegistryCategory::Vlm      => "vlm",
        RegistryCategory::Asr      => "asr",
        RegistryCategory::Tts      => "tts",
        RegistryCategory::ImageGen => "image",
        RegistryCategory::VideoGen => "video",
    }
}

fn registry_category_as_f64(cat: RegistryCategory) -> f64 {
    match cat {
        RegistryCategory::Llm      => 0.0,
        RegistryCategory::Vlm      => 1.0,
        RegistryCategory::Asr      => 2.0,
        RegistryCategory::Tts      => 3.0,
        RegistryCategory::ImageGen => 4.0,
        RegistryCategory::VideoGen => 5.0,
    }
}

/// Check if a registry model's files are present on disk.
fn shell_is_model_downloaded(model: &moly_data::RegistryModel) -> bool {
    let expanded = model.storage.expanded_path();
    let path = Path::new(&expanded);
    if !path.exists() { return false; }
    if model.storage.size_bytes > 100 * 1024 * 1024 {
        return has_weight_files_shell(path);
    }
    std::fs::read_dir(path)
        .map(|e| e.filter_map(|x| x.ok())
             .filter(|x| !x.file_name().to_string_lossy().starts_with('.')).count())
        .unwrap_or(0) > 0
}

fn has_weight_files_shell(dir: &Path) -> bool {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for e in entries.flatten() {
            let n = e.file_name();
            let name = n.to_string_lossy();
            if name.ends_with(".safetensors") || name.ends_with(".bin") { return true; }
            if e.path().is_dir() && has_weight_files_shell(&e.path()) { return true; }
        }
    }
    false
}

// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Default)]
enum NavigationTarget {
    /// Chat History page - blank page with "Chat History" text
    #[default]
    ChatHistory,
    /// Active chat - shows the chat interface
    ActiveChat,
    Settings,
    LlmHub,
    VlmHub,
    AsrHub,
    TtsHub,
    ImageHub,
}

#[derive(Live)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    store: Store,
    #[rust]
    app_data: MolyAppData,
    #[rust]
    current_view: NavigationTarget,
    #[rust]
    initialized: bool,
    /// Whether the chat history "Show More" section is expanded
    #[rust]
    chat_history_expanded: bool,
    /// Chat IDs displayed in the tiles (max 12)
    #[rust]
    displayed_chat_ids: Vec<ChatId>,
    /// Current search query for filtering chat history
    #[rust]
    search_query: String,
    /// Whether the canvas panel is collapsed
    #[rust]
    canvas_panel_collapsed: bool,
    /// Width of the canvas panel when expanded
    #[rust]
    canvas_panel_width: f64,
    /// Whether the splitter is being dragged
    #[rust]
    splitter_dragging: bool,
    /// Whether A2UI is enabled for the current chat
    #[rust]
    a2ui_enabled: bool,
    /// Starting X position when drag started
    #[rust]
    splitter_drag_start_x: f64,
    /// Starting width when drag started
    #[rust]
    splitter_drag_start_width: f64,
    /// Current A2UI JSON received from the model
    #[rust]
    pending_a2ui_json: Option<String>,
    /// Chat IDs shown in the sidebar history sublist (up to 6)
    #[rust]
    sidebar_chat_ids: Vec<moly_data::ChatId>,

    // ── Model-selector state ────────────────────────────────────────────────
    /// Whether the model-selector dropdown is currently open
    #[rust]
    selector_open: bool,
    /// Registry ID of the currently loaded local model (empty = none)
    #[rust]
    loaded_model_id: String,
    /// Display name of the loaded model
    #[rust]
    loaded_model_name: String,
    /// Category of the loaded model
    #[rust]
    loaded_model_category: Option<RegistryCategory>,
    /// Load state for the shell-level model selector
    #[rust]
    shell_load_state: ShellModelLoadState,
    /// Receiver for the async load thread
    #[rust]
    load_rx: Option<mpsc::Receiver<Result<(), String>>>,
    /// List of downloaded models available for selection
    #[rust]
    downloaded_models: Vec<DownloadedModelEntry>,
}

impl LiveHook for App {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        if !self.initialized {
            // Load Store from disk (this is called after Makepad creates the struct)
            self.store = Store::load();

            // Set current_view from loaded preferences
            self.current_view = match self.store.current_view() {
                "Settings"  => NavigationTarget::Settings,
                "ActiveChat" => NavigationTarget::ActiveChat,
                "LlmHub"   => NavigationTarget::LlmHub,
                "VlmHub"   => NavigationTarget::VlmHub,
                "AsrHub"   => NavigationTarget::AsrHub,
                "TtsHub"   => NavigationTarget::TtsHub,
                "ImageHub" => NavigationTarget::ImageHub,
                _ => NavigationTarget::ChatHistory,
            };

            // Initialize MolyAppData from Store preferences
            self.app_data = MolyAppData::new();
            self.app_data.sync_from_preferences(
                self.store.is_sidebar_expanded(),
                self.store.current_view(),
                self.store.preferences.get_current_chat_model(),
            );

            self.initialized = true;
            ::log::info!("App initialized via LiveHook, store loaded from disk");
        }
    }
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        moly_widgets::live_design(cx);
        // Register moly-kit widgets (Chat, Messages, PromptInput, etc.)
        moly_kit::widgets::live_design(cx);
        // Register app widgets from external app crates via MolyApp trait
        <moly_chat::MolyChatApp as MolyApp>::live_design(cx);
        <moly_settings::MolySettingsApp as MolyApp>::live_design(cx);
        <moly_mcp::MolyMcpApp as MolyApp>::live_design(cx);
        <moly_hub::MolyHubApp as MolyApp>::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.update_sidebar(cx);
        // Force apply view state on startup (bypass same-view check)
        self.apply_view_state(cx, self.current_view);
        // Populate sidebar chat history items
        self.update_sidebar_chats(cx);
        // Initialize canvas panel — collapsed by default, opens when A2UI is enabled
        self.canvas_panel_width = 500.0;
        self.canvas_panel_collapsed = true;
        ::log::info!("App initialized with Store and MolyAppData");
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // ── Model selector pill click ───────────────────────────────────────
        if self.ui.view(ids!(body.body_layout.header.model_selector_btn)).finger_down(&actions).is_some() {
            if self.selector_open {
                self.close_selector(cx);
            } else {
                self.open_selector(cx);
            }
        }

        // ── Eject / unload button click ────────────────────────────────────
        if self.ui.view(ids!(body.body_layout.header.eject_btn)).finger_down(&actions).is_some() {
            self.start_unload_model(cx);
        }

        // ── Dropdown: click-outside dismiss area ───────────────────────────
        if self.selector_open {
            if self.ui.view(ids!(body.model_selector_dropdown.dismiss_area)).finger_down(&actions).is_some() {
                self.close_selector(cx);
            }

            // ── Dropdown slot clicks (explicit, no macro_rules) ─────────────
            let n = self.downloaded_models.len();
            if n > 0 && self.ui.view(ids!(body.model_selector_dropdown.dropdown_wrapper.dropdown_panel.model_scroll.slot_0)).finger_down(&actions).is_some() {
                let entry = self.downloaded_models[0].clone();
                self.close_selector(cx);
                self.start_load_model(cx, entry);
            }
            if n > 1 && self.ui.view(ids!(body.model_selector_dropdown.dropdown_wrapper.dropdown_panel.model_scroll.slot_1)).finger_down(&actions).is_some() {
                let entry = self.downloaded_models[1].clone();
                self.close_selector(cx);
                self.start_load_model(cx, entry);
            }
            if n > 2 && self.ui.view(ids!(body.model_selector_dropdown.dropdown_wrapper.dropdown_panel.model_scroll.slot_2)).finger_down(&actions).is_some() {
                let entry = self.downloaded_models[2].clone();
                self.close_selector(cx);
                self.start_load_model(cx, entry);
            }
            if n > 3 && self.ui.view(ids!(body.model_selector_dropdown.dropdown_wrapper.dropdown_panel.model_scroll.slot_3)).finger_down(&actions).is_some() {
                let entry = self.downloaded_models[3].clone();
                self.close_selector(cx);
                self.start_load_model(cx, entry);
            }
            if n > 4 && self.ui.view(ids!(body.model_selector_dropdown.dropdown_wrapper.dropdown_panel.model_scroll.slot_4)).finger_down(&actions).is_some() {
                let entry = self.downloaded_models[4].clone();
                self.close_selector(cx);
                self.start_load_model(cx, entry);
            }
            if n > 5 && self.ui.view(ids!(body.model_selector_dropdown.dropdown_wrapper.dropdown_panel.model_scroll.slot_5)).finger_down(&actions).is_some() {
                let entry = self.downloaded_models[5].clone();
                self.close_selector(cx);
                self.start_load_model(cx, entry);
            }
            if n > 6 && self.ui.view(ids!(body.model_selector_dropdown.dropdown_wrapper.dropdown_panel.model_scroll.slot_6)).finger_down(&actions).is_some() {
                let entry = self.downloaded_models[6].clone();
                self.close_selector(cx);
                self.start_load_model(cx, entry);
            }
            if n > 7 && self.ui.view(ids!(body.model_selector_dropdown.dropdown_wrapper.dropdown_panel.model_scroll.slot_7)).finger_down(&actions).is_some() {
                let entry = self.downloaded_models[7].clone();
                self.close_selector(cx);
                self.start_load_model(cx, entry);
            }
            if n > 8 && self.ui.view(ids!(body.model_selector_dropdown.dropdown_wrapper.dropdown_panel.model_scroll.slot_8)).finger_down(&actions).is_some() {
                let entry = self.downloaded_models[8].clone();
                self.close_selector(cx);
                self.start_load_model(cx, entry);
            }
            if n > 9 && self.ui.view(ids!(body.model_selector_dropdown.dropdown_wrapper.dropdown_panel.model_scroll.slot_9)).finger_down(&actions).is_some() {
                let entry = self.downloaded_models[9].clone();
                self.close_selector(cx);
                self.start_load_model(cx, entry);
            }
        }

        // Handle hamburger menu click
        if self.ui.view(ids!(body.body_layout.header.hamburger_btn)).finger_down(&actions).is_some() {
            ::log::info!(">>> Hamburger button clicked! <<<");
            self.store.toggle_sidebar();
            self.update_sidebar(cx);
        }

        // Handle New Chat button click (first item in sidebar)
        // Use full path from Window root: body.content.sidebar.new_chat_btn
        let new_chat_clicked = self.ui.button(ids!(body.body_layout.content.sidebar.new_chat_btn)).clicked(&actions);
        let chat_clicked = self.ui.button(ids!(body.body_layout.content.sidebar.chat_section.chat_history_btn)).clicked(&actions);

        if new_chat_clicked {
            ::log::info!(">>> New Chat button clicked! <<<");

            // Request new chat directly on ChatApp (bypasses action dispatch timing issues)
            if let Some(mut chat_app) = self.ui.widget(ids!(body.body_layout.content.main_content.chat_with_canvas.chat_app))
                .borrow_mut::<moly_chat::screen::ChatApp>()
            {
                chat_app.request_new_chat();
            }

            // Clear A2UI canvas for the new chat
            self.pending_a2ui_json = None;
            self.clear_a2ui_canvas(cx);

            // Always show active chat view when creating new chat
            self.current_view = NavigationTarget::ActiveChat;
            self.store.set_current_view("ActiveChat");
            self.apply_view_state(cx, NavigationTarget::ActiveChat);
            self.update_sidebar_chats(cx);
        } else if chat_clicked {
            ::log::info!("Chat button clicked - opening chat history page");
            // Navigate to chat history page (blank page with "Chat History" text)
            self.navigate_to(cx, NavigationTarget::ChatHistory);
        }

        // Handle Show More button click
        if self.ui.view(ids!(body.body_layout.content.sidebar.chat_section.chat_history_visible.show_more_btn)).finger_down(&actions).is_some() {
            self.chat_history_expanded = !self.chat_history_expanded;
            self.update_chat_history_visibility(cx);
        }
        if self.ui.button(ids!(body.body_layout.content.sidebar.llm_btn)).clicked(&actions) {
            self.navigate_to(cx, NavigationTarget::LlmHub);
        }
        if self.ui.button(ids!(body.body_layout.content.sidebar.vlm_btn)).clicked(&actions) {
            self.navigate_to(cx, NavigationTarget::VlmHub);
        }
        if self.ui.button(ids!(body.body_layout.content.sidebar.asr_btn)).clicked(&actions) {
            self.navigate_to(cx, NavigationTarget::AsrHub);
        }
        if self.ui.button(ids!(body.body_layout.content.sidebar.tts_btn)).clicked(&actions) {
            self.navigate_to(cx, NavigationTarget::TtsHub);
        }
        if self.ui.button(ids!(body.body_layout.content.sidebar.image_btn)).clicked(&actions) {
            self.navigate_to(cx, NavigationTarget::ImageHub);
        }
        if self.ui.button(ids!(body.body_layout.content.sidebar.settings_btn)).clicked(&actions) {
            ::log::info!(">>> Settings button clicked! <<<");
            self.navigate_to(cx, NavigationTarget::Settings);
        }

        // Handle sidebar chat history item clicks
        {
            let mut sidebar_clicked: Option<usize> = None;
            macro_rules! check_sidebar {
                ($index:expr, $section:ident, $item:ident) => {
                    if sidebar_clicked.is_none() && $index < self.sidebar_chat_ids.len() {
                        if self.ui.view(ids!(body.body_layout.content.sidebar.chat_section.$section.$item))
                            .finger_down(&actions).is_some()
                        {
                            sidebar_clicked = Some($index);
                        }
                    }
                };
            }
            check_sidebar!(0, chat_history_visible, chat_item_0);
            check_sidebar!(1, chat_history_visible, chat_item_1);
            check_sidebar!(2, chat_history_visible, chat_item_2);
            check_sidebar!(3, chat_history_more, chat_item_3);
            check_sidebar!(4, chat_history_more, chat_item_4);
            check_sidebar!(5, chat_history_more, chat_item_5);

            if let Some(idx) = sidebar_clicked {
                let chat_id = self.sidebar_chat_ids[idx];
                self.store.chats.set_current_chat(Some(chat_id));
                if let Some(mut chat_app) = self.ui.widget(ids!(body.body_layout.content.main_content.chat_with_canvas.chat_app))
                    .borrow_mut::<moly_chat::screen::ChatApp>()
                {
                    chat_app.load_chat(chat_id);
                }
                self.current_view = NavigationTarget::ActiveChat;
                self.store.set_current_view("ActiveChat");
                self.apply_view_state(cx, NavigationTarget::ActiveChat);
            }
        }

        // Handle chat tile clicks
        self.handle_chat_tile_clicks(cx, actions);

        // Handle search input changes
        let search_input = self.ui.text_input(ids!(body.body_layout.content.main_content.chat_history_page.search_container.search_input));
        if search_input.changed(&actions).is_some() {
            self.search_query = search_input.text();
            self.update_chat_tiles(cx);
        }

        // Handle canvas reopen strip (shown when canvas is collapsed)
        if self.ui.view(ids!(body.body_layout.content.main_content.chat_with_canvas.canvas_reopen_btn)).finger_down(&actions).is_some() {
            ::log::info!(">>> Canvas reopen strip clicked! <<<");
            self.toggle_canvas_panel(cx);
        }

        // Handle canvas collapse strip (shown when canvas is expanded)
        if self.ui.view(ids!(body.body_layout.content.main_content.chat_with_canvas.canvas_section.canvas_toggle_column)).finger_down(&actions).is_some() {
            ::log::info!(">>> Canvas collapse strip clicked! <<<");
            self.toggle_canvas_panel(cx);
        }

        // Handle canvas splitter drag start
        let splitter = self.ui.view(ids!(body.body_layout.content.main_content.chat_with_canvas.canvas_splitter));
        if let Some(fd) = splitter.finger_down(&actions) {
            if !self.canvas_panel_collapsed {
                self.splitter_dragging = true;
                self.splitter_drag_start_x = fd.abs.x;
                self.splitter_drag_start_width = self.canvas_panel_width;
                ::log::info!("Splitter drag started at x={}", fd.abs.x);
            }
        }

        // Handle navigation requests from child widgets
        for action in actions {
            if let StoreAction::Navigate(view) = action.cast() {
                let target = match view.as_str() {
                    "ActiveChat"  => Some(NavigationTarget::ActiveChat),
                    "ChatHistory" => Some(NavigationTarget::ChatHistory),
                    "Settings"    => Some(NavigationTarget::Settings),
                    "LlmHub"   => Some(NavigationTarget::LlmHub),
                    "VlmHub"   => Some(NavigationTarget::VlmHub),
                    "AsrHub"   => Some(NavigationTarget::AsrHub),
                    "TtsHub"   => Some(NavigationTarget::TtsHub),
                    "ImageHub" => Some(NavigationTarget::ImageHub),
                    _ => None,
                };
                if let Some(t) = target {
                    ::log::info!("StoreAction::Navigate({}) → {:?}", view, t);
                    self.navigate_to(cx, t);
                }
            }
            // Handle "Open in Chat" from Model Hub — create new chat with the selected model
            if let StoreAction::OpenChatWithModel { model_id, .. } = action.cast() {
                ::log::info!(">>> OpenChatWithModel: {} <<<", model_id);
                // Set active local model (injects ominix-local provider)
                self.store.set_active_local_model(Some(model_id.clone()));
                // Request a new chat session
                if let Some(mut chat_app) = self.ui.widget(ids!(body.body_layout.content.main_content.chat_with_canvas.chat_app))
                    .borrow_mut::<moly_chat::screen::ChatApp>()
                {
                    chat_app.request_new_chat();
                }
                self.navigate_to(cx, NavigationTarget::ActiveChat);
                self.update_sidebar_chats(cx);
            }
        }

        // Refresh sidebar when ChatApp creates a new chat (deferred from request_new_chat)
        for action in actions {
            if let moly_chat::screen::ChatHistoryAction::ChatCreated = action.cast() {
                ::log::info!("ChatHistoryAction::ChatCreated — refreshing sidebar");
                self.update_sidebar_chats(cx);
            }
        }

        // Handle A2UI toggle from PromptInput and A2UI tool calls from Chat
        for action in actions {
            if let PromptInputAction::A2uiToggled(enabled) = action.cast() {
                ::log::info!("A2UI toggled: {}", enabled);
                self.a2ui_enabled = enabled;
                if enabled {
                    // Auto-expand canvas panel when A2UI is enabled
                    if self.canvas_panel_collapsed {
                        self.toggle_canvas_panel(cx);
                    }
                } else {
                    // Auto-collapse canvas panel when A2UI is disabled
                    if !self.canvas_panel_collapsed {
                        self.toggle_canvas_panel(cx);
                    }
                    // Clear pending A2UI JSON when disabled
                    self.pending_a2ui_json = None;
                }
            }

            // Handle A2UI JSON from Chat widget
            match action.cast() {
                ChatAction::A2uiJson(json) => {
                    ::log::info!(
                        "Received A2UI JSON ({} bytes)",
                        json.len()
                    );
                    self.pending_a2ui_json = Some(json);
                    self.render_a2ui_canvas(cx);
                }
                ChatAction::A2uiToggled(enabled) => {
                    ::log::info!(
                        "ChatAction::A2uiToggled({})",
                        enabled
                    );
                }
                ChatAction::None => {}
            }

            // Handle A2UI surface data model changes (two-way binding)
            if let A2uiSurfaceAction::DataModelChanged {
                surface_id, path, value
            } = action.cast() {
                ::log::info!(
                    "A2UI DataModelChanged: surface={}, path={}, value={}",
                    surface_id, path, value
                );
                let surface_ref = self.ui.widget(ids!(
                    body.content.main_content.chat_with_canvas
                        .canvas_section.canvas_content
                        .canvas_area.a2ui_surface
                ));
                if let Some(mut surface) =
                    surface_ref.borrow_mut::<A2uiSurface>()
                {
                    if let Some(processor) = surface.processor_mut() {
                        if let Some(data_model) =
                            processor.get_data_model_mut(&surface_id)
                        {
                            data_model.set(&path, value);
                        }
                    }
                }
                self.ui.redraw(cx);
            }
        }

        // Poll global state for pending A2UI JSON
        // (action propagation from nested Chat widget doesn't reach here)
        if let Some(json) = take_pending_a2ui_json() {
            ::log::info!(
                "Picked up pending A2UI JSON from global state ({} bytes)",
                json.len()
            );
            self.pending_a2ui_json = Some(json);
            self.render_a2ui_canvas(cx);
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        // Handle splitter dragging with global mouse events
        if self.splitter_dragging {
            match event {
                Event::MouseMove(mm) => {
                    // Dragging left (negative delta) should increase canvas width
                    // Dragging right (positive delta) should decrease canvas width
                    let delta = mm.abs.x - self.splitter_drag_start_x;
                    let new_width = (self.splitter_drag_start_width - delta).max(200.0).min(1200.0);
                    self.canvas_panel_width = new_width;

                    self.ui.view(ids!(body.body_layout.content.main_content.chat_with_canvas.canvas_section))
                        .apply_over(cx, live!{ width: (new_width) });
                    self.ui.redraw(cx);
                }
                Event::MouseUp(_) => {
                    self.splitter_dragging = false;
                    ::log::info!("Splitter drag ended, width={}", self.canvas_panel_width);
                }
                _ => {}
            }
        }

        // Poll model load thread for completion
        self.poll_load_result(cx);

        // Pass Store to child widgets via Scope
        // TODO: Migrate apps to use MolyAppData instead of Store directly
        // For now, we pass Store for backwards compatibility
        // IMPORTANT: ui.handle_event must be called BEFORE match_event
        // because actions are generated during handle_event and then
        // processed by match_event's handle_actions
        let scope = &mut Scope::with_data(&mut self.store);
        self.ui.handle_event(cx, event, scope);

        // Process actions after they've been generated
        self.match_event(cx, event);
    }
}

impl App {
    // ── Model selector methods ────────────────────────────────────────────────

    /// Scan the registry for downloaded models and cache the list.
    fn refresh_downloaded_models(&mut self) {
        let registry = ModelRegistry::load();
        self.downloaded_models = registry.models.iter()
            .filter(|m| shell_is_model_downloaded(m))
            .map(|m| DownloadedModelEntry {
                registry_id:    m.id.clone(),
                name:           m.name.clone(),
                api_model_id:   m.runtime.api_model_id.clone(),
                category:       m.category,
                model_type_str: category_to_model_type(m.category),
                size_display:   m.storage.size_display.clone(),
            })
            .collect();
        ::log::info!("Model selector: {} downloaded models", self.downloaded_models.len());
    }

    /// Open the dropdown and populate slots.
    fn open_selector(&mut self, cx: &mut Cx) {
        if self.shell_load_state == ShellModelLoadState::Loading { return; }
        self.refresh_downloaded_models();
        self.selector_open = true;
        self.update_dropdown_slots(cx);
        self.ui.view(ids!(body.model_selector_dropdown)).set_visible(cx, true);
        self.ui.redraw(cx);
    }

    /// Close the dropdown.
    fn close_selector(&mut self, cx: &mut Cx) {
        self.selector_open = false;
        self.ui.view(ids!(body.model_selector_dropdown)).set_visible(cx, false);
        self.ui.redraw(cx);
    }

    /// Update selector pill label and eject-button visibility.
    fn update_selector_bar(&mut self, cx: &mut Cx) {
        let label_text = match self.shell_load_state {
            ShellModelLoadState::Unloaded => "Select a model to load".to_string(),
            ShellModelLoadState::Loading  => format!("Loading {}...", self.loaded_model_name),
            ShellModelLoadState::Loaded   => self.loaded_model_name.clone(),
            ShellModelLoadState::Error    => "Load failed — click to retry".to_string(),
        };
        let loaded = matches!(self.shell_load_state, ShellModelLoadState::Loaded);

        self.ui.label(ids!(body.body_layout.header.model_selector_btn.selector_label))
            .set_text(cx, &label_text);

        let loaded_val = if loaded { 1.0 } else { 0.0 };
        self.ui.view(ids!(body.body_layout.header.model_selector_btn.selector_dot))
            .apply_over(cx, live!{ draw_bg: { loaded: (loaded_val) } });

        self.ui.view(ids!(body.body_layout.header.eject_btn))
            .set_visible(cx, loaded);

        // Category tag — show with correct type/color when a model is loaded
        let tag = self.ui.view(ids!(body.body_layout.header.model_selector_btn.category_tag));
        tag.set_visible(cx, loaded);
        if loaded {
            let cat_val = registry_category_as_f64(
                self.loaded_model_category.unwrap_or(RegistryCategory::Llm)
            );
            let cat_label = match self.loaded_model_category {
                Some(RegistryCategory::Llm)      => "LLM",
                Some(RegistryCategory::Vlm)      => "VLM",
                Some(RegistryCategory::Asr)      => "ASR",
                Some(RegistryCategory::Tts)      => "TTS",
                Some(RegistryCategory::ImageGen) => "Image",
                Some(RegistryCategory::VideoGen) => "Video",
                None => "LLM",
            };
            tag.apply_over(cx, live! { draw_bg: { cat: (cat_val) } });
            tag.label(ids!(category_tag_label)).set_text(cx, cat_label);
            tag.label(ids!(category_tag_label)).apply_over(cx, live! { draw_text: { cat: (cat_val) } });
        }
    }

    /// Write data into a single dropdown slot widget.
    fn update_slot_view(&self, cx: &mut Cx, slot: WidgetRef, entry: &DownloadedModelEntry) {
        slot.set_visible(cx, true);
        slot.label(ids!(slot_name)).set_text(cx, &entry.name);
        let meta = format!("{} · {}", entry.category.label(), entry.size_display);
        slot.label(ids!(slot_meta)).set_text(cx, &meta);
        let is_loaded = self.loaded_model_id == entry.registry_id;
        let loaded_val = if is_loaded { 1.0 } else { 0.0 };
        slot.view(ids!(slot_loaded_dot)).apply_over(cx, live!{ draw_bg: { loaded: (loaded_val) } });
        let cat_val = registry_category_as_f64(entry.category);
        slot.view(ids!(category_dot)).apply_over(cx, live!{ draw_bg: { cat: (cat_val) } });
    }

    /// Populate/hide all 10 dropdown slots from `self.downloaded_models`.
    fn update_dropdown_slots(&mut self, cx: &mut Cx) {
        let models = self.downloaded_models.clone();
        let n = models.len();

        self.ui.view(ids!(body.model_selector_dropdown.dropdown_wrapper.dropdown_panel.empty_state))
            .set_visible(cx, n == 0);
        self.ui.view(ids!(body.model_selector_dropdown.dropdown_wrapper.dropdown_panel.model_scroll))
            .set_visible(cx, n > 0);

        // Explicit per-slot update (ids!() can't be in macro_rules)
        let slot = self.ui.widget(ids!(body.model_selector_dropdown.dropdown_wrapper.dropdown_panel.model_scroll.slot_0));
        if n > 0 { self.update_slot_view(cx, slot, &models[0]); } else { slot.set_visible(cx, false); }

        let slot = self.ui.widget(ids!(body.model_selector_dropdown.dropdown_wrapper.dropdown_panel.model_scroll.slot_1));
        if n > 1 { self.update_slot_view(cx, slot, &models[1]); } else { slot.set_visible(cx, false); }

        let slot = self.ui.widget(ids!(body.model_selector_dropdown.dropdown_wrapper.dropdown_panel.model_scroll.slot_2));
        if n > 2 { self.update_slot_view(cx, slot, &models[2]); } else { slot.set_visible(cx, false); }

        let slot = self.ui.widget(ids!(body.model_selector_dropdown.dropdown_wrapper.dropdown_panel.model_scroll.slot_3));
        if n > 3 { self.update_slot_view(cx, slot, &models[3]); } else { slot.set_visible(cx, false); }

        let slot = self.ui.widget(ids!(body.model_selector_dropdown.dropdown_wrapper.dropdown_panel.model_scroll.slot_4));
        if n > 4 { self.update_slot_view(cx, slot, &models[4]); } else { slot.set_visible(cx, false); }

        let slot = self.ui.widget(ids!(body.model_selector_dropdown.dropdown_wrapper.dropdown_panel.model_scroll.slot_5));
        if n > 5 { self.update_slot_view(cx, slot, &models[5]); } else { slot.set_visible(cx, false); }

        let slot = self.ui.widget(ids!(body.model_selector_dropdown.dropdown_wrapper.dropdown_panel.model_scroll.slot_6));
        if n > 6 { self.update_slot_view(cx, slot, &models[6]); } else { slot.set_visible(cx, false); }

        let slot = self.ui.widget(ids!(body.model_selector_dropdown.dropdown_wrapper.dropdown_panel.model_scroll.slot_7));
        if n > 7 { self.update_slot_view(cx, slot, &models[7]); } else { slot.set_visible(cx, false); }

        let slot = self.ui.widget(ids!(body.model_selector_dropdown.dropdown_wrapper.dropdown_panel.model_scroll.slot_8));
        if n > 8 { self.update_slot_view(cx, slot, &models[8]); } else { slot.set_visible(cx, false); }

        let slot = self.ui.widget(ids!(body.model_selector_dropdown.dropdown_wrapper.dropdown_panel.model_scroll.slot_9));
        if n > 9 { self.update_slot_view(cx, slot, &models[9]); } else { slot.set_visible(cx, false); }
    }

    /// Start loading a model in a background thread.
    fn start_load_model(&mut self, cx: &mut Cx, entry: DownloadedModelEntry) {
        let (tx, rx) = mpsc::channel::<Result<(), String>>();
        self.load_rx = Some(rx);
        self.shell_load_state    = ShellModelLoadState::Loading;
        self.loaded_model_id     = entry.registry_id.clone();
        self.loaded_model_name   = entry.name.clone();
        self.loaded_model_category = Some(entry.category);

        let api_model_id  = entry.api_model_id.clone();
        let model_type    = entry.model_type_str.to_string();

        std::thread::spawn(move || {
            let result = ensure_server_running()
                .and_then(|()| ModelRuntimeClient::localhost().load_model(&api_model_id, &model_type));
            let _ = tx.send(result);
        });

        self.update_selector_bar(cx);
        self.ui.redraw(cx);
    }

    /// Optimistically unload the current model (fire-and-forget).
    fn start_unload_model(&mut self, cx: &mut Cx) {
        let model_type = match self.loaded_model_category {
            Some(RegistryCategory::Llm)      => "llm",
            Some(RegistryCategory::Vlm)      => "vlm",
            Some(RegistryCategory::Asr)      => "asr",
            Some(RegistryCategory::Tts)      => "tts",
            Some(RegistryCategory::ImageGen) => "image",
            Some(RegistryCategory::VideoGen) => "video",
            None                             => "all",
        }.to_string();

        std::thread::spawn(move || {
            ModelRuntimeClient::localhost().unload_model(&model_type).ok();
        });

        // Optimistic UI reset
        self.shell_load_state    = ShellModelLoadState::Unloaded;
        self.loaded_model_id     = String::new();
        self.loaded_model_name   = String::new();
        self.loaded_model_category = None;
        self.store.set_active_local_model(None);

        self.update_selector_bar(cx);
        self.ui.redraw(cx);
    }

    /// Poll the load thread; navigate on success, report on failure.
    fn poll_load_result(&mut self, cx: &mut Cx) {
        let result = self.load_rx.as_ref().and_then(|rx| rx.try_recv().ok());
        let Some(result) = result else { return };
        self.load_rx = None;

        match result {
            Ok(()) => {
                self.shell_load_state = ShellModelLoadState::Loaded;

                // Inject model into store so ChatApp routes to localhost
                let model_id = self.loaded_model_id.clone();
                self.store.set_active_local_model(Some(model_id));

                // Navigate to the appropriate view for this model category
                let nav = match self.loaded_model_category {
                    Some(RegistryCategory::Vlm)      => NavigationTarget::VlmHub,
                    Some(RegistryCategory::Asr)      => NavigationTarget::AsrHub,
                    Some(RegistryCategory::Tts)      => NavigationTarget::TtsHub,
                    Some(RegistryCategory::ImageGen) => NavigationTarget::ImageHub,
                    _                                => NavigationTarget::ActiveChat,
                };

                if nav == NavigationTarget::ActiveChat {
                    if let Some(mut chat_app) = self.ui
                        .widget(ids!(body.body_layout.content.main_content.chat_with_canvas.chat_app))
                        .borrow_mut::<moly_chat::screen::ChatApp>()
                    {
                        chat_app.request_new_chat();
                    }
                }

                self.navigate_to(cx, nav);

                // Auto-focus the loaded model inside its hub inference panel
                let mid = self.loaded_model_id.clone();
                match nav {
                    NavigationTarget::VlmHub => {
                        if let Some(mut h) = self.ui.widget(ids!(body.body_layout.content.main_content.vlm_hub_app))
                            .borrow_mut::<moly_hub::screen::ModelHubApp>() { h.focus_model(cx, &mid); }
                    }
                    NavigationTarget::AsrHub => {
                        if let Some(mut h) = self.ui.widget(ids!(body.body_layout.content.main_content.asr_hub_app))
                            .borrow_mut::<moly_hub::screen::ModelHubApp>() { h.focus_model(cx, &mid); }
                    }
                    NavigationTarget::TtsHub => {
                        if let Some(mut h) = self.ui.widget(ids!(body.body_layout.content.main_content.tts_hub_app))
                            .borrow_mut::<moly_hub::screen::ModelHubApp>() { h.focus_model(cx, &mid); }
                    }
                    NavigationTarget::ImageHub => {
                        if let Some(mut h) = self.ui.widget(ids!(body.body_layout.content.main_content.image_hub_app))
                            .borrow_mut::<moly_hub::screen::ModelHubApp>() { h.focus_model(cx, &mid); }
                    }
                    _ => {}
                }

                self.update_sidebar_chats(cx);
            }
            Err(e) => {
                self.shell_load_state    = ShellModelLoadState::Error;
                self.loaded_model_id     = String::new();
                self.loaded_model_name   = String::new();
                self.loaded_model_category = None;
                ::log::error!("Model load failed: {}", e);
            }
        }

        self.update_selector_bar(cx);
        self.ui.redraw(cx);
    }

    fn navigate_to(&mut self, cx: &mut Cx, target: NavigationTarget) {
        ::log::info!("navigate_to: current={:?}, target={:?}", self.current_view, target);
        self.current_view = target;

        // Persist to Store
        let view_name = match target {
            NavigationTarget::ChatHistory => "ChatHistory",
            NavigationTarget::ActiveChat  => "ActiveChat",
            NavigationTarget::Settings    => "Settings",
            NavigationTarget::LlmHub      => "LlmHub",
            NavigationTarget::VlmHub      => "VlmHub",
            NavigationTarget::AsrHub      => "AsrHub",
            NavigationTarget::TtsHub      => "TtsHub",
            NavigationTarget::ImageHub    => "ImageHub",
        };
        self.store.set_current_view(view_name);

        self.apply_view_state(cx, target);
    }

    /// Apply UI state for the given view (visibility and button selection)
    fn apply_view_state(&mut self, cx: &mut Cx, target: NavigationTarget) {
        // Update app visibility
        // Chat history page and active chat are mutually exclusive
        let show_chat_history = target == NavigationTarget::ChatHistory;
        let show_active_chat = target == NavigationTarget::ActiveChat;

        self.ui.widget(ids!(body.body_layout.content.main_content.chat_history_page)).set_visible(cx, show_chat_history);
        self.ui.widget(ids!(body.body_layout.content.main_content.chat_with_canvas)).set_visible(cx, show_active_chat);
        self.ui.widget(ids!(body.body_layout.content.main_content.llm_hub_app)).set_visible(cx, target == NavigationTarget::LlmHub);
        self.ui.widget(ids!(body.body_layout.content.main_content.vlm_hub_app)).set_visible(cx, target == NavigationTarget::VlmHub);
        self.ui.widget(ids!(body.body_layout.content.main_content.asr_hub_app)).set_visible(cx, target == NavigationTarget::AsrHub);
        self.ui.widget(ids!(body.body_layout.content.main_content.tts_hub_app)).set_visible(cx, target == NavigationTarget::TtsHub);
        self.ui.widget(ids!(body.body_layout.content.main_content.image_hub_app)).set_visible(cx, target == NavigationTarget::ImageHub);
        self.ui.widget(ids!(body.body_layout.content.main_content.settings_app)).set_visible(cx, target == NavigationTarget::Settings);

        // Notify ChatApp when it becomes visible (to refresh model list)
        if show_active_chat {
            if let Some(mut chat_app) = self.ui.widget(ids!(body.body_layout.content.main_content.chat_with_canvas.chat_app)).borrow_mut::<moly_chat::screen::ChatApp>() {
                chat_app.on_become_visible();
            }
        }

        // Update chat tiles when showing chat history
        if show_chat_history {
            self.update_chat_tiles(cx);
        }

        // Update button selection state (SidebarButton is a Button with draw_bg.selected)
        // Chat button is selected for both ChatHistory and ActiveChat
        let chat_selected = show_chat_history || show_active_chat;
        self.ui.button(ids!(body.body_layout.content.sidebar.chat_section.chat_history_btn)).apply_over(cx, live! {
            draw_bg: { selected: (if chat_selected { 1.0 } else { 0.0 }) }
        });
        self.ui.button(ids!(body.body_layout.content.sidebar.llm_btn)).apply_over(cx, live! {
            draw_bg: { selected: (if target == NavigationTarget::LlmHub { 1.0 } else { 0.0 }) }
        });
        self.ui.button(ids!(body.body_layout.content.sidebar.vlm_btn)).apply_over(cx, live! {
            draw_bg: { selected: (if target == NavigationTarget::VlmHub { 1.0 } else { 0.0 }) }
        });
        self.ui.button(ids!(body.body_layout.content.sidebar.asr_btn)).apply_over(cx, live! {
            draw_bg: { selected: (if target == NavigationTarget::AsrHub { 1.0 } else { 0.0 }) }
        });
        self.ui.button(ids!(body.body_layout.content.sidebar.tts_btn)).apply_over(cx, live! {
            draw_bg: { selected: (if target == NavigationTarget::TtsHub { 1.0 } else { 0.0 }) }
        });
        self.ui.button(ids!(body.body_layout.content.sidebar.image_btn)).apply_over(cx, live! {
            draw_bg: { selected: (if target == NavigationTarget::ImageHub { 1.0 } else { 0.0 }) }
        });
        self.ui.button(ids!(body.body_layout.content.sidebar.settings_btn)).apply_over(cx, live! {
            draw_bg: { selected: (if target == NavigationTarget::Settings { 1.0 } else { 0.0 }) }
        });

        self.ui.redraw(cx);
    }

    fn update_sidebar(&mut self, cx: &mut Cx) {
        let expanded = self.store.is_sidebar_expanded();
        let width = if expanded { 250.0 } else { 60.0 };

        self.ui.view(ids!(body.body_layout.content.sidebar)).apply_over(cx, live! {
            width: (width)
        });

        // Hide section label views and chat history sublist when sidebar is collapsed to icon-only mode
        self.ui.view(ids!(body.body_layout.content.sidebar.chat_section_label)).set_visible(cx, expanded);
        self.ui.view(ids!(body.body_layout.content.sidebar.models_section_label)).set_visible(cx, expanded);
        self.ui.view(ids!(body.body_layout.content.sidebar.chat_section.chat_history_visible)).set_visible(cx, expanded);

        self.ui.redraw(cx);
    }

    /// Populate sidebar chat history items (items 0-5) from the Store.
    /// Called on startup, after new chat creation, and after chat deletion.
    fn update_sidebar_chats(&mut self, cx: &mut Cx) {
        let chats: Vec<_> = self.store.chats.get_sorted_chats()
            .into_iter()
            .take(6)
            .collect();
        let n = chats.len();
        self.sidebar_chat_ids = chats.iter().map(|c| c.id).collect();

        macro_rules! update_item {
            ($index:expr, $section:ident, $item:ident) => {
                let vis = $index < n;
                self.ui.view(ids!(body.body_layout.content.sidebar.chat_section.$section.$item))
                    .set_visible(cx, vis);
                if vis {
                    // Sanitize: collapse newlines, truncate to single display line
                    let raw = &chats[$index].title;
                    let single: String = raw.split_whitespace().collect::<Vec<_>>().join(" ");
                    let display = if single.chars().count() > 28 {
                        let head: String = single.chars().take(26).collect();
                        format!("{}…", head)
                    } else {
                        single
                    };
                    self.ui.label(ids!(body.body_layout.content.sidebar.chat_section.$section.$item.title))
                        .set_text(cx, &display);
                }
            };
        }

        update_item!(0, chat_history_visible, chat_item_0);
        update_item!(1, chat_history_visible, chat_item_1);
        update_item!(2, chat_history_visible, chat_item_2);
        update_item!(3, chat_history_more, chat_item_3);
        update_item!(4, chat_history_more, chat_item_4);
        update_item!(5, chat_history_more, chat_item_5);

        // Only show "Show More" when there are more than 3 chats
        self.ui.view(ids!(body.body_layout.content.sidebar.chat_section.chat_history_visible.show_more_btn))
            .set_visible(cx, n > 3);

        self.ui.redraw(cx);
    }

    /// Update chat history visibility based on expanded state
    fn update_chat_history_visibility(&mut self, cx: &mut Cx) {
        // Update "Show More" section visibility
        self.ui.view(ids!(body.body_layout.content.sidebar.chat_section.chat_history_more)).set_visible(cx, self.chat_history_expanded);

        // Update "Show More" button text and arrow
        let (text, arrow) = if self.chat_history_expanded {
            ("Show Less", "v")
        } else {
            ("Show More", ">")
        };
        self.ui.label(ids!(body.body_layout.content.sidebar.chat_section.chat_history_visible.show_more_label)).set_text(cx, text);
        self.ui.label(ids!(body.body_layout.content.sidebar.chat_section.chat_history_visible.show_more_arrow)).set_text(cx, arrow);

        self.ui.redraw(cx);
    }

    /// Toggle the canvas panel visibility (slide in/out)
    fn toggle_canvas_panel(&mut self, cx: &mut Cx) {
        self.canvas_panel_collapsed = !self.canvas_panel_collapsed;

        // Initialize width if not set (default to 500px)
        if self.canvas_panel_width == 0.0 {
            self.canvas_panel_width = 500.0;
        }

        if self.canvas_panel_collapsed {
            // Collapse: hide canvas section, show reopen strip
            self.ui.view(ids!(body.body_layout.content.main_content.chat_with_canvas.canvas_section))
                .set_visible(cx, false);
            self.ui.view(ids!(body.body_layout.content.main_content.chat_with_canvas.canvas_splitter))
                .apply_over(cx, live!{ width: 0 });
            self.ui.view(ids!(body.body_layout.content.main_content.chat_with_canvas.canvas_reopen_btn))
                .set_visible(cx, true);
        } else {
            // Expand: show canvas section at saved width, hide reopen strip
            let width = self.canvas_panel_width;
            self.ui.view(ids!(body.body_layout.content.main_content.chat_with_canvas.canvas_reopen_btn))
                .set_visible(cx, false);
            self.ui.view(ids!(body.body_layout.content.main_content.chat_with_canvas.canvas_section))
                .set_visible(cx, true);
            self.ui.view(ids!(body.body_layout.content.main_content.chat_with_canvas.canvas_section))
                .apply_over(cx, live!{ width: (width) });
            self.ui.view(ids!(body.body_layout.content.main_content.chat_with_canvas.canvas_section.canvas_content))
                .set_visible(cx, true);
            self.ui.view(ids!(body.body_layout.content.main_content.chat_with_canvas.canvas_splitter))
                .apply_over(cx, live!{ width: 16 });
        }

        self.ui.redraw(cx);
    }

    /// Render A2UI components in the canvas area from JSON.
    ///
    /// Takes A2UI JSON (generated by the LLM as structured output) and feeds it
    /// directly to the A2uiSurface for rendering.
    fn render_a2ui_canvas(&mut self, cx: &mut Cx) {
        let Some(json_str) = self.pending_a2ui_json.take() else {
            return;
        };

        let preview_end = json_str
            .char_indices()
            .take_while(|(i, _)| *i < 600)
            .last()
            .map(|(i, c)| i + c.len_utf8())
            .unwrap_or(json_str.len().min(600));
        eprintln!(
            "[A2UI render] JSON ({} bytes), first ~600 chars:\n{}",
            json_str.len(),
            &json_str[..preview_end]
        );
        // Dump full JSON to temp file for debugging
        let _ = std::fs::write("/tmp/a2ui_last_json.txt", &json_str);

        // Test: can serde parse it as generic JSON?
        match serde_json::from_str::<serde_json::Value>(&json_str) {
            Ok(val) => {
                let kind = if val.is_array() {
                    format!("array of {}", val.as_array().unwrap().len())
                } else if val.is_object() {
                    "object".to_string()
                } else {
                    "other".to_string()
                };
                eprintln!("[A2UI render] JSON parses as generic Value: {}", kind);
            }
            Err(e) => {
                eprintln!(
                    "[A2UI render] JSON fails as generic Value: {}",
                    e
                );
                // Dump the problematic area around line 10
                let lines: Vec<&str> = json_str.lines().collect();
                let start = if lines.len() > 7 { 7 } else { 0 };
                let end = if lines.len() > 13 { 13 } else { lines.len() };
                for (i, line) in lines[start..end].iter().enumerate() {
                    eprintln!("  line {}: {}", start + i + 1, line);
                }
            }
        }

        let surface_ref = self.ui.widget(ids!(
            body.content.main_content.chat_with_canvas
                .canvas_section.canvas_content
                .canvas_area.a2ui_surface
        ));
        if let Some(mut surface) = surface_ref.borrow_mut::<A2uiSurface>() {
            surface.clear();
            match surface.process_json(&json_str) {
                Ok(events) => {
                    eprintln!(
                        "[A2UI render] Surface processed {} events",
                        events.len()
                    );
                }
                Err(e) => {
                    eprintln!("[A2UI render] Surface parse error: {}", e);
                }
            }
        } else {
            eprintln!("[A2UI render] Could not borrow A2uiSurface");
        }

        self.ui.redraw(cx);
    }

    /// Clear the A2UI canvas surface.
    fn clear_a2ui_canvas(&mut self, cx: &mut Cx) {
        let surface_ref = self.ui.widget(ids!(
            body.content.main_content.chat_with_canvas
                .canvas_section.canvas_content
                .canvas_area.a2ui_surface
        ));
        if let Some(mut surface) =
            surface_ref.borrow_mut::<A2uiSurface>()
        {
            surface.clear();
        }
        self.ui.redraw(cx);
    }

    /// Update the chat history tiles with data from Store
    fn update_chat_tiles(&mut self, cx: &mut Cx) {
        // Only show chats that have messages (filter out empty chats)
        // Also filter by search query if present
        let search_lower = self.search_query.to_lowercase();
        let chats: Vec<_> = self.store.chats.get_sorted_chats()
            .into_iter()
            .filter(|c| !c.messages.is_empty())
            .filter(|c| {
                if search_lower.is_empty() {
                    return true;
                }
                // Check title
                if c.title.to_lowercase().contains(&search_lower) {
                    return true;
                }
                // Check message content
                c.messages.iter().any(|m| m.content.text.to_lowercase().contains(&search_lower))
            })
            .collect();
        let chat_count = chats.len().min(12); // Max 12 tiles

        // Update displayed_chat_ids
        self.displayed_chat_ids = chats.iter().take(12).map(|c| c.id).collect();

        // Show/hide empty state and scroll container
        let has_chats = chat_count > 0;
        self.ui.view(ids!(body.body_layout.content.main_content.chat_history_page.empty_state)).set_visible(cx, !has_chats);
        self.ui.view(ids!(body.body_layout.content.main_content.chat_history_page.chat_tiles_scroll)).set_visible(cx, has_chats);

        // Show/hide row containers based on how many chats we have
        // Row 0 visible if we have any chats (indices 0-3)
        // Row 1 visible if we have more than 4 chats (indices 4-7)
        // Row 2 visible if we have more than 8 chats (indices 8-11)
        self.ui.view(ids!(body.body_layout.content.main_content.chat_history_page.chat_tiles_scroll.chat_tiles_container.tile_row_0))
            .set_visible(cx, chat_count > 0);
        self.ui.view(ids!(body.body_layout.content.main_content.chat_history_page.chat_tiles_scroll.chat_tiles_container.tile_row_1))
            .set_visible(cx, chat_count > 4);
        self.ui.view(ids!(body.body_layout.content.main_content.chat_history_page.chat_tiles_scroll.chat_tiles_container.tile_row_2))
            .set_visible(cx, chat_count > 8);

        macro_rules! update_tile {
            ($index:expr, $row:ident, $tile:ident) => {
                let visible = $index < chat_count;
                self.ui.view(ids!(body.body_layout.content.main_content.chat_history_page.chat_tiles_scroll.chat_tiles_container.$row.$tile))
                    .set_visible(cx, visible);
                if visible {
                    let chat = chats[$index];
                    self.ui.label(ids!(body.body_layout.content.main_content.chat_history_page.chat_tiles_scroll.chat_tiles_container.$row.$tile.header.title))
                        .set_text(cx, &chat.title);
                    let date_str = chat.accessed_at.format("%b %d, %Y").to_string();
                    self.ui.label(ids!(body.body_layout.content.main_content.chat_history_page.chat_tiles_scroll.chat_tiles_container.$row.$tile.date_label))
                        .set_text(cx, &date_str);
                }
            };
        }

        update_tile!(0, tile_row_0, tile_0);
        update_tile!(1, tile_row_0, tile_1);
        update_tile!(2, tile_row_0, tile_2);
        update_tile!(3, tile_row_0, tile_3);
        update_tile!(4, tile_row_1, tile_0);
        update_tile!(5, tile_row_1, tile_1);
        update_tile!(6, tile_row_1, tile_2);
        update_tile!(7, tile_row_1, tile_3);
        update_tile!(8, tile_row_2, tile_0);
        update_tile!(9, tile_row_2, tile_1);
        update_tile!(10, tile_row_2, tile_2);
        update_tile!(11, tile_row_2, tile_3);

        self.ui.redraw(cx);
    }

    /// Handle chat tile clicks and delete button clicks
    fn handle_chat_tile_clicks(&mut self, cx: &mut Cx, actions: &Actions) {
        let mut tile_clicked: Option<usize> = None;
        let mut delete_clicked: Option<usize> = None;

        macro_rules! check_tile {
            ($index:expr, $row:ident, $tile:ident) => {
                if $index < self.displayed_chat_ids.len() && delete_clicked.is_none() && tile_clicked.is_none() {
                    if self.ui.view(ids!(body.body_layout.content.main_content.chat_history_page.chat_tiles_scroll.chat_tiles_container.$row.$tile.header.delete_btn))
                        .finger_down(actions).is_some() {
                        delete_clicked = Some($index);
                    }
                    else if self.ui.view(ids!(body.body_layout.content.main_content.chat_history_page.chat_tiles_scroll.chat_tiles_container.$row.$tile))
                        .finger_down(actions).is_some() {
                        tile_clicked = Some($index);
                    }
                }
            };
        }

        check_tile!(0, tile_row_0, tile_0);
        check_tile!(1, tile_row_0, tile_1);
        check_tile!(2, tile_row_0, tile_2);
        check_tile!(3, tile_row_0, tile_3);
        check_tile!(4, tile_row_1, tile_0);
        check_tile!(5, tile_row_1, tile_1);
        check_tile!(6, tile_row_1, tile_2);
        check_tile!(7, tile_row_1, tile_3);
        check_tile!(8, tile_row_2, tile_0);
        check_tile!(9, tile_row_2, tile_1);
        check_tile!(10, tile_row_2, tile_2);
        check_tile!(11, tile_row_2, tile_3);

        // Handle delete action
        if let Some(idx) = delete_clicked {
            let chat_id = self.displayed_chat_ids[idx];
            ::log::info!("Delete button clicked for chat at index {}, id={}", idx, chat_id);
            self.store.chats.delete_chat(chat_id);
            self.update_chat_tiles(cx);
            self.update_sidebar_chats(cx);
            return;
        }

        // Handle tile click (open chat)
        if let Some(idx) = tile_clicked {
            let chat_id = self.displayed_chat_ids[idx];
            ::log::info!("Chat tile clicked at index {}, id={}", idx, chat_id);

            // Set current chat in store
            self.store.chats.set_current_chat(Some(chat_id));

            // Load chat in ChatApp
            if let Some(mut chat_app) = self.ui.widget(ids!(body.body_layout.content.main_content.chat_with_canvas.chat_app))
                .borrow_mut::<moly_chat::screen::ChatApp>()
            {
                chat_app.load_chat(chat_id);
            }

            // Navigate to active chat
            self.current_view = NavigationTarget::ActiveChat;
            self.store.set_current_view("ActiveChat");
            self.apply_view_state(cx, NavigationTarget::ActiveChat);
        }
    }
}


app_main!(App);
