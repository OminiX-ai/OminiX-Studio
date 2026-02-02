pub mod theme;
pub mod app_trait;
pub mod moly_theme;
pub mod app_data;
pub mod page_router;

pub use app_trait::{MolyApp, AppInfo, AppRegistry, TimerControl};
pub use moly_theme::MolyTheme;
pub use app_data::{MolyAppData, AppAction};
pub use page_router::PageRouter;

use makepad_widgets::Cx;

/// Register all shared widgets with Makepad.
///
/// This function must be called during app initialization, typically in `LiveRegister::live_register`.
///
/// **Important**: Theme is registered first as other widgets depend on its font and color definitions.
pub fn live_design(cx: &mut Cx) {
    // Theme provides fonts and base styles - must be first
    theme::live_design(cx);
}
