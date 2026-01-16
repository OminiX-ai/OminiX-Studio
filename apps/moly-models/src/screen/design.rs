//! Models Screen UI Design

use makepad_widgets::*;

use super::ModelsApp;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use moly_widgets::theme::*;

    pub ModelsApp = {{ModelsApp}} {
        width: Fill, height: Fill
        flow: Down, align: {x: 0.5, y: 0.5}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix(#f5f7fa, #0f172a, self.dark_mode);
            }
        }

        title_label = <Label> {
            text: "Models App"
            draw_text: {
                instance dark_mode: 0.0
                fn get_color(self) -> vec4 {
                    return mix(#1f2937, #f1f5f9, self.dark_mode);
                }
                text_style: <THEME_FONT_BOLD>{ font_size: 32.0 }
            }
        }
        subtitle_label = <Label> {
            margin: {top: 8}
            text: "Model Discovery & Downloads"
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
