//! MCP Screen UI Design

use makepad_widgets::*;

use super::McpApp;

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
            fn pixel(self) -> vec4 {
                return #f5f7fa;
            }
        }

        title_label = <Label> {
            text: "MCP App"
            draw_text: {
                fn get_color(self) -> vec4 {
                    return #1f2937;
                }
                text_style: <FONT_SEMIBOLD>{ font_size: 32.0 }
            }
        }
        subtitle_label = <Label> {
            margin: {top: 8}
            text: "Model Context Protocol (Desktop Only)"
            draw_text: {
                fn get_color(self) -> vec4 {
                    return #6b7280;
                }
                text_style: <FONT_REGULAR>{ font_size: 14.0 }
            }
        }
    }
}
