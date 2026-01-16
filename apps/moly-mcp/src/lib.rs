//! Moly MCP App
//!
//! Model Context Protocol integration (Desktop Only).

pub mod screen;

use makepad_widgets::Cx;
use moly_widgets::{MolyApp, AppInfo};

pub use screen::{McpApp, McpAppRef};

/// Main app struct for MolyApp trait implementation
pub struct MolyMcpApp;

impl MolyApp for MolyMcpApp {
    fn info() -> AppInfo {
        AppInfo {
            name: "MCP",
            id: "moly-mcp",
            description: "Model Context Protocol (Desktop Only)",
        }
    }

    fn live_design(cx: &mut Cx) {
        crate::screen::design::live_design(cx);
    }
}
