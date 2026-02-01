use makepad_widgets::*;

use moly_data::{ChatId, Store};
use moly_kit::aitk::protocol::ToolCall;
use moly_kit::a2ui::{A2uiSurface, A2uiSurfaceAction};
use moly_kit::widgets::chat::ChatAction;
use moly_kit::widgets::prompt_input::PromptInputAction;
use moly_kit::widgets::take_pending_a2ui_tool_calls;
use moly_widgets::{MolyApp, MolyAppData};

use crate::a2ui_builder::A2uiBuilder;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use moly_widgets::theme::*;
    use moly_kit::a2ui::surface::*;

    // Import app widgets from external app crates
    use moly_chat::screen::design::*;
    use moly_models::screen::design::*;
    use moly_settings::screen::design::*;
    use moly_mcp::screen::design::*;
    use moly_local_models::screen::design::*;

    // Icon dependencies
    ICON_HAMBURGER = dep("crate://self/resources/icons/hamburger.svg")
    ICON_SUN = dep("crate://self/resources/icons/sun.svg")
    ICON_MOON = dep("crate://self/resources/icons/moon.svg")
    ICON_CHAT = dep("crate://self/resources/icons/chat.svg")
    ICON_MODELS = dep("crate://self/resources/icons/app.svg")
    ICON_SETTINGS = dep("crate://self/resources/icons/settings.svg")
    ICON_LOCAL_MODELS = dep("crate://self/resources/icons/local-models.svg")
    ICON_NEW_CHAT = dep("crate://self/resources/icons/new-chat.svg")
    ICON_TRASH = dep("crate://self/resources/icons/trash.svg")

    // Logo
    IMG_LOGO = dep("crate://self/resources/moly-logo.png")

    // Provider icons - registered globally so they can be loaded by moly-kit
    ICON_PROVIDER_OPENAI = dep("crate://self/resources/providers/openai.png")
    ICON_PROVIDER_ANTHROPIC = dep("crate://self/resources/providers/anthropic.png")
    ICON_PROVIDER_GEMINI = dep("crate://self/resources/providers/gemini.png")
    ICON_PROVIDER_OLLAMA = dep("crate://self/resources/providers/ollama.png")
    ICON_PROVIDER_DEEPSEEK = dep("crate://self/resources/providers/deepseek.png")
    ICON_PROVIDER_OPENROUTER = dep("crate://self/resources/providers/openrouter.png")
    ICON_PROVIDER_SILICONFLOW = dep("crate://self/resources/providers/siliconflow.png")

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
            instance selected: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                // Light mode colors (theme switching handled separately)
                let normal = #ffffff;
                let hover_color = #f1f5f9;
                let selected_color = #e0e7ff;
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
            color: #1f2937
        }

        draw_icon: {
            fn get_color(self) -> vec4 {
                return #4b5563;
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
                flow: Down
                show_bg: true
                draw_bg: {
                    color: #f5f7fa
                }

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

                    // Logo
                    logo = <Image> {
                        source: (IMG_LOGO)
                        width: 32, height: 32
                        margin: {right: 8}
                    }

                    title_label = <Label> {
                        text: "OminiX Studio"
                        draw_text: {
                            color: #1f2937
                            text_style: <FONT_SEMIBOLD>{ font_size: 24.0 }
                        }
                    }

                    <View> { width: Fill } // Spacer

                    // Theme toggle button
                    theme_toggle = <View> {
                        width: 40, height: Fit
                        align: {x: 0.5, y: 0.5}
                        cursor: Hand
                        event_order: Down
                        show_bg: false

                        theme_icon = <Icon> {
                            draw_icon: {
                                svg_file: (ICON_SUN)
                                color: #f59e0b
                            }
                            icon_walk: {width: 20, height: 20}
                        }
                    }
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

                        // New Chat - first item in sidebar
                        new_chat_btn = <SidebarButton> {
                            text: "New Chat"
                            draw_icon: { svg_file: (ICON_NEW_CHAT) }
                        }

                        // Chat section - click to expand/collapse history
                        chat_section = <View> {
                            width: Fill, height: Fit
                            flow: Down
                            margin: {bottom: 8}

                            chat_btn = <SidebarButton> {
                                text: "Chat"
                                draw_icon: { svg_file: (ICON_CHAT) }
                            }

                            // Chat history list (visible items)
                            chat_history_visible = <View> {
                                width: Fill, height: Fit
                                flow: Down
                                padding: {left: 32}

                                // Chat history items - visible with placeholder text
                                chat_item_0 = <View> {
                                    width: Fill, height: 32
                                    padding: {left: 8, right: 8}
                                    align: {y: 0.5}
                                    cursor: Hand
                                    show_bg: true
                                    draw_bg: {
                                        instance hover: 0.0
                                        instance selected: 1.0
                                        fn pixel(self) -> vec4 {
                                            let base = #ffffff;
                                            let hover_color = #f1f5f9;
                                            let selected_color = #dbeafe;
                                            return mix(mix(base, hover_color, self.hover), selected_color, self.selected);
                                        }
                                    }
                                    chat_title_0 = <Label> {
                                        width: Fill
                                        text: "Current Chat"
                                        draw_text: {
                                            color: #374151
                                            text_style: { font_size: 11.0 }
                                            wrap: Ellipsis
                                        }
                                    }
                                }
                                chat_item_1 = <View> {
                                    width: Fill, height: 32
                                    padding: {left: 8, right: 8}
                                    align: {y: 0.5}
                                    cursor: Hand
                                    show_bg: true
                                    draw_bg: {
                                        instance hover: 0.0
                                        instance selected: 0.0
                                        fn pixel(self) -> vec4 {
                                            let base = #ffffff;
                                            let hover_color = #f1f5f9;
                                            let selected_color = #dbeafe;
                                            return mix(mix(base, hover_color, self.hover), selected_color, self.selected);
                                        }
                                    }
                                    chat_title_1 = <Label> {
                                        width: Fill
                                        text: "Previous Chat 1"
                                        draw_text: {
                                            color: #374151
                                            text_style: { font_size: 11.0 }
                                            wrap: Ellipsis
                                        }
                                    }
                                }
                                chat_item_2 = <View> {
                                    width: Fill, height: 32
                                    padding: {left: 8, right: 8}
                                    align: {y: 0.5}
                                    cursor: Hand
                                    show_bg: true
                                    draw_bg: {
                                        instance hover: 0.0
                                        instance selected: 0.0
                                        fn pixel(self) -> vec4 {
                                            let base = #ffffff;
                                            let hover_color = #f1f5f9;
                                            let selected_color = #dbeafe;
                                            return mix(mix(base, hover_color, self.hover), selected_color, self.selected);
                                        }
                                    }
                                    chat_title_2 = <Label> {
                                        width: Fill
                                        text: "Previous Chat 2"
                                        draw_text: {
                                            color: #374151
                                            text_style: { font_size: 11.0 }
                                            wrap: Ellipsis
                                        }
                                    }
                                }

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
                                            let base = #ffffff;
                                            let hover_color = #f1f5f9;
                                            return mix(base, hover_color, self.hover);
                                        }
                                    }
                                    show_more_label = <Label> {
                                        width: Fill
                                        text: "Show More"
                                        draw_text: {
                                            color: #6b7280
                                            text_style: { font_size: 11.0 }
                                        }
                                    }
                                    show_more_arrow = <Label> {
                                        text: ">"
                                        draw_text: {
                                            color: #6b7280
                                            text_style: { font_size: 11.0 }
                                        }
                                    }
                                }
                            }

                            // More chat history items (hidden by default)
                            chat_history_more = <View> {
                                width: Fill, height: Fit
                                flow: Down
                                padding: {left: 32}
                                visible: false

                                chat_item_3 = <View> {
                                    width: Fill, height: 32
                                    padding: {left: 8, right: 8}
                                    align: {y: 0.5}
                                    cursor: Hand
                                    visible: false
                                    show_bg: true
                                    draw_bg: {
                                        instance hover: 0.0
                                        instance selected: 0.0
                                        fn pixel(self) -> vec4 {
                                            let base = #ffffff;
                                            let hover_color = #f1f5f9;
                                            let selected_color = #dbeafe;
                                            return mix(mix(base, hover_color, self.hover), selected_color, self.selected);
                                        }
                                    }
                                    chat_title_3 = <Label> {
                                        width: Fill
                                        draw_text: {
                                            color: #374151
                                            text_style: { font_size: 11.0 }
                                            wrap: Ellipsis
                                        }
                                    }
                                }
                                chat_item_4 = <View> {
                                    width: Fill, height: 32
                                    padding: {left: 8, right: 8}
                                    align: {y: 0.5}
                                    cursor: Hand
                                    visible: false
                                    show_bg: true
                                    draw_bg: {
                                        instance hover: 0.0
                                        instance selected: 0.0
                                        fn pixel(self) -> vec4 {
                                            let base = #ffffff;
                                            let hover_color = #f1f5f9;
                                            let selected_color = #dbeafe;
                                            return mix(mix(base, hover_color, self.hover), selected_color, self.selected);
                                        }
                                    }
                                    chat_title_4 = <Label> {
                                        width: Fill
                                        draw_text: {
                                            color: #374151
                                            text_style: { font_size: 11.0 }
                                            wrap: Ellipsis
                                        }
                                    }
                                }
                                chat_item_5 = <View> {
                                    width: Fill, height: 32
                                    padding: {left: 8, right: 8}
                                    align: {y: 0.5}
                                    cursor: Hand
                                    visible: false
                                    show_bg: true
                                    draw_bg: {
                                        instance hover: 0.0
                                        instance selected: 0.0
                                        fn pixel(self) -> vec4 {
                                            let base = #ffffff;
                                            let hover_color = #f1f5f9;
                                            let selected_color = #dbeafe;
                                            return mix(mix(base, hover_color, self.hover), selected_color, self.selected);
                                        }
                                    }
                                    chat_title_5 = <Label> {
                                        width: Fill
                                        draw_text: {
                                            color: #374151
                                            text_style: { font_size: 11.0 }
                                            wrap: Ellipsis
                                        }
                                    }
                                }
                            }
                        }
                        models_btn = <SidebarButton> {
                            text: "Models"
                            draw_icon: { svg_file: (ICON_MODELS) }
                        }

                        local_models_btn = <SidebarButton> {
                            text: "Local Models"
                            draw_icon: { svg_file: (ICON_LOCAL_MODELS) }
                        }

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
                                <Label> {
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

                                search_container = <View> {
                                    width: 500, height: 48
                                    show_bg: true
                                    draw_bg: {
                                        fn pixel(self) -> vec4 {
                                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                            sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 12.0);
                                            sdf.fill(#e5e7eb);
                                            return sdf.result;
                                        }
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
                                <Label> {
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

                                    // Row 0: tiles 0-3
                                    tile_row_0 = <View> {
                                        width: Fill, height: Fit
                                        flow: Right
                                        spacing: 20
                                        visible: false

                                        chat_tile_0 = <RoundedView> {
                                            width: Fill, height: 144
                                            show_bg: true, draw_bg: { color: #ffffff, border_radius: 12.0 }
                                            flow: Down
                                            padding: {top: 16, left: 16, right: 16, bottom: 16}
                                            cursor: Hand
                                            visible: false
                                            <View> {
                                                width: Fill, height: Fit
                                                flow: Right
                                                align: {y: 0.0}
                                                chat_tile_title_0 = <Label> {
                                                    width: Fill
                                                    draw_text: { color: #1f2937, text_style: <FONT_SEMIBOLD>{ font_size: 11.0 }, wrap: Ellipsis }
                                                }
                                                delete_btn_0 = <View> {
                                                    width: 28, height: 28
                                                    align: {x: 0.5, y: 0.5}
                                                    cursor: Hand
                                                    <Icon> { draw_icon: { svg_file: (ICON_TRASH), color: #9ca3af }, icon_walk: {width: 18, height: 18} }
                                                }
                                            }
                                            <View> { width: Fill, height: Fill }
                                            chat_tile_date_0 = <Label> { draw_text: { color: #9ca3af, text_style: { font_size: 10.0 } } }
                                        }

                                        chat_tile_1 = <RoundedView> {
                                            width: Fill, height: 144
                                            show_bg: true, draw_bg: { color: #ffffff, border_radius: 12.0 }
                                            flow: Down
                                            padding: {top: 16, left: 16, right: 16, bottom: 16}
                                            cursor: Hand
                                            visible: false
                                            <View> {
                                                width: Fill, height: Fit
                                                flow: Right
                                                align: {y: 0.0}
                                                chat_tile_title_1 = <Label> {
                                                    width: Fill
                                                    draw_text: { color: #1f2937, text_style: <FONT_SEMIBOLD>{ font_size: 11.0 }, wrap: Ellipsis }
                                                }
                                                delete_btn_1 = <View> {
                                                    width: 28, height: 28
                                                    align: {x: 0.5, y: 0.5}
                                                    cursor: Hand
                                                    <Icon> { draw_icon: { svg_file: (ICON_TRASH), color: #9ca3af }, icon_walk: {width: 18, height: 18} }
                                                }
                                            }
                                            <View> { width: Fill, height: Fill }
                                            chat_tile_date_1 = <Label> { draw_text: { color: #9ca3af, text_style: { font_size: 10.0 } } }
                                        }

                                        chat_tile_2 = <RoundedView> {
                                            width: Fill, height: 144
                                            show_bg: true, draw_bg: { color: #ffffff, border_radius: 12.0 }
                                            flow: Down
                                            padding: {top: 16, left: 16, right: 16, bottom: 16}
                                            cursor: Hand
                                            visible: false
                                            <View> {
                                                width: Fill, height: Fit
                                                flow: Right
                                                align: {y: 0.0}
                                                chat_tile_title_2 = <Label> {
                                                    width: Fill
                                                    draw_text: { color: #1f2937, text_style: <FONT_SEMIBOLD>{ font_size: 11.0 }, wrap: Ellipsis }
                                                }
                                                delete_btn_2 = <View> {
                                                    width: 28, height: 28
                                                    align: {x: 0.5, y: 0.5}
                                                    cursor: Hand
                                                    <Icon> { draw_icon: { svg_file: (ICON_TRASH), color: #9ca3af }, icon_walk: {width: 18, height: 18} }
                                                }
                                            }
                                            <View> { width: Fill, height: Fill }
                                            chat_tile_date_2 = <Label> { draw_text: { color: #9ca3af, text_style: { font_size: 10.0 } } }
                                        }

                                        chat_tile_3 = <RoundedView> {
                                            width: Fill, height: 144
                                            show_bg: true, draw_bg: { color: #ffffff, border_radius: 12.0 }
                                            flow: Down
                                            padding: {top: 16, left: 16, right: 16, bottom: 16}
                                            cursor: Hand
                                            visible: false
                                            <View> {
                                                width: Fill, height: Fit
                                                flow: Right
                                                align: {y: 0.0}
                                                chat_tile_title_3 = <Label> {
                                                    width: Fill
                                                    draw_text: { color: #1f2937, text_style: <FONT_SEMIBOLD>{ font_size: 11.0 }, wrap: Ellipsis }
                                                }
                                                delete_btn_3 = <View> {
                                                    width: 28, height: 28
                                                    align: {x: 0.5, y: 0.5}
                                                    cursor: Hand
                                                    <Icon> { draw_icon: { svg_file: (ICON_TRASH), color: #9ca3af }, icon_walk: {width: 18, height: 18} }
                                                }
                                            }
                                            <View> { width: Fill, height: Fill }
                                            chat_tile_date_3 = <Label> { draw_text: { color: #9ca3af, text_style: { font_size: 10.0 } } }
                                        }
                                    }

                                    // Row 1: tiles 4-7
                                    // Row 1: tiles 4-7
                                    tile_row_1 = <View> {
                                        width: Fill, height: Fit
                                        flow: Right
                                        spacing: 20
                                        visible: false

                                        chat_tile_4 = <RoundedView> {
                                            width: Fill, height: 144
                                            show_bg: true, draw_bg: { color: #ffffff, border_radius: 12.0 }
                                            flow: Down
                                            padding: {top: 16, left: 16, right: 16, bottom: 16}
                                            cursor: Hand
                                            visible: false
                                            <View> {
                                                width: Fill, height: Fit
                                                flow: Right
                                                align: {y: 0.0}
                                                chat_tile_title_4 = <Label> {
                                                    width: Fill
                                                    draw_text: { color: #1f2937, text_style: <FONT_SEMIBOLD>{ font_size: 11.0 }, wrap: Ellipsis }
                                                }
                                                delete_btn_4 = <View> {
                                                    width: 28, height: 28
                                                    align: {x: 0.5, y: 0.5}
                                                    cursor: Hand
                                                    <Icon> { draw_icon: { svg_file: (ICON_TRASH), color: #9ca3af }, icon_walk: {width: 18, height: 18} }
                                                }
                                            }
                                            <View> { width: Fill, height: Fill }
                                            chat_tile_date_4 = <Label> { draw_text: { color: #9ca3af, text_style: { font_size: 10.0 } } }
                                        }

                                        chat_tile_5 = <RoundedView> {
                                            width: Fill, height: 144
                                            show_bg: true, draw_bg: { color: #ffffff, border_radius: 12.0 }
                                            flow: Down
                                            padding: {top: 16, left: 16, right: 16, bottom: 16}
                                            cursor: Hand
                                            visible: false
                                            <View> {
                                                width: Fill, height: Fit
                                                flow: Right
                                                align: {y: 0.0}
                                                chat_tile_title_5 = <Label> {
                                                    width: Fill
                                                    draw_text: { color: #1f2937, text_style: <FONT_SEMIBOLD>{ font_size: 11.0 }, wrap: Ellipsis }
                                                }
                                                delete_btn_5 = <View> {
                                                    width: 28, height: 28
                                                    align: {x: 0.5, y: 0.5}
                                                    cursor: Hand
                                                    <Icon> { draw_icon: { svg_file: (ICON_TRASH), color: #9ca3af }, icon_walk: {width: 18, height: 18} }
                                                }
                                            }
                                            <View> { width: Fill, height: Fill }
                                            chat_tile_date_5 = <Label> { draw_text: { color: #9ca3af, text_style: { font_size: 10.0 } } }
                                        }

                                        chat_tile_6 = <RoundedView> {
                                            width: Fill, height: 144
                                            show_bg: true, draw_bg: { color: #ffffff, border_radius: 12.0 }
                                            flow: Down
                                            padding: {top: 16, left: 16, right: 16, bottom: 16}
                                            cursor: Hand
                                            visible: false
                                            <View> {
                                                width: Fill, height: Fit
                                                flow: Right
                                                align: {y: 0.0}
                                                chat_tile_title_6 = <Label> {
                                                    width: Fill
                                                    draw_text: { color: #1f2937, text_style: <FONT_SEMIBOLD>{ font_size: 11.0 }, wrap: Ellipsis }
                                                }
                                                delete_btn_6 = <View> {
                                                    width: 28, height: 28
                                                    align: {x: 0.5, y: 0.5}
                                                    cursor: Hand
                                                    <Icon> { draw_icon: { svg_file: (ICON_TRASH), color: #9ca3af }, icon_walk: {width: 18, height: 18} }
                                                }
                                            }
                                            <View> { width: Fill, height: Fill }
                                            chat_tile_date_6 = <Label> { draw_text: { color: #9ca3af, text_style: { font_size: 10.0 } } }
                                        }

                                        chat_tile_7 = <RoundedView> {
                                            width: Fill, height: 144
                                            show_bg: true, draw_bg: { color: #ffffff, border_radius: 12.0 }
                                            flow: Down
                                            padding: {top: 16, left: 16, right: 16, bottom: 16}
                                            cursor: Hand
                                            visible: false
                                            <View> {
                                                width: Fill, height: Fit
                                                flow: Right
                                                align: {y: 0.0}
                                                chat_tile_title_7 = <Label> {
                                                    width: Fill
                                                    draw_text: { color: #1f2937, text_style: <FONT_SEMIBOLD>{ font_size: 11.0 }, wrap: Ellipsis }
                                                }
                                                delete_btn_7 = <View> {
                                                    width: 28, height: 28
                                                    align: {x: 0.5, y: 0.5}
                                                    cursor: Hand
                                                    <Icon> { draw_icon: { svg_file: (ICON_TRASH), color: #9ca3af }, icon_walk: {width: 18, height: 18} }
                                                }
                                            }
                                            <View> { width: Fill, height: Fill }
                                            chat_tile_date_7 = <Label> { draw_text: { color: #9ca3af, text_style: { font_size: 10.0 } } }
                                        }
                                    }

                                    // Row 2: tiles 8-11
                                    // Row 2: tiles 8-11
                                    tile_row_2 = <View> {
                                        width: Fill, height: Fit
                                        flow: Right
                                        spacing: 20
                                        visible: false

                                        chat_tile_8 = <RoundedView> {
                                            width: Fill, height: 144
                                            show_bg: true, draw_bg: { color: #ffffff, border_radius: 12.0 }
                                            flow: Down
                                            padding: {top: 16, left: 16, right: 16, bottom: 16}
                                            cursor: Hand
                                            visible: false
                                            <View> {
                                                width: Fill, height: Fit
                                                flow: Right
                                                align: {y: 0.0}
                                                chat_tile_title_8 = <Label> {
                                                    width: Fill
                                                    draw_text: { color: #1f2937, text_style: <FONT_SEMIBOLD>{ font_size: 11.0 }, wrap: Ellipsis }
                                                }
                                                delete_btn_8 = <View> {
                                                    width: 28, height: 28
                                                    align: {x: 0.5, y: 0.5}
                                                    cursor: Hand
                                                    <Icon> { draw_icon: { svg_file: (ICON_TRASH), color: #9ca3af }, icon_walk: {width: 18, height: 18} }
                                                }
                                            }
                                            <View> { width: Fill, height: Fill }
                                            chat_tile_date_8 = <Label> { draw_text: { color: #9ca3af, text_style: { font_size: 10.0 } } }
                                        }

                                        chat_tile_9 = <RoundedView> {
                                            width: Fill, height: 144
                                            show_bg: true, draw_bg: { color: #ffffff, border_radius: 12.0 }
                                            flow: Down
                                            padding: {top: 16, left: 16, right: 16, bottom: 16}
                                            cursor: Hand
                                            visible: false
                                            <View> {
                                                width: Fill, height: Fit
                                                flow: Right
                                                align: {y: 0.0}
                                                chat_tile_title_9 = <Label> {
                                                    width: Fill
                                                    draw_text: { color: #1f2937, text_style: <FONT_SEMIBOLD>{ font_size: 11.0 }, wrap: Ellipsis }
                                                }
                                                delete_btn_9 = <View> {
                                                    width: 28, height: 28
                                                    align: {x: 0.5, y: 0.5}
                                                    cursor: Hand
                                                    <Icon> { draw_icon: { svg_file: (ICON_TRASH), color: #9ca3af }, icon_walk: {width: 18, height: 18} }
                                                }
                                            }
                                            <View> { width: Fill, height: Fill }
                                            chat_tile_date_9 = <Label> { draw_text: { color: #9ca3af, text_style: { font_size: 10.0 } } }
                                        }

                                        chat_tile_10 = <RoundedView> {
                                            width: Fill, height: 144
                                            show_bg: true, draw_bg: { color: #ffffff, border_radius: 12.0 }
                                            flow: Down
                                            padding: {top: 16, left: 16, right: 16, bottom: 16}
                                            cursor: Hand
                                            visible: false
                                            <View> {
                                                width: Fill, height: Fit
                                                flow: Right
                                                align: {y: 0.0}
                                                chat_tile_title_10 = <Label> {
                                                    width: Fill
                                                    draw_text: { color: #1f2937, text_style: <FONT_SEMIBOLD>{ font_size: 11.0 }, wrap: Ellipsis }
                                                }
                                                delete_btn_10 = <View> {
                                                    width: 28, height: 28
                                                    align: {x: 0.5, y: 0.5}
                                                    cursor: Hand
                                                    <Icon> { draw_icon: { svg_file: (ICON_TRASH), color: #9ca3af }, icon_walk: {width: 18, height: 18} }
                                                }
                                            }
                                            <View> { width: Fill, height: Fill }
                                            chat_tile_date_10 = <Label> { draw_text: { color: #9ca3af, text_style: { font_size: 10.0 } } }
                                        }

                                        chat_tile_11 = <RoundedView> {
                                            width: Fill, height: 144
                                            show_bg: true, draw_bg: { color: #ffffff, border_radius: 12.0 }
                                            flow: Down
                                            padding: {top: 16, left: 16, right: 16, bottom: 16}
                                            cursor: Hand
                                            visible: false
                                            <View> {
                                                width: Fill, height: Fit
                                                flow: Right
                                                align: {y: 0.0}
                                                chat_tile_title_11 = <Label> {
                                                    width: Fill
                                                    draw_text: { color: #1f2937, text_style: <FONT_SEMIBOLD>{ font_size: 11.0 }, wrap: Ellipsis }
                                                }
                                                delete_btn_11 = <View> {
                                                    width: 28, height: 28
                                                    align: {x: 0.5, y: 0.5}
                                                    cursor: Hand
                                                    <Icon> { draw_icon: { svg_file: (ICON_TRASH), color: #9ca3af }, icon_walk: {width: 18, height: 18} }
                                                }
                                            }
                                            <View> { width: Fill, height: Fill }
                                            chat_tile_date_11 = <Label> { draw_text: { color: #9ca3af, text_style: { font_size: 10.0 } } }
                                        }
                                    }
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
                                width: 16, height: Fill  // 0 when collapsed, 16 when expanded
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

                            // Right: Canvas panel (visible by default for A2UI)
                            canvas_section = <View> {
                                width: 500, height: Fill
                                flow: Right
                                visible: true

                                // Toggle column (hidden - canvas visibility controlled by A2UI toggle)
                                canvas_toggle_column = <View> {
                                    visible: false
                                    width: Fit, height: Fill
                                    show_bg: true
                                    draw_bg: { color: #f8fafc }
                                    align: {x: 0.5, y: 0.0}
                                    padding: {left: 4, right: 4, top: 8}

                                    toggle_canvas_btn = <Button> {
                                        width: Fit, height: Fit
                                        padding: {left: 8, right: 8, top: 6, bottom: 6}
                                        text: "<"

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
                                        }

                                        draw_text: {
                                            text_style: <FONT_BOLD>{ font_size: 11.0 }
                                            color: #64748b
                                        }
                                        draw_bg: {
                                            instance hover: 0.0
                                            fn pixel(self) -> vec4 {
                                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                                                let base = #e2e8f0;
                                                let hover_color = #cbd5e1;
                                                let color = mix(base, hover_color, self.hover);
                                                sdf.fill(color);
                                                return sdf.result;
                                            }
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
                        }

                        // Models app
                        models_app = <ModelsApp> {
                            visible: false
                        }

                        // Settings app
                        settings_app = <SettingsApp> {
                            visible: false
                        }

                        // Local Models app
                        local_models_app = <LocalModelsApp> {
                            visible: false
                        }

                        // MCP app (desktop only)
                        mcp_app = <McpApp> {
                            visible: false
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
enum NavigationTarget {
    /// Chat History page - blank page with "Chat History" text
    #[default]
    ChatHistory,
    /// Active chat - shows the chat interface
    ActiveChat,
    Models,
    LocalModels,
    Settings,
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
    /// Current A2UI tool calls received from the model
    #[rust]
    a2ui_tool_calls: Vec<ToolCall>,
}

impl LiveHook for App {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        if !self.initialized {
            // Load Store from disk (this is called after Makepad creates the struct)
            self.store = Store::load();

            // Set current_view from loaded preferences
            self.current_view = match self.store.current_view() {
                "Models" => NavigationTarget::Models,
                "LocalModels" => NavigationTarget::LocalModels,
                "Settings" => NavigationTarget::Settings,
                "ActiveChat" => NavigationTarget::ActiveChat,
                _ => NavigationTarget::ChatHistory,
            };

            // Initialize MolyAppData from Store preferences
            self.app_data = MolyAppData::new(self.store.is_dark_mode());
            self.app_data.sync_from_preferences(
                self.store.is_dark_mode(),
                self.store.is_sidebar_expanded(),
                self.store.current_view(),
                self.store.preferences.get_current_chat_model(),
            );
            // Snap theme to target (no animation on startup)
            self.app_data.theme.snap_to_target();

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
        <moly_models::MolyModelsApp as MolyApp>::live_design(cx);
        <moly_settings::MolySettingsApp as MolyApp>::live_design(cx);
        <moly_mcp::MolyMcpApp as MolyApp>::live_design(cx);
        <moly_local_models::MolyLocalModelsApp as MolyApp>::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        // Apply initial state from Store (no animation on startup)
        self.apply_theme_animation(cx);
        self.update_sidebar(cx);
        // Force apply view state on startup (bypass same-view check)
        self.apply_view_state(cx, self.current_view);
        // Initialize canvas panel width
        self.canvas_panel_width = 500.0;
        ::log::info!("App initialized with Store and MolyAppData");
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Handle hamburger menu click
        if self.ui.view(ids!(body.header.hamburger_btn)).finger_down(&actions).is_some() {
            ::log::info!(">>> Hamburger button clicked! <<<");
            self.store.toggle_sidebar();
            self.update_sidebar(cx);
        }

        // Handle theme toggle click
        if self.ui.view(ids!(body.header.theme_toggle)).finger_down(&actions).is_some() {
            ::log::info!(">>> Theme toggle clicked! <<<");
            self.store.toggle_dark_mode();
            self.app_data.theme.toggle_dark_mode();
            // Start animation
            cx.new_next_frame();
        }

        // Handle New Chat button click (first item in sidebar)
        // Use full path from Window root: body.content.sidebar.new_chat_btn
        let new_chat_clicked = self.ui.button(ids!(body.content.sidebar.new_chat_btn)).clicked(&actions);
        let chat_clicked = self.ui.button(ids!(body.content.sidebar.chat_section.chat_btn)).clicked(&actions);

        if new_chat_clicked {
            ::log::info!(">>> New Chat button clicked! <<<");

            // Request new chat directly on ChatApp (bypasses action dispatch timing issues)
            if let Some(mut chat_app) = self.ui.widget(ids!(body.content.main_content.chat_with_canvas.chat_app))
                .borrow_mut::<moly_chat::screen::ChatApp>()
            {
                chat_app.request_new_chat();
            }

            // Always show active chat view when creating new chat
            self.current_view = NavigationTarget::ActiveChat;
            self.store.set_current_view("ActiveChat");
            self.apply_view_state(cx, NavigationTarget::ActiveChat);
        } else if chat_clicked {
            ::log::info!("Chat button clicked - opening chat history page");
            // Navigate to chat history page (blank page with "Chat History" text)
            self.navigate_to(cx, NavigationTarget::ChatHistory);
        }

        // Handle Show More button click
        if self.ui.view(ids!(body.content.sidebar.chat_section.chat_history_visible.show_more_btn)).finger_down(&actions).is_some() {
            self.chat_history_expanded = !self.chat_history_expanded;
            self.update_chat_history_visibility(cx);
        }
        if self.ui.button(ids!(body.content.sidebar.models_btn)).clicked(&actions) {
            ::log::info!(">>> Models button clicked! <<<");
            self.navigate_to(cx, NavigationTarget::Models);
        }
        if self.ui.button(ids!(body.content.sidebar.local_models_btn)).clicked(&actions) {
            ::log::info!(">>> Local Models button clicked! <<<");
            self.navigate_to(cx, NavigationTarget::LocalModels);
        }
        if self.ui.button(ids!(body.content.sidebar.settings_btn)).clicked(&actions) {
            ::log::info!(">>> Settings button clicked! <<<");
            self.navigate_to(cx, NavigationTarget::Settings);
        }

        // Handle chat tile clicks
        self.handle_chat_tile_clicks(cx, actions);

        // Handle search input changes
        let search_input = self.ui.text_input(ids!(body.content.main_content.chat_history_page.search_container.search_input));
        if search_input.changed(&actions).is_some() {
            self.search_query = search_input.text();
            self.update_chat_tiles(cx);
        }

        // Handle canvas panel toggle button
        if self.ui.button(ids!(body.content.main_content.chat_with_canvas.canvas_section.canvas_toggle_column.toggle_canvas_btn)).clicked(&actions) {
            ::log::info!(">>> Canvas toggle button clicked! <<<");
            self.toggle_canvas_panel(cx);
        }

        // Handle canvas splitter drag start
        let splitter = self.ui.view(ids!(body.content.main_content.chat_with_canvas.canvas_splitter));
        if let Some(fd) = splitter.finger_down(&actions) {
            if !self.canvas_panel_collapsed {
                self.splitter_dragging = true;
                self.splitter_drag_start_x = fd.abs.x;
                self.splitter_drag_start_width = self.canvas_panel_width;
                ::log::info!("Splitter drag started at x={}", fd.abs.x);
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
                    // Clear A2UI tool calls when disabled
                    self.a2ui_tool_calls.clear();
                }
            }

            // Handle A2UI tool calls from Chat widget
            match action.cast() {
                ChatAction::A2uiToolCalls(tool_calls) => {
                    ::log::info!(
                        "Received {} A2UI tool calls",
                        tool_calls.len()
                    );
                    self.a2ui_tool_calls = tool_calls;
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

        // Poll global state for pending A2UI tool calls
        // (action propagation from nested Chat widget doesn't reach here)
        let pending = take_pending_a2ui_tool_calls();
        if !pending.is_empty() {
            ::log::info!(
                "Picked up {} pending A2UI tool calls from global state",
                pending.len()
            );
            self.a2ui_tool_calls = pending;
            self.render_a2ui_canvas(cx);
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        // Handle theme animation on NextFrame
        if let Event::NextFrame(_) = event {
            if self.app_data.theme.animate_step(cx) {
                self.apply_theme_animation(cx);
            }
        }

        // Handle splitter dragging with global mouse events
        if self.splitter_dragging {
            match event {
                Event::MouseMove(mm) => {
                    // Dragging left (negative delta) should increase canvas width
                    // Dragging right (positive delta) should decrease canvas width
                    let delta = mm.abs.x - self.splitter_drag_start_x;
                    let new_width = (self.splitter_drag_start_width - delta).max(200.0).min(1200.0);
                    self.canvas_panel_width = new_width;

                    self.ui.view(ids!(body.content.main_content.chat_with_canvas.canvas_section))
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
    fn navigate_to(&mut self, cx: &mut Cx, target: NavigationTarget) {
        ::log::info!("navigate_to: current={:?}, target={:?}", self.current_view, target);
        if self.current_view == target {
            ::log::info!("navigate_to: same view, skipping");
            return;
        }

        self.current_view = target;

        // Persist to Store
        let view_name = match target {
            NavigationTarget::ChatHistory => "ChatHistory",
            NavigationTarget::ActiveChat => "ActiveChat",
            NavigationTarget::Models => "Models",
            NavigationTarget::LocalModels => "LocalModels",
            NavigationTarget::Settings => "Settings",
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

        self.ui.widget(ids!(body.content.main_content.chat_history_page)).set_visible(cx, show_chat_history);
        self.ui.widget(ids!(body.content.main_content.chat_with_canvas)).set_visible(cx, show_active_chat);
        self.ui.widget(ids!(body.content.main_content.models_app)).set_visible(cx, target == NavigationTarget::Models);
        self.ui.widget(ids!(body.content.main_content.local_models_app)).set_visible(cx, target == NavigationTarget::LocalModels);
        self.ui.widget(ids!(body.content.main_content.settings_app)).set_visible(cx, target == NavigationTarget::Settings);

        // Notify ChatApp when it becomes visible (to refresh model list)
        if show_active_chat {
            if let Some(mut chat_app) = self.ui.widget(ids!(body.content.main_content.chat_with_canvas.chat_app)).borrow_mut::<moly_chat::screen::ChatApp>() {
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
        self.ui.button(ids!(body.content.sidebar.chat_section.chat_btn)).apply_over(cx, live! {
            draw_bg: { selected: (if chat_selected { 1.0 } else { 0.0 }) }
        });
        self.ui.button(ids!(body.content.sidebar.models_btn)).apply_over(cx, live! {
            draw_bg: { selected: (if target == NavigationTarget::Models { 1.0 } else { 0.0 }) }
        });
        self.ui.button(ids!(body.content.sidebar.local_models_btn)).apply_over(cx, live! {
            draw_bg: { selected: (if target == NavigationTarget::LocalModels { 1.0 } else { 0.0 }) }
        });
        self.ui.button(ids!(body.content.sidebar.settings_btn)).apply_over(cx, live! {
            draw_bg: { selected: (if target == NavigationTarget::Settings { 1.0 } else { 0.0 }) }
        });

        self.ui.redraw(cx);
    }

    fn update_sidebar(&mut self, cx: &mut Cx) {
        let expanded = self.store.is_sidebar_expanded();
        let width = if expanded { 250.0 } else { 60.0 };

        self.ui.view(ids!(body.content.sidebar)).apply_over(cx, live! {
            width: (width)
        });

        // Note: With SidebarButton (Button widget), text is drawn by draw_text and can't be hidden separately.
        // When sidebar is collapsed (width: 60), the text will be clipped automatically.
        // This is a common pattern in modern apps where collapsed sidebars show only icons.

        // Show/hide chat history based on sidebar state
        self.ui.view(ids!(body.content.sidebar.chat_section.chat_history_visible)).set_visible(cx, expanded);

        self.ui.redraw(cx);
    }

    /// Update chat history visibility based on expanded state
    fn update_chat_history_visibility(&mut self, cx: &mut Cx) {
        // Update "Show More" section visibility
        self.ui.view(ids!(body.content.sidebar.chat_section.chat_history_more)).set_visible(cx, self.chat_history_expanded);

        // Update "Show More" button text and arrow
        let (text, arrow) = if self.chat_history_expanded {
            ("Show Less", "v")
        } else {
            ("Show More", ">")
        };
        self.ui.label(ids!(body.content.sidebar.chat_section.chat_history_visible.show_more_label)).set_text(cx, text);
        self.ui.label(ids!(body.content.sidebar.chat_section.chat_history_visible.show_more_arrow)).set_text(cx, arrow);

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
            // Collapse: hide entire canvas section
            self.ui.view(ids!(body.content.main_content.chat_with_canvas.canvas_section))
                .set_visible(cx, false);
            self.ui.view(ids!(body.content.main_content.chat_with_canvas.canvas_splitter))
                .apply_over(cx, live!{ width: 0 });
        } else {
            // Expand: show canvas section at saved width
            let width = self.canvas_panel_width;
            self.ui.view(ids!(body.content.main_content.chat_with_canvas.canvas_section))
                .set_visible(cx, true);
            self.ui.view(ids!(body.content.main_content.chat_with_canvas.canvas_section))
                .apply_over(cx, live!{ width: (width) });
            self.ui.view(ids!(body.content.main_content.chat_with_canvas.canvas_section.canvas_content))
                .set_visible(cx, true);
            self.ui.view(ids!(body.content.main_content.chat_with_canvas.canvas_splitter))
                .apply_over(cx, live!{ width: 16 });
        }

        self.ui.redraw(cx);
    }

    /// Render A2UI components in the canvas area based on received tool calls.
    ///
    /// Converts tool calls to A2UI JSON protocol and feeds to A2uiSurface.
    fn render_a2ui_canvas(&mut self, cx: &mut Cx) {
        if self.a2ui_tool_calls.is_empty() {
            return;
        }

        // Convert tool calls to A2UI JSON using the builder
        let mut builder = A2uiBuilder::new();
        for tool_call in &self.a2ui_tool_calls {
            let args = serde_json::Value::Object(
                tool_call.arguments.clone(),
            );
            builder.process_tool_call(&tool_call.name, &args);
        }
        let a2ui_json = builder.build_a2ui_json();
        let json_str = serde_json::to_string(&a2ui_json)
            .unwrap_or_default();

        ::log::info!(
            "A2UI JSON ({} tool calls): {}",
            self.a2ui_tool_calls.len(),
            &json_str[..json_str.len().min(500)]
        );

        // Feed JSON to A2uiSurface for rendering
        let surface_ref = self.ui.widget(ids!(
            body.content.main_content.chat_with_canvas
                .canvas_section.canvas_content
                .canvas_area.a2ui_surface
        ));
        if let Some(mut surface) = surface_ref.borrow_mut::<A2uiSurface>() {
            surface.clear();
            match surface.process_json(&json_str) {
                Ok(events) => {
                    ::log::info!(
                        "A2uiSurface processed {} events",
                        events.len()
                    );
                }
                Err(e) => {
                    ::log::error!("A2uiSurface JSON parse error: {}", e);
                }
            }
        } else {
            ::log::error!("Could not borrow A2uiSurface");
        }

        self.ui.redraw(cx);
    }

    /// Apply animated theme value to all UI elements
    /// Called each frame during theme transition
    /// Note: Currently using static light mode colors. Dark mode can be implemented
    /// by swapping color values or using a different theming approach.
    fn apply_theme_animation(&mut self, cx: &mut Cx) {
        // Theme animation currently disabled - using static colors
        // External app widgets (chat_app, models_app, etc.) handle their own theming
        // through the Store/preferences
        let _ = self.app_data.theme.dark_mode_anim; // Silence unused warning
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
        self.ui.view(ids!(body.content.main_content.chat_history_page.empty_state)).set_visible(cx, !has_chats);
        self.ui.view(ids!(body.content.main_content.chat_history_page.chat_tiles_scroll)).set_visible(cx, has_chats);

        // Show/hide row containers based on how many chats we have
        // Row 0 visible if we have any chats (indices 0-3)
        // Row 1 visible if we have more than 4 chats (indices 4-7)
        // Row 2 visible if we have more than 8 chats (indices 8-11)
        self.ui.view(ids!(body.content.main_content.chat_history_page.chat_tiles_scroll.chat_tiles_container.tile_row_0))
            .set_visible(cx, chat_count > 0);
        self.ui.view(ids!(body.content.main_content.chat_history_page.chat_tiles_scroll.chat_tiles_container.tile_row_1))
            .set_visible(cx, chat_count > 4);
        self.ui.view(ids!(body.content.main_content.chat_history_page.chat_tiles_scroll.chat_tiles_container.tile_row_2))
            .set_visible(cx, chat_count > 8);

        // Helper macro to update a single tile (tiles are now nested in rows)
        macro_rules! update_tile {
            ($index:expr, $row:ident, $tile:ident, $title:ident, $date:ident) => {
                let visible = $index < chat_count;
                self.ui.view(ids!(body.content.main_content.chat_history_page.chat_tiles_scroll.chat_tiles_container.$row.$tile))
                    .set_visible(cx, visible);
                if visible {
                    let chat = chats[$index];
                    self.ui.label(ids!(body.content.main_content.chat_history_page.chat_tiles_scroll.chat_tiles_container.$row.$tile.$title))
                        .set_text(cx, &chat.title);
                    let date_str = chat.accessed_at.format("%b %d, %Y").to_string();
                    self.ui.label(ids!(body.content.main_content.chat_history_page.chat_tiles_scroll.chat_tiles_container.$row.$tile.$date))
                        .set_text(cx, &date_str);
                }
            };
        }

        // Update all 12 tiles (4 tiles per row, 3 rows)
        // Row 0: tiles 0-3
        update_tile!(0, tile_row_0, chat_tile_0, chat_tile_title_0, chat_tile_date_0);
        update_tile!(1, tile_row_0, chat_tile_1, chat_tile_title_1, chat_tile_date_1);
        update_tile!(2, tile_row_0, chat_tile_2, chat_tile_title_2, chat_tile_date_2);
        update_tile!(3, tile_row_0, chat_tile_3, chat_tile_title_3, chat_tile_date_3);
        // Row 1: tiles 4-7
        update_tile!(4, tile_row_1, chat_tile_4, chat_tile_title_4, chat_tile_date_4);
        update_tile!(5, tile_row_1, chat_tile_5, chat_tile_title_5, chat_tile_date_5);
        update_tile!(6, tile_row_1, chat_tile_6, chat_tile_title_6, chat_tile_date_6);
        update_tile!(7, tile_row_1, chat_tile_7, chat_tile_title_7, chat_tile_date_7);
        // Row 2: tiles 8-11
        update_tile!(8, tile_row_2, chat_tile_8, chat_tile_title_8, chat_tile_date_8);
        update_tile!(9, tile_row_2, chat_tile_9, chat_tile_title_9, chat_tile_date_9);
        update_tile!(10, tile_row_2, chat_tile_10, chat_tile_title_10, chat_tile_date_10);
        update_tile!(11, tile_row_2, chat_tile_11, chat_tile_title_11, chat_tile_date_11);

        self.ui.redraw(cx);
    }

    /// Handle chat tile clicks and delete button clicks
    fn handle_chat_tile_clicks(&mut self, cx: &mut Cx, actions: &Actions) {
        let mut tile_clicked: Option<usize> = None;
        let mut delete_clicked: Option<usize> = None;

        // Helper macro to check a single tile (tiles are now nested in rows)
        macro_rules! check_tile {
            ($index:expr, $row:ident, $tile:ident, $delete_btn:ident) => {
                if $index < self.displayed_chat_ids.len() && delete_clicked.is_none() && tile_clicked.is_none() {
                    // Check delete button first
                    if self.ui.view(ids!(body.content.main_content.chat_history_page.chat_tiles_scroll.chat_tiles_container.$row.$tile.$delete_btn))
                        .finger_down(actions).is_some() {
                        delete_clicked = Some($index);
                    }
                    // Check tile click
                    else if self.ui.view(ids!(body.content.main_content.chat_history_page.chat_tiles_scroll.chat_tiles_container.$row.$tile))
                        .finger_down(actions).is_some() {
                        tile_clicked = Some($index);
                    }
                }
            };
        }

        // Check all 12 tiles (4 tiles per row, 3 rows)
        // Row 0: tiles 0-3
        check_tile!(0, tile_row_0, chat_tile_0, delete_btn_0);
        check_tile!(1, tile_row_0, chat_tile_1, delete_btn_1);
        check_tile!(2, tile_row_0, chat_tile_2, delete_btn_2);
        check_tile!(3, tile_row_0, chat_tile_3, delete_btn_3);
        // Row 1: tiles 4-7
        check_tile!(4, tile_row_1, chat_tile_4, delete_btn_4);
        check_tile!(5, tile_row_1, chat_tile_5, delete_btn_5);
        check_tile!(6, tile_row_1, chat_tile_6, delete_btn_6);
        check_tile!(7, tile_row_1, chat_tile_7, delete_btn_7);
        // Row 2: tiles 8-11
        check_tile!(8, tile_row_2, chat_tile_8, delete_btn_8);
        check_tile!(9, tile_row_2, chat_tile_9, delete_btn_9);
        check_tile!(10, tile_row_2, chat_tile_10, delete_btn_10);
        check_tile!(11, tile_row_2, chat_tile_11, delete_btn_11);

        // Handle delete action
        if let Some(idx) = delete_clicked {
            let chat_id = self.displayed_chat_ids[idx];
            ::log::info!("Delete button clicked for chat at index {}, id={}", idx, chat_id);
            self.store.chats.delete_chat(chat_id);
            self.update_chat_tiles(cx);
            return;
        }

        // Handle tile click (open chat)
        if let Some(idx) = tile_clicked {
            let chat_id = self.displayed_chat_ids[idx];
            ::log::info!("Chat tile clicked at index {}, id={}", idx, chat_id);

            // Set current chat in store
            self.store.chats.set_current_chat(Some(chat_id));

            // Load chat in ChatApp
            if let Some(mut chat_app) = self.ui.widget(ids!(body.content.main_content.chat_with_canvas.chat_app))
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
