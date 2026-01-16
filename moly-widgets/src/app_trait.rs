//! # MolyApp Trait - Plugin App Interface
//!
//! This module defines the standard interface for apps that integrate with the Moly shell.
//!
//! ## Architecture
//!
//! Apps are separate crates that implement the MolyApp trait. The shell imports and
//! registers them via `live_design(cx)` calls. Widget types are then available for
//! use in the shell's `live_design!` macro via full module paths.
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
//!
//! pub struct MyCoolApp;
//!
//! impl MolyApp for MyCoolApp {
//!     fn info() -> AppInfo {
//!         AppInfo {
//!             name: "My Cool App",
//!             id: "my-cool-app",
//!             description: "A cool Moly app",
//!         }
//!     }
//!
//!     fn live_design(cx: &mut Cx) {
//!         crate::screen::live_design(cx);
//!     }
//! }
//! ```

use makepad_widgets::Cx;

/// Metadata about a registered app
#[derive(Clone, Debug)]
pub struct AppInfo {
    /// Display name shown in UI
    pub name: &'static str,
    /// Unique identifier for the app
    pub id: &'static str,
    /// Description of the app
    pub description: &'static str,
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
