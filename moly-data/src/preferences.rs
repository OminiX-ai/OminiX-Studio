use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::providers::{get_supported_providers, ProviderId, ProviderPreferences};

const PREFERENCES_FILENAME: &str = "preferences.json";

/// User preferences that persist across sessions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Preferences {
    /// Whether dark mode is enabled
    #[serde(default)]
    pub dark_mode: bool,

    /// Whether the sidebar is expanded
    #[serde(default = "default_sidebar_expanded")]
    pub sidebar_expanded: bool,

    /// The last selected navigation view (as string for serialization)
    #[serde(default)]
    pub current_view: String,

    /// AI provider configurations
    #[serde(default)]
    pub providers_preferences: Vec<ProviderPreferences>,

    /// Currently selected chat model
    #[serde(default)]
    pub current_chat_model: Option<String>,
}

fn default_sidebar_expanded() -> bool {
    true
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            dark_mode: false,
            sidebar_expanded: true,
            current_view: "Chat".to_string(),
            providers_preferences: get_supported_providers(),
            current_chat_model: None,
        }
    }
}

impl Preferences {
    /// Load preferences from disk, or return defaults if not found
    pub fn load() -> Self {
        let path = Self::preferences_path();
        log::debug!("Loading preferences from {:?}", path);

        if let Ok(contents) = std::fs::read_to_string(&path) {
            match serde_json::from_str::<Preferences>(&contents) {
                Ok(mut prefs) => {
                    log::debug!("Parsed preferences successfully");
                    // Ensure all supported providers exist
                    prefs.merge_with_supported_providers();
                    return prefs;
                }
                Err(e) => {
                    log::error!("Failed to parse preferences: {:?}", e);
                }
            }
        } else {
            log::debug!("No preferences file found, using defaults");
        }

        Preferences::default()
    }

    /// Save preferences to disk
    pub fn save(&self) {
        let path = Self::preferences_path();

        // Ensure directory exists
        if let Some(parent) = path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                log::error!("Failed to create preferences directory: {:?}", e);
                return;
            }
        }

        match serde_json::to_string_pretty(self) {
            Ok(json) => {
                if let Err(e) = std::fs::write(&path, &json) {
                    log::error!("Failed to write preferences: {:?}", e);
                } else {
                    log::info!("Saved preferences to {:?} ({} bytes)", path, json.len());
                }
            }
            Err(e) => {
                log::error!("Failed to serialize preferences: {:?}", e);
            }
        }
    }

    /// Get the path to the preferences file
    fn preferences_path() -> PathBuf {
        // Use home directory for reliable persistence
        if let Some(home) = dirs::home_dir() {
            let path = home.join(".moly").join(PREFERENCES_FILENAME);
            log::debug!("Preferences path: {:?}", path);
            path
        } else {
            // Fallback to current directory
            PathBuf::from(".moly").join(PREFERENCES_FILENAME)
        }
    }

    /// Set dark mode and save
    pub fn set_dark_mode(&mut self, dark_mode: bool) {
        log::info!("set_dark_mode: {}", dark_mode);
        self.dark_mode = dark_mode;
        self.save();
    }

    /// Set sidebar expanded state and save
    pub fn set_sidebar_expanded(&mut self, expanded: bool) {
        log::info!("set_sidebar_expanded: {}", expanded);
        self.sidebar_expanded = expanded;
        self.save();
    }

    /// Set current view and save
    pub fn set_current_view(&mut self, view: &str) {
        log::info!("set_current_view: {}", view);
        self.current_view = view.to_string();
        self.save();
    }

    /// Get a provider by ID
    pub fn get_provider(&self, id: &ProviderId) -> Option<&ProviderPreferences> {
        self.providers_preferences.iter().find(|p| &p.id == id)
    }

    /// Get a mutable provider by ID
    pub fn get_provider_mut(&mut self, id: &ProviderId) -> Option<&mut ProviderPreferences> {
        self.providers_preferences.iter_mut().find(|p| &p.id == id)
    }

    /// Update a provider's API key and save
    pub fn set_provider_api_key(&mut self, id: &ProviderId, api_key: Option<String>) {
        log::info!("set_provider_api_key: provider={}, key_len={:?}",
            id, api_key.as_ref().map(|k| k.len()));
        if let Some(provider) = self.get_provider_mut(id) {
            provider.api_key = api_key;
            self.save();
        } else {
            log::warn!("set_provider_api_key: provider {} not found!", id);
        }
    }

    /// Update a provider's URL and save
    pub fn set_provider_url(&mut self, id: &ProviderId, url: String) {
        log::info!("set_provider_url: provider={}, url={}", id, url);
        if let Some(provider) = self.get_provider_mut(id) {
            provider.url = url;
            self.save();
        }
    }

    /// Update a provider's enabled state and save
    pub fn set_provider_enabled(&mut self, id: &ProviderId, enabled: bool) {
        if let Some(provider) = self.get_provider_mut(id) {
            provider.enabled = enabled;
            self.save();
        }
    }

    /// Set the current chat model and save
    pub fn set_current_chat_model(&mut self, model: Option<String>) {
        log::info!("set_current_chat_model: {:?}", model);
        self.current_chat_model = model;
        self.save();
    }

    /// Get the current chat model
    pub fn get_current_chat_model(&self) -> Option<&str> {
        self.current_chat_model.as_deref()
    }

    /// Get all enabled providers with API keys
    pub fn get_enabled_providers(&self) -> Vec<&ProviderPreferences> {
        self.providers_preferences
            .iter()
            .filter(|p| p.enabled && p.has_api_key())
            .collect()
    }

    /// Get the first enabled provider with an API key (for backwards compatibility)
    pub fn get_active_provider(&self) -> Option<&ProviderPreferences> {
        self.providers_preferences
            .iter()
            .find(|p| p.enabled && p.has_api_key())
    }

    /// Merge loaded preferences with supported providers (add any missing)
    pub fn merge_with_supported_providers(&mut self) {
        let supported = get_supported_providers();
        for sp in supported {
            if !self.providers_preferences.iter().any(|p| p.id == sp.id) {
                self.providers_preferences.push(sp);
            }
        }
    }
}
