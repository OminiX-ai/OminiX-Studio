//! Moly Chat App
//!
//! Chat application with multi-provider support and chat history persistence.

pub mod screen;

use makepad_widgets::{Cx, live_id, LiveId};
use moly_widgets::{MolyApp, AppInfo};

pub use screen::{ChatApp, ChatAppRef, ChatHistoryAction};

/// Main app struct for MolyApp trait implementation
pub struct MolyChatApp;

impl MolyApp for MolyChatApp {
    fn info() -> AppInfo {
        AppInfo {
            name: "Chat",
            id: "moly-chat",
            description: "AI chat interface with multi-provider support",
            icon: live_id!(IconChat),
            page_id: live_id!(chat_app),
        }
    }

    fn live_design(cx: &mut Cx) {
        crate::screen::design::live_design(cx);
    }
}
