pub mod screen;

use makepad_widgets::{Cx, live_id, LiveId};
use moly_widgets::{MolyApp, AppInfo};

pub use screen::{LocalModelsApp, LocalModelsAppRef};

pub struct MolyLocalModelsApp;

impl MolyApp for MolyLocalModelsApp {
    fn info() -> AppInfo {
        AppInfo {
            name: "Local Models",
            id: "moly-local-models",
            description: "Manage local OminiX-MLX models",
            icon: live_id!(IconLocalModels),
            page_id: live_id!(local_models_app),
        }
    }

    fn live_design(cx: &mut Cx) {
        crate::screen::design::live_design(cx);
    }
}
