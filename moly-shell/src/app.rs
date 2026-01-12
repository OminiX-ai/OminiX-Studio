use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use moly_widgets::theme::*;

    // Icon dependencies
    ICON_HAMBURGER = dep("crate://self/resources/icons/hamburger.svg")
    ICON_SUN = dep("crate://self/resources/icons/sun.svg")
    ICON_MOON = dep("crate://self/resources/icons/moon.svg")

    // Navigation button style
    NavButton = <Button> {
        width: Fill, height: 48
        margin: {bottom: 4}
        padding: {left: 16, right: 16}
        align: {x: 0.0, y: 0.5}

        draw_bg: {
            instance hover: 0.0
            instance selected: 0.0
            instance dark_mode: 0.0

            fn get_bg_color(self) -> vec4 {
                let base_color = mix(#ffffff, #1f293b, self.dark_mode);
                let hover_color = mix(#f1f5f9, #334155, self.dark_mode);
                let selected_color = mix(#e0e7ff, #4338ca, self.dark_mode);

                return mix(
                    mix(base_color, hover_color, self.hover),
                    selected_color,
                    self.selected
                );
            }

            fn pixel(self) -> vec4 {
                return Pal::premul(self.get_bg_color());
            }
        }

        draw_text: {
            instance dark_mode: 0.0
            fn get_color(self) -> vec4 {
                return mix(#1f2937, #f1f5f9, self.dark_mode);
            }
            text_style: { font_size: 13.0 }
        }
    }

    App = {{App}} {
        ui: <Window> {
            window: { title: "Moly", inner_size: vec2(1400, 900) }
            pass: {
                clear_color: #f5f7fa
            }

            body = <View> {
                width: Fill, height: Fill
                flow: Down
                show_bg: true
                draw_bg: {
                    instance dark_mode: 0.0
                    fn pixel(self) -> vec4 {
                        return mix(#f5f7fa, #0f172a, self.dark_mode);
                    }
                }

                // Header
                header = <View> {
                    width: Fill, height: 72
                    flow: Right
                    align: {y: 0.5}
                    padding: {left: 20, right: 20, top: 16}
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 {
                            return mix(#ffffff, #1f293b, self.dark_mode);
                        }
                    }

                    // Hamburger menu button
                    hamburger_btn = <View> {
                        width: 40, height: Fit
                        margin: {right: 12}
                        align: {x: 0.5, y: 0.5}
                        cursor: Hand

                        hamburger_icon = <Icon> {
                            draw_icon: {
                                svg_file: (ICON_HAMBURGER)
                                instance dark_mode: 0.0
                                fn get_color(self) -> vec4 {
                                    return mix(#6b7280, #cbd5e1, self.dark_mode);
                                }
                            }
                            icon_walk: {width: 20, height: 20}
                        }
                    }

                    title_label = <Label> {
                        text: "Moly"
                        draw_text: {
                            instance dark_mode: 0.0
                            fn get_color(self) -> vec4 {
                                return mix(#1f2937, #f1f5f9, self.dark_mode);
                            }
                            text_style: <THEME_FONT_BOLD>{ font_size: 24.0 }
                        }
                    }

                    <View> { width: Fill } // Spacer

                    // Theme toggle button
                    theme_toggle = <View> {
                        width: 40, height: Fit
                        align: {x: 0.5, y: 0.5}
                        cursor: Hand

                        theme_icon = <Icon> {
                            draw_icon: {
                                svg_file: (ICON_SUN)
                                instance dark_mode: 0.0
                                fn get_color(self) -> vec4 {
                                    return mix(#f59e0b, #fbbf24, self.dark_mode);
                                }
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
                            instance dark_mode: 0.0
                            fn pixel(self) -> vec4 {
                                return mix(#ffffff, #1f293b, self.dark_mode);
                            }
                        }
                        flow: Down, padding: {top: 16, bottom: 16, left: 8, right: 8}

                        chat_btn = <NavButton> {
                            text: "Chat"
                        }
                        models_btn = <NavButton> {
                            text: "Models"
                        }
                        settings_btn = <NavButton> {
                            text: "Settings"
                        }
                    }

                    // Main content views
                    main_content = <View> {
                        width: Fill, height: Fill
                        flow: Overlay

                        // Chat view
                        chat_view = <View> {
                            visible: true
                            width: Fill, height: Fill
                            show_bg: true
                            draw_bg: {
                                instance dark_mode: 0.0
                                fn pixel(self) -> vec4 {
                                    return mix(#f5f7fa, #0f172a, self.dark_mode);
                                }
                            }
                            flow: Down, align: {x: 0.5, y: 0.5}

                            <Label> {
                                text: "Chat View"
                                draw_text: {
                                    instance dark_mode: 0.0
                                    fn get_color(self) -> vec4 {
                                        return mix(#1f2937, #f1f5f9, self.dark_mode);
                                    }
                                    text_style: <THEME_FONT_BOLD>{ font_size: 32.0 }
                                }
                            }
                            <Label> {
                                margin: {top: 8}
                                text: "Phase 1 Complete - Ready for Phase 2"
                                draw_text: {
                                    instance dark_mode: 0.0
                                    fn get_color(self) -> vec4 {
                                        return mix(#6b7280, #94a3b8, self.dark_mode);
                                    }
                                    text_style: { font_size: 14.0 }
                                }
                            }
                        }

                        // Models view
                        models_view = <View> {
                            visible: false
                            width: Fill, height: Fill
                            show_bg: true
                            draw_bg: {
                                instance dark_mode: 0.0
                                fn pixel(self) -> vec4 {
                                    return mix(#f5f7fa, #0f172a, self.dark_mode);
                                }
                            }
                            flow: Down, align: {x: 0.5, y: 0.5}

                            <Label> {
                                text: "Models View"
                                draw_text: {
                                    instance dark_mode: 0.0
                                    fn get_color(self) -> vec4 {
                                        return mix(#1f2937, #f1f5f9, self.dark_mode);
                                    }
                                    text_style: <THEME_FONT_BOLD>{ font_size: 32.0 }
                                }
                            }
                        }

                        // Settings view
                        settings_view = <View> {
                            visible: false
                            width: Fill, height: Fill
                            show_bg: true
                            draw_bg: {
                                instance dark_mode: 0.0
                                fn pixel(self) -> vec4 {
                                    return mix(#f5f7fa, #0f172a, self.dark_mode);
                                }
                            }
                            flow: Down, align: {x: 0.5, y: 0.5}

                            <Label> {
                                text: "Settings View"
                                draw_text: {
                                    instance dark_mode: 0.0
                                    fn get_color(self) -> vec4 {
                                        return mix(#1f2937, #f1f5f9, self.dark_mode);
                                    }
                                    text_style: <THEME_FONT_BOLD>{ font_size: 32.0 }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
enum NavigationTarget {
    #[default]
    Chat,
    Models,
    Settings,
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    current_view: NavigationTarget,
    #[rust]
    dark_mode: bool,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        moly_widgets::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Handle theme toggle click
        if self.ui.view(ids!(theme_toggle)).finger_down(&actions).is_some() {
            self.dark_mode = !self.dark_mode;
            self.update_theme(cx);
        }

        // Handle navigation
        if self.ui.button(ids!(chat_btn)).clicked(&actions) {
            self.navigate_to(cx, NavigationTarget::Chat);
        }
        if self.ui.button(ids!(models_btn)).clicked(&actions) {
            self.navigate_to(cx, NavigationTarget::Models);
        }
        if self.ui.button(ids!(settings_btn)).clicked(&actions) {
            self.navigate_to(cx, NavigationTarget::Settings);
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

impl App {
    fn navigate_to(&mut self, cx: &mut Cx, target: NavigationTarget) {
        if self.current_view == target {
            return;
        }

        self.current_view = target;

        // Update view visibility
        self.ui.view(ids!(chat_view)).set_visible(cx, target == NavigationTarget::Chat);
        self.ui.view(ids!(models_view)).set_visible(cx, target == NavigationTarget::Models);
        self.ui.view(ids!(settings_view)).set_visible(cx, target == NavigationTarget::Settings);

        // Update button selection state
        self.ui.button(ids!(chat_btn)).apply_over(cx, live! {
            draw_bg: { selected: (if target == NavigationTarget::Chat { 1.0 } else { 0.0 }) }
        });
        self.ui.button(ids!(models_btn)).apply_over(cx, live! {
            draw_bg: { selected: (if target == NavigationTarget::Models { 1.0 } else { 0.0 }) }
        });
        self.ui.button(ids!(settings_btn)).apply_over(cx, live! {
            draw_bg: { selected: (if target == NavigationTarget::Settings { 1.0 } else { 0.0 }) }
        });

        self.ui.redraw(cx);
    }

    fn update_theme(&mut self, cx: &mut Cx) {
        let dark_mode_value = if self.dark_mode { 1.0 } else { 0.0 };

        // Update all dark_mode instances
        self.ui.view(ids!(body)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode_value) }
        });
        self.ui.view(ids!(header)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode_value) }
        });

        // Update header icons and text
        self.ui.icon(ids!(hamburger_btn.hamburger_icon)).apply_over(cx, live! {
            draw_icon: { dark_mode: (dark_mode_value) }
        });
        self.ui.label(ids!(title_label)).apply_over(cx, live! {
            draw_text: { dark_mode: (dark_mode_value) }
        });
        self.ui.icon(ids!(theme_toggle.theme_icon)).apply_over(cx, live! {
            draw_icon: { dark_mode: (dark_mode_value) }
        });

        self.ui.view(ids!(sidebar)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode_value) }
        });

        // Update navigation buttons
        self.ui.button(ids!(chat_btn)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode_value) }
            draw_text: { dark_mode: (dark_mode_value) }
        });
        self.ui.button(ids!(models_btn)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode_value) }
            draw_text: { dark_mode: (dark_mode_value) }
        });
        self.ui.button(ids!(settings_btn)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode_value) }
            draw_text: { dark_mode: (dark_mode_value) }
        });

        // Update content views
        self.ui.view(ids!(chat_view)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode_value) }
        });
        self.ui.view(ids!(models_view)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode_value) }
        });
        self.ui.view(ids!(settings_view)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode_value) }
        });

        self.ui.redraw(cx);
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            ui: WidgetRef::default(),
            current_view: NavigationTarget::Chat,
            dark_mode: false,
        }
    }
}

app_main!(App);
