//! # MolyAppData - Scope-Injected App Data
//!
//! This module provides MolyAppData, the minimal interface that apps receive
//! via Makepad's Scope mechanism. Apps should access data through MolyAppData
//! rather than accessing Store directly.
//!
//! ## Architecture
//!
//! The shell creates MolyAppData and injects it into the scope:
//! ```ignore
//! scope.with_data(&mut self.app_data, |cx, scope| {
//!     self.ui.handle_event(cx, event, scope);
//! });
//! ```
//!
//! Apps receive it via the scope:
//! ```ignore
//! fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
//!     let app_data = scope.data.get::<MolyAppData>().unwrap();
//!     let dark_mode = app_data.theme.dark_mode;
//! }
//! ```
//!
//! ## Design Goals
//!
//! 1. **Minimal Interface**: Apps only see what they need
//! 2. **Read-Mostly**: Prefer read access; mutations go through actions
//! 3. **Decoupling**: Apps don't depend on Store internals

use crate::moly_theme::MolyTheme;

/// Data injected into app scope
///
/// This is the primary interface between the shell and apps.
/// Apps should access this instead of Store directly.
#[derive(Clone, Debug)]
pub struct MolyAppData {
    /// Runtime theme with animation support
    pub theme: MolyTheme,

    /// Current provider ID (if any configured)
    pub current_provider_id: Option<String>,

    /// Current chat model name (if selected)
    pub current_model: Option<String>,

    /// Whether a chat response is currently streaming
    pub is_streaming: bool,

    /// Whether sidebar is expanded
    pub sidebar_expanded: bool,

    /// Current navigation view name
    pub current_view: String,
}

impl Default for MolyAppData {
    fn default() -> Self {
        Self {
            theme: MolyTheme::default(),
            current_provider_id: None,
            current_model: None,
            is_streaming: false,
            sidebar_expanded: true,
            current_view: "Chat".to_string(),
        }
    }
}

impl MolyAppData {
    /// Create new MolyAppData with specified dark mode
    pub fn new(dark_mode: bool) -> Self {
        Self {
            theme: MolyTheme::new(dark_mode),
            ..Default::default()
        }
    }

    /// Update from preferences (called by shell on load/change)
    pub fn sync_from_preferences(
        &mut self,
        dark_mode: bool,
        sidebar_expanded: bool,
        current_view: &str,
        current_model: Option<&str>,
    ) {
        self.theme.set_dark_mode(dark_mode);
        self.sidebar_expanded = sidebar_expanded;
        self.current_view = current_view.to_string();
        self.current_model = current_model.map(|s| s.to_string());
    }

    /// Set current provider info
    pub fn set_provider(&mut self, provider_id: Option<String>) {
        self.current_provider_id = provider_id;
    }

    /// Set streaming state
    pub fn set_streaming(&mut self, streaming: bool) {
        self.is_streaming = streaming;
    }

    /// Check if dark mode is enabled
    pub fn is_dark_mode(&self) -> bool {
        self.theme.dark_mode
    }

    /// Get the animation value for shaders (0.0 = light, 1.0 = dark)
    pub fn dark_mode_anim(&self) -> f64 {
        self.theme.dark_mode_anim
    }
}

/// Actions that apps can dispatch for state changes
///
/// Instead of mutating state directly, apps post actions that
/// the shell processes centrally.
#[derive(Clone, Debug)]
pub enum AppAction {
    /// Toggle dark mode
    ToggleDarkMode,
    /// Set dark mode explicitly
    SetDarkMode(bool),
    /// Toggle sidebar
    ToggleSidebar,
    /// Navigate to a view
    Navigate(String),
    /// Select a chat model
    SelectModel(String),
    /// Send a chat message
    SendMessage(String),
    /// Create a new chat
    NewChat,
    /// Delete a chat
    DeleteChat(u128),
}
