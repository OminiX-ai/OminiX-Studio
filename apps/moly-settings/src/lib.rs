//! Moly Settings App
//!
//! Provider configuration and application settings.

pub mod screen;

use makepad_widgets::Cx;
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
        }
    }

    fn live_design(cx: &mut Cx) {
        crate::screen::design::live_design(cx);
    }
}
