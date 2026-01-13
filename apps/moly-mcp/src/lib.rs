use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use moly_widgets::theme::*;

    pub McpApp = {{McpApp}} {
        width: Fill, height: Fill
        flow: Down, align: {x: 0.5, y: 0.5}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix(#f5f7fa, #0f172a, self.dark_mode);
            }
        }

        <Label> {
            text: "MCP App"
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
            text: "Model Context Protocol (Desktop Only)"
            draw_text: {
                instance dark_mode: 0.0
                fn get_color(self) -> vec4 {
                    return mix(#6b7280, #94a3b8, self.dark_mode);
                }
                text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct McpApp {
    #[deref]
    pub view: View,
}

impl Widget for McpApp {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
