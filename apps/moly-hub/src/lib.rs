pub mod screen;

use makepad_widgets::{Cx, live_id, LiveId};
use moly_widgets::{MolyApp, AppInfo};

pub use screen::ModelHubApp;

pub struct MolyHubApp;

impl MolyApp for MolyHubApp {
    fn info() -> AppInfo {
        AppInfo {
            name: "Model Hub",
            id: "moly-hub",
            description: "Download, manage, and run MLX models locally",
            icon: live_id!(IconHub),
            page_id: live_id!(hub_app),
        }
    }

    fn live_design(cx: &mut Cx) {
        crate::screen::design::live_design(cx);
    }
}
