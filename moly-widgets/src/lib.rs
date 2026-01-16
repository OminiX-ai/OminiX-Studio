pub mod theme;
pub mod app_trait;

pub use app_trait::{MolyApp, AppInfo, AppRegistry};

use makepad_widgets::*;

live_design! {
    use crate::theme::*;
}
