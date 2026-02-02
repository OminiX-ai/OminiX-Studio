//! # MolyApp Trait - Plugin App Interface
//!
//! This module defines the standard interface for apps that integrate with the Moly shell.
//! Based on mofa-studio's minimal-coupling architecture with only 4 integration points.
//!
//! ## Architecture
//!
//! Apps are separate crates that implement the MolyApp trait. The shell imports and
//! registers them via `live_design(cx)` calls. Widget types are then available for
//! use in the shell's `live_design!` macro via full module paths.
//!
//! ## 4-Point Coupling Pattern
//!
//! Each app connects to the shell through exactly 4 touch points:
//! 1. Import: `use moly_chat::MolyChatApp;`
//! 2. Live Register: `MolyChatApp::live_design(cx);`
//! 3. Metadata: `MolyChatApp::info()` for registry
//! 4. UI Definition: `<MolyChatScreen> {}` in live_design!
//!
//! ## Usage in Shell
//!
//! ```rust,ignore
//! use moly_widgets::{MolyApp, AppRegistry};
//! use moly_chat::MolyChatApp;
//! use moly_settings::MolySettingsApp;
//!
//! // In LiveRegister (order matters - apps before widgets that use them)
//! fn live_register(cx: &mut Cx) {
//!     makepad_widgets::live_design(cx);
//!     moly_widgets::live_design(cx);
//!     <MolyChatApp as MolyApp>::live_design(cx);
//!     <MolySettingsApp as MolyApp>::live_design(cx);
//!     // Then shell widgets that use app screens
//! }
//! ```
//!
//! ## Creating a New App
//!
//! ```rust,ignore
//! use moly_widgets::{MolyApp, AppInfo};
//! use makepad_widgets::live_id;
//!
//! pub struct MyCoolApp;
//!
//! impl MolyApp for MyCoolApp {
//!     fn info() -> AppInfo {
//!         AppInfo {
//!             name: "My Cool App",
//!             id: "my-cool-app",
//!             description: "A cool Moly app",
//!             icon: live_id!(IconStar),
//!             page_id: live_id!(my_cool_screen),
//!         }
//!     }
//!
//!     fn live_design(cx: &mut Cx) {
//!         crate::screen::live_design(cx);
//!     }
//! }
//! ```

use makepad_widgets::{Cx, LiveId};

/// Metadata about a registered app
#[derive(Clone, Debug)]
pub struct AppInfo {
    /// Display name shown in UI
    pub name: &'static str,
    /// Unique identifier for the app
    pub id: &'static str,
    /// Description of the app
    pub description: &'static str,
    /// Icon LiveId for sidebar/navigation
    pub icon: LiveId,
    /// Page/screen LiveId for navigation
    pub page_id: LiveId,
}

/// Trait for apps that integrate with Moly shell
///
/// # Example
/// ```ignore
/// impl MolyApp for MolyChatApp {
///     fn info() -> AppInfo {
///         AppInfo {
///             name: "Chat",
///             id: "moly-chat",
///             description: "AI chat interface",
///             icon: live_id!(IconChat),
///             page_id: live_id!(moly_chat_screen),
///         }
///     }
///
///     fn live_design(cx: &mut Cx) {
///         crate::screen::live_design(cx);
///     }
/// }
/// ```
pub trait MolyApp {
    /// Returns metadata about this app
    fn info() -> AppInfo where Self: Sized;

    /// Register this app's widgets with Makepad
    fn live_design(cx: &mut Cx);
}

/// Trait for widgets that have background timers/animations
///
/// Implement this trait to properly pause and resume resources when
/// the app is hidden/shown during navigation.
///
/// # Example
/// ```ignore
/// impl TimerControl for MolyChatScreen {
///     fn stop_timers(&self, cx: &mut Cx) {
///         // Stop polling, animations, etc.
///     }
///
///     fn start_timers(&self, cx: &mut Cx) {
///         // Resume polling, animations, etc.
///     }
/// }
/// ```
pub trait TimerControl {
    /// Called when the widget is being hidden (navigated away from)
    fn stop_timers(&self, cx: &mut Cx);

    /// Called when the widget is being shown (navigated to)
    fn start_timers(&self, cx: &mut Cx);
}

/// Registry of all installed apps
///
/// Provides metadata for runtime queries (e.g., sidebar generation).
pub struct AppRegistry {
    apps: Vec<AppInfo>,
}

impl AppRegistry {
    /// Create a new empty registry
    pub const fn new() -> Self {
        Self { apps: Vec::new() }
    }

    /// Register an app in the registry
    pub fn register(&mut self, info: AppInfo) {
        self.apps.push(info);
    }

    /// Get all registered apps
    pub fn apps(&self) -> &[AppInfo] {
        &self.apps
    }

    /// Find an app by ID
    pub fn find_by_id(&self, id: &str) -> Option<&AppInfo> {
        self.apps.iter().find(|app| app.id == id)
    }

    /// Number of registered apps
    pub fn len(&self) -> usize {
        self.apps.len()
    }

    /// Check if registry is empty
    pub fn is_empty(&self) -> bool {
        self.apps.is_empty()
    }
}

impl Default for AppRegistry {
    fn default() -> Self {
        Self::new()
    }
}
