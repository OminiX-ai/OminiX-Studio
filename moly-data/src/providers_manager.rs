use std::collections::HashMap;
use moly_kit::aitk::clients::openai::OpenAiClient;
use moly_kit::aitk::clients::openai_realtime::OpenAiRealtimeClient;
use moly_kit::aitk::protocol::{Bot, BotClient, BotId, EntityAvatar};

use crate::ominix_image_client::{OminiXImageClient, ImageGenerationConfig};
use crate::providers::{ProviderPreferences, ProviderType};

/// Manages multiple AI provider clients and their models
pub struct ProvidersManager {
    /// Map of provider_id -> OpenAiClient (for text chat)
    clients: HashMap<String, OpenAiClient>,
    /// Map of provider_id -> OpenAiRealtimeClient (for voice chat)
    realtime_clients: HashMap<String, OpenAiRealtimeClient>,
    /// Map of provider_id -> OminiXImageClient (for image generation)
    image_clients: HashMap<String, OminiXImageClient>,
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
            realtime_clients: HashMap::new(),
            image_clients: HashMap::new(),
            provider_bots: HashMap::new(),
            all_bots: Vec::new(),
            active_provider_id: None,
        }
    }

    /// Configure clients for all enabled providers
    pub fn configure_providers(&mut self, providers: &[&ProviderPreferences]) {
        self.clients.clear();
        self.realtime_clients.clear();
        self.image_clients.clear();
        self.provider_bots.clear();
        self.all_bots.clear();

        for provider in providers {
            // OminiX Image doesn't require API key for local server
            let api_key = provider.api_key.as_ref().map(|k| k.trim()).unwrap_or("");

            match provider.provider_type {
                ProviderType::OpenAiRealtime => {
                    if api_key.is_empty() {
                        continue;
                    }
                    // Create realtime client for voice chat
                    let mut client = OpenAiRealtimeClient::new(provider.url.clone());
                    if client.set_key(api_key).is_ok() {
                        // Set system prompt if configured
                        if let Some(prompt) = &provider.system_prompt {
                            let _ = client.set_system_prompt(prompt);
                        }
                        client.set_tools_enabled(provider.tools_enabled);
                        log::info!("Configured realtime client for provider: {} ({})", provider.id, provider.url);
                        self.realtime_clients.insert(provider.id.clone(), client);
                    }
                }
                ProviderType::OminiXImage => {
                    // Create OminiX image client (no API key required for local)
                    let mut client = OminiXImageClient::new(provider.url.clone())
                        .with_config(ImageGenerationConfig::new().with_size("512x512"));

                    // Set API key if provided (for remote servers)
                    if !api_key.is_empty() {
                        let _ = client.set_key(api_key);
                    }

                    log::info!("Configured OminiX image client for provider: {} ({})", provider.id, provider.url);
                    self.image_clients.insert(provider.id.clone(), client);
                }
                _ => {
                    if api_key.is_empty() {
                        continue;
                    }
                    // Create standard OpenAI-compatible client for text chat
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

    /// Get realtime client for a specific provider
    pub fn get_realtime_client(&self, provider_id: &str) -> Option<&OpenAiRealtimeClient> {
        self.realtime_clients.get(provider_id)
    }

    /// Clone realtime client for a specific provider
    pub fn clone_realtime_client(&self, provider_id: &str) -> Option<OpenAiRealtimeClient> {
        self.realtime_clients.get(provider_id).cloned()
    }

    /// Get image client for a specific provider
    pub fn get_image_client(&self, provider_id: &str) -> Option<&OminiXImageClient> {
        self.image_clients.get(provider_id)
    }

    /// Get mutable image client for a specific provider
    pub fn get_image_client_mut(&mut self, provider_id: &str) -> Option<&mut OminiXImageClient> {
        self.image_clients.get_mut(provider_id)
    }

    /// Clone image client for a specific provider
    pub fn clone_image_client(&self, provider_id: &str) -> Option<OminiXImageClient> {
        self.image_clients.get(provider_id).cloned()
    }

    /// Get a boxed BotClient for any provider type
    pub fn get_bot_client(&self, provider_id: &str) -> Option<Box<dyn BotClient>> {
        if let Some(client) = self.clients.get(provider_id) {
            Some(Box::new(client.clone()))
        } else if let Some(client) = self.realtime_clients.get(provider_id) {
            Some(Box::new(client.clone()))
        } else if let Some(client) = self.image_clients.get(provider_id) {
            Some(Box::new(client.clone()))
        } else {
            None
        }
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
        // BotId is typically in format "provider_url/model_name"
        let bot_id_str = bot_id.as_str();
        for (provider_id, _) in &self.clients {
            if bot_id_str.contains(provider_id) {
                return Some(provider_id);
            }
        }
        for (provider_id, _) in &self.realtime_clients {
            if bot_id_str.contains(provider_id) {
                return Some(provider_id);
            }
        }
        for (provider_id, _) in &self.image_clients {
            if bot_id_str.contains(provider_id) {
                return Some(provider_id);
            }
        }
        None
    }

    /// Inject a local OminiX model provider (no API key required, uses localhost:8080)
    ///
    /// This bypasses the normal configure_providers flow and directly registers
    /// an OpenAI-compatible client for the locally-running ominix-api server.
    pub fn inject_local_model(&mut self, model_id: &str) {
        let mut client = OpenAiClient::new("http://localhost:8080/v1".to_string());
        // Local server accepts any non-empty key string
        let _ = client.set_key("sk-local");
        self.clients.insert("ominix-local".to_string(), client);

        let bot = Bot {
            id: BotId::new(model_id),
            name: model_id.to_string(),
            avatar: EntityAvatar::Text("AI".to_string()),
            capabilities: Default::default(),
        };
        self.provider_bots.insert("ominix-local".to_string(), vec![bot]);
        self.rebuild_all_bots();
        log::info!("Injected local model: {} at localhost:8080", model_id);
    }

    /// Remove the injected local model provider
    pub fn remove_local_model(&mut self) {
        self.clients.remove("ominix-local");
        self.provider_bots.remove("ominix-local");
        self.rebuild_all_bots();
        log::info!("Removed local model provider");
    }

    /// Check if a local model is currently injected
    pub fn has_local_model(&self) -> bool {
        self.clients.contains_key("ominix-local")
    }

    /// Check if any providers are configured
    pub fn has_providers(&self) -> bool {
        !self.clients.is_empty() || !self.realtime_clients.is_empty() || !self.image_clients.is_empty()
    }

    /// Get list of configured provider IDs
    pub fn configured_provider_ids(&self) -> Vec<&str> {
        self.clients.keys()
            .chain(self.realtime_clients.keys())
            .chain(self.image_clients.keys())
            .map(|s| s.as_str())
            .collect()
    }

    /// Check if a provider is a realtime provider
    pub fn is_realtime_provider(&self, provider_id: &str) -> bool {
        self.realtime_clients.contains_key(provider_id)
    }

    /// Check if a provider is an image generation provider
    pub fn is_image_provider(&self, provider_id: &str) -> bool {
        self.image_clients.contains_key(provider_id)
    }

    /// Configure image generation settings for a provider
    pub fn configure_image_settings(
        &mut self,
        provider_id: &str,
        size: Option<&str>,
        strength: Option<f32>,
    ) {
        if let Some(client) = self.image_clients.get_mut(provider_id) {
            if let Some(size) = size {
                client.set_size(size);
            }
            if let Some(strength) = strength {
                client.set_strength(strength);
            }
        }
    }

    /// Set reference image for img2img on a provider
    pub fn set_image_reference(&mut self, provider_id: &str, image_base64: Option<String>) {
        if let Some(client) = self.image_clients.get_mut(provider_id) {
            client.set_reference_image(image_base64);
        }
    }
}
