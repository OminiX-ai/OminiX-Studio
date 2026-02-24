pub mod screen;

use makepad_widgets::{Cx, live_id, LiveId};
use moly_widgets::{MolyApp, AppInfo};

pub use screen::{VoiceApp, VoiceAppRef};

pub struct MolyVoiceApp;

impl MolyApp for MolyVoiceApp {
    fn info() -> AppInfo {
        AppInfo {
            name: "Voice",
            id: "moly-voice",
            description: "Clone and synthesize voices",
            icon: live_id!(IconVoice),
            page_id: live_id!(voice_app),
        }
    }

    fn live_design(cx: &mut Cx) {
        crate::screen::design::live_design(cx);
    }
}
