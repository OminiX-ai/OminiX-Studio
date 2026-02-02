//! Moly Models App
//!
//! Model discovery and downloads application.

pub mod screen;

use makepad_widgets::{Cx, live_id, LiveId};
use moly_widgets::{MolyApp, AppInfo};

pub use screen::{ModelsApp, ModelsAppRef};

/// Main app struct for MolyApp trait implementation
pub struct MolyModelsApp;

impl MolyApp for MolyModelsApp {
    fn info() -> AppInfo {
        AppInfo {
            name: "Models",
            id: "moly-models",
            description: "Model discovery and downloads",
            icon: live_id!(IconModels),
            page_id: live_id!(models_app),
        }
    }

    fn live_design(cx: &mut Cx) {
        crate::screen::design::live_design(cx);
    }
}
