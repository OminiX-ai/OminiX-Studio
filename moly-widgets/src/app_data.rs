//! # MolyAppData - Scope-Injected App Data
//!
//! This module provides MolyAppData, the minimal interface that apps receive
//! via Makepad's Scope mechanism.

/// Data injected into app scope
#[derive(Clone, Debug)]
pub struct MolyAppData {
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
            current_provider_id: None,
            current_model: None,
            is_streaming: false,
            sidebar_expanded: true,
            current_view: "Chat".to_string(),
        }
    }
}

impl MolyAppData {
    /// Create new MolyAppData
    pub fn new() -> Self {
        Self::default()
    }

    /// Update from preferences (called by shell on load/change)
    pub fn sync_from_preferences(
        &mut self,
        sidebar_expanded: bool,
        current_view: &str,
        current_model: Option<&str>,
    ) {
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
}

/// Actions that apps can dispatch for state changes
#[derive(Clone, Debug)]
pub enum AppAction {
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
