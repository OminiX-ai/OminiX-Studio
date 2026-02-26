use makepad_widgets::*;
use moly_kit::prelude::*;
use std::sync::{Arc, Mutex};

use crate::chats::Chats;
use crate::moly_client::MolyClient;
use crate::preferences::Preferences;
use crate::providers_manager::ProvidersManager;
use crate::model_registry::RegistryCategory;

/// Actions that can be dispatched to modify the Store
#[derive(Clone, Debug, DefaultNone)]
pub enum StoreAction {
    /// Toggle dark mode
    ToggleDarkMode,
    /// Set dark mode explicitly
    SetDarkMode(bool),
    /// Toggle sidebar expanded/collapsed
    ToggleSidebar,
    /// Set sidebar expanded state explicitly
    SetSidebarExpanded(bool),
    /// Navigate to a specific view
    Navigate(String),
    /// Set the active local model for chat routing (api_model_id from registry, or None to clear)
    SetLocalModel(Option<String>),
    /// Open a new chat session pre-loaded with a specific model
    OpenChatWithModel { model_id: String, category: RegistryCategory },
    /// No action
    None,
}

/// Central state container for the application
///
/// The Store holds all shared application state and is passed down
/// to widgets via Makepad's Scope mechanism.
///
/// # Usage
///
/// In App's handle_event:
/// ```rust,ignore
/// let scope = &mut Scope::with_data(&mut self.store);
/// self.ui.handle_event(cx, event, scope);
/// ```
///
/// In child widgets:
/// ```rust,ignore
/// let store = scope.data.get::<Store>().unwrap();
/// // Read from store
///
/// let store = scope.data.get_mut::<Store>().unwrap();
/// // Modify store
/// ```
pub struct Store {
    /// User preferences (persisted to disk)
    pub preferences: Preferences,

    /// Chat sessions management
    pub chats: Chats,

    /// The ChatController for the current chat (from aitk)
    pub chat_controller: Option<Arc<Mutex<ChatController>>>,

    /// Multi-provider client manager
    pub providers_manager: ProvidersManager,

    /// Moly Server client for model discovery and downloads
    pub moly_client: MolyClient,

    /// Whether the Store has been fully initialized
    pub initialized: bool,

    /// The api_model_id of the currently active local model (routes chat to localhost:8080).
    /// Set when user clicks "Open in Chat" from Model Hub; cleared on model unload.
    pub active_local_model: Option<String>,

    /// Pending model to open in a new chat session.
    /// Set by StoreAction::OpenChatWithModel; cleared once consumed by ChatApp.
    pub pending_chat_model: Option<(String, RegistryCategory)>,
}

impl Default for Store {
    fn default() -> Self {
        // WARNING: This creates a Store with default preferences (no API keys!)
        // Use Store::load() instead to load from disk.
        Self {
            preferences: Preferences::default(),
            chats: Chats::new(),
            chat_controller: None,
            providers_manager: ProvidersManager::new(),
            moly_client: MolyClient::new(),
            initialized: false,
            active_local_model: None,
            pending_chat_model: None,
        }
    }
}

impl Store {
    /// Create a new Store by loading preferences from disk
    pub fn load() -> Self {
        let preferences = Preferences::load();

        // Create a ChatController with basic async spawner
        let chat_controller = ChatController::new_arc();
        {
            let mut controller = chat_controller.lock().unwrap();
            controller.set_basic_spawner();
        }

        // Create ProvidersManager and configure with enabled providers
        let mut providers_manager = ProvidersManager::new();
        let enabled_providers: Vec<_> = preferences.get_enabled_providers();
        providers_manager.configure_providers(&enabled_providers);

        // Load chats from disk
        let chats = Chats::load();

        // Create MolyClient for model discovery
        let moly_client = MolyClient::new();

        Self {
            preferences,
            chats,
            chat_controller: Some(chat_controller),
            providers_manager,
            moly_client,
            initialized: true,
            active_local_model: None,
            pending_chat_model: None,
        }
    }

    /// Reconfigure providers manager when provider settings change
    pub fn reconfigure_providers(&mut self) {
        let enabled_providers: Vec<_> = self.preferences.get_enabled_providers();
        self.providers_manager.configure_providers(&enabled_providers);
        // Re-inject local model if active (configure_providers clears all clients)
        if let Some(ref model_id) = self.active_local_model.clone() {
            self.providers_manager.inject_local_model(model_id);
        }
    }

    /// Set the active local model for chat routing (injects provider at localhost:8080)
    pub fn set_active_local_model(&mut self, model_id: Option<String>) {
        match &model_id {
            Some(id) => self.providers_manager.inject_local_model(id),
            None     => self.providers_manager.remove_local_model(),
        }
        self.active_local_model = model_id;
    }

    /// Get the currently active local model ID (api_model_id from registry)
    pub fn get_active_local_model(&self) -> Option<&str> {
        self.active_local_model.as_deref()
    }

    /// Set pending chat model (signals ChatApp to open a new chat with this model)
    pub fn set_pending_chat_model(&mut self, model_id: String, category: RegistryCategory) {
        self.pending_chat_model = Some((model_id, category));
    }

    /// Take pending chat model (clears it after reading)
    pub fn take_pending_chat_model(&mut self) -> Option<(String, RegistryCategory)> {
        self.pending_chat_model.take()
    }

    /// Get a reference to the ChatController
    pub fn get_chat_controller(&self) -> Option<Arc<Mutex<ChatController>>> {
        self.chat_controller.clone()
    }

    /// Check if dark mode is enabled
    pub fn is_dark_mode(&self) -> bool {
        self.preferences.dark_mode
    }

    /// Set dark mode state
    pub fn set_dark_mode(&mut self, dark_mode: bool) {
        self.preferences.set_dark_mode(dark_mode);
    }

    /// Toggle dark mode
    pub fn toggle_dark_mode(&mut self) {
        self.set_dark_mode(!self.is_dark_mode());
    }

    /// Check if sidebar is expanded
    pub fn is_sidebar_expanded(&self) -> bool {
        self.preferences.sidebar_expanded
    }

    /// Set sidebar expanded state
    pub fn set_sidebar_expanded(&mut self, expanded: bool) {
        self.preferences.set_sidebar_expanded(expanded);
    }

    /// Toggle sidebar expanded/collapsed
    pub fn toggle_sidebar(&mut self) {
        self.set_sidebar_expanded(!self.is_sidebar_expanded());
    }

    /// Get current view name
    pub fn current_view(&self) -> &str {
        &self.preferences.current_view
    }

    /// Set current view
    pub fn set_current_view(&mut self, view: &str) {
        self.preferences.set_current_view(view);
    }

    /// Handle a StoreAction and update state accordingly
    pub fn handle_action(&mut self, action: &StoreAction) {
        match action {
            StoreAction::ToggleDarkMode => {
                self.toggle_dark_mode();
            }
            StoreAction::SetDarkMode(dark_mode) => {
                self.set_dark_mode(*dark_mode);
            }
            StoreAction::ToggleSidebar => {
                self.toggle_sidebar();
            }
            StoreAction::SetSidebarExpanded(expanded) => {
                self.set_sidebar_expanded(*expanded);
            }
            StoreAction::Navigate(view) => {
                self.set_current_view(view);
            }
            StoreAction::SetLocalModel(model_id) => {
                self.set_active_local_model(model_id.clone());
            }
            StoreAction::OpenChatWithModel { model_id, category } => {
                self.set_active_local_model(Some(model_id.clone()));
                self.set_pending_chat_model(model_id.clone(), *category);
            }
            StoreAction::None => {}
        }
    }
}
