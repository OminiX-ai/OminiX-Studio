//! Models Screen Widget Implementation

pub mod design;

use makepad_widgets::*;
use moly_data::Store;

#[derive(Live, LiveHook, Widget)]
pub struct ModelsApp {
    #[deref]
    pub view: View,
}

impl Widget for ModelsApp {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Access Store from Scope to get dark mode state
        if let Some(store) = scope.data.get::<Store>() {
            let dark_mode_value = if store.is_dark_mode() { 1.0 } else { 0.0 };

            self.view.apply_over(cx, live! {
                draw_bg: { dark_mode: (dark_mode_value) }
            });
            self.view.label(ids!(title_label)).apply_over(cx, live! {
                draw_text: { dark_mode: (dark_mode_value) }
            });
            self.view.label(ids!(subtitle_label)).apply_over(cx, live! {
                draw_text: { dark_mode: (dark_mode_value) }
            });
        }

        self.view.draw_walk(cx, scope, walk)
    }
}
