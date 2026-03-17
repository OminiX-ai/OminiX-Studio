//! MCP Screen Widget Implementation

pub mod design;

use makepad_widgets::*;

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
