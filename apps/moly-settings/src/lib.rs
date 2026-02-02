//! Moly Settings App
//!
//! Provider configuration and application settings.

pub mod screen;

use makepad_widgets::{Cx, live_id, LiveId};
use moly_widgets::{MolyApp, AppInfo};

pub use screen::{SettingsApp, SettingsAppRef};

/// Main app struct for MolyApp trait implementation
pub struct MolySettingsApp;

impl MolyApp for MolySettingsApp {
    fn info() -> AppInfo {
        AppInfo {
            name: "Settings",
            id: "moly-settings",
            description: "Provider configuration and app settings",
            icon: live_id!(IconSettings),
            page_id: live_id!(settings_app),
        }
    }

    fn live_design(cx: &mut Cx) {
        // Note: makepad_component is registered by moly-kit
        crate::screen::design::live_design(cx);
    }
}
