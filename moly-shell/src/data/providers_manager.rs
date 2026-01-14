use std::collections::HashMap;
use moly_kit::aitk::clients::openai::OpenAiClient;
use moly_kit::aitk::protocol::{Bot, BotId};

use super::providers::ProviderPreferences;

/// Manages multiple AI provider clients and their models
pub struct ProvidersManager {
    /// Map of provider_id -> OpenAiClient
    clients: HashMap<String, OpenAiClient>,
    /// Map of provider_id -> list of bots from that provider
    provider_bots: HashMap<String, Vec<Bot>>,
    /// Combined list of all bots from all providers
    all_bots: Vec<Bot>,
    /// Currently active provider ID
    active_provider_id: Option<String>,
}

impl Default for ProvidersManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ProvidersManager {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
            provider_bots: HashMap::new(),
            all_bots: Vec::new(),
            active_provider_id: None,
        }
    }

    /// Configure clients for all enabled providers
    pub fn configure_providers(&mut self, providers: &[&ProviderPreferences]) {
        self.clients.clear();
        self.provider_bots.clear();
        self.all_bots.clear();

        for provider in providers {
            if let Some(api_key) = &provider.api_key {
                let api_key = api_key.trim();
                if api_key.is_empty() {
                    continue;
                }

                let mut client = OpenAiClient::new(provider.url.clone());
                if client.set_key(api_key).is_ok() {
                    log::info!("Configured client for provider: {} ({})", provider.id, provider.url);
                    self.clients.insert(provider.id.clone(), client);

                    // Set first provider as active if none set
                    if self.active_provider_id.is_none() {
                        self.active_provider_id = Some(provider.id.clone());
                    }
                }
            }
        }
    }

    /// Get the currently active client
    pub fn get_active_client(&self) -> Option<&OpenAiClient> {
        self.active_provider_id.as_ref().and_then(|id| self.clients.get(id))
    }

    /// Get a mutable reference to the active client
    pub fn get_active_client_mut(&mut self) -> Option<&mut OpenAiClient> {
        if let Some(id) = &self.active_provider_id {
            self.clients.get_mut(id)
        } else {
            None
        }
    }

    /// Get client for a specific provider
    pub fn get_client(&self, provider_id: &str) -> Option<&OpenAiClient> {
        self.clients.get(provider_id)
    }

    /// Clone client for a specific provider (needed for ChatController)
    pub fn clone_client(&self, provider_id: &str) -> Option<OpenAiClient> {
        self.clients.get(provider_id).cloned()
    }

    /// Set the active provider by ID
    pub fn set_active_provider(&mut self, provider_id: &str) -> bool {
        if self.clients.contains_key(provider_id) {
            self.active_provider_id = Some(provider_id.to_string());
            log::info!("Active provider set to: {}", provider_id);
            true
        } else {
            log::warn!("Cannot set active provider: {} not configured", provider_id);
            false
        }
    }

    /// Get the active provider ID
    pub fn active_provider_id(&self) -> Option<&str> {
        self.active_provider_id.as_deref()
    }

    /// Set bots for a specific provider
    pub fn set_provider_bots(&mut self, provider_id: &str, bots: Vec<Bot>) {
        log::info!("Setting {} bots for provider {}", bots.len(), provider_id);
        self.provider_bots.insert(provider_id.to_string(), bots);
        self.rebuild_all_bots();
    }

    /// Rebuild the combined bots list from all providers
    fn rebuild_all_bots(&mut self) {
        self.all_bots.clear();
        for (provider_id, bots) in &self.provider_bots {
            for bot in bots {
                // Clone bot and ensure it has provider info in the ID
                let bot = bot.clone();
                // The BotId should already contain the provider URL, but we can log it
                log::debug!("Adding bot: {} from provider {}", bot.name, provider_id);
                self.all_bots.push(bot);
            }
        }
        log::info!("Total bots from all providers: {}", self.all_bots.len());
    }

    /// Get all bots from all providers
    pub fn get_all_bots(&self) -> &[Bot] {
        &self.all_bots
    }

    /// Clear all bots from all providers
    pub fn clear_all_bots(&mut self) {
        self.provider_bots.clear();
        self.all_bots.clear();
        log::info!("Cleared all bots from providers manager");
    }

    /// Get the provider ID for a given bot ID (by matching the provider string)
    pub fn get_provider_for_bot(&self, bot_id: &BotId) -> Option<&str> {
        // First check exact match in our stored bots
        for (provider_id, bots) in &self.provider_bots {
            if bots.iter().any(|b| &b.id == bot_id) {
                return Some(provider_id);
            }
        }
        // Check by provider string in the bot_id
        let bot_provider = bot_id.provider();
        for (provider_id, _) in &self.clients {
            // Match by checking if the bot's provider contains a known provider URL pattern
            if bot_provider.contains(provider_id) {
                return Some(provider_id);
            }
        }
        None
    }

    /// Check if any providers are configured
    pub fn has_providers(&self) -> bool {
        !self.clients.is_empty()
    }

    /// Get list of configured provider IDs
    pub fn configured_provider_ids(&self) -> Vec<&str> {
        self.clients.keys().map(|s| s.as_str()).collect()
    }
}
