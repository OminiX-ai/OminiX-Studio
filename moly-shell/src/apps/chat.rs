use makepad_widgets::*;
use moly_kit::prelude::*;
use moly_kit::aitk::controllers::chat::{ChatStateMutation, ChatTask};
use moly_kit::aitk::protocol::{Bot, BotId};
use std::sync::{Arc, Mutex};

use crate::data::Store;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use moly_widgets::theme::*;
    use moly_kit::widgets::chat::Chat;

    pub ChatApp = {{ChatApp}} {
        width: Fill, height: Fill
        flow: Down
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix(#f5f7fa, #0f172a, self.dark_mode);
            }
        }

        // Header with provider status
        header = <View> {
            width: Fill, height: Fit
            flow: Down
            padding: 16
            spacing: 4

            title_label = <Label> {
                text: "Chat"
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#1f2937, #f1f5f9, self.dark_mode);
                    }
                    text_style: <THEME_FONT_BOLD>{ font_size: 20.0 }
                }
            }

            status_label = <Label> {
                text: "No provider configured - Go to Settings to add an API key"
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        return mix(#f59e0b, #fbbf24, self.dark_mode);
                    }
                    text_style: <THEME_FONT_REGULAR>{ font_size: 11.0 }
                }
            }
        }

        // Chat widget from moly-kit
        chat = <Chat> {
            width: Fill, height: Fill
        }
    }
}

#[derive(Live, Widget)]
pub struct ChatApp {
    #[deref]
    pub view: View,

    // We create our own controller and set it on the Chat widget
    #[rust(ChatController::new_arc())]
    chat_controller: Arc<Mutex<ChatController>>,

    #[rust]
    controller_set_on_widget: bool,

    #[rust]
    providers_configured: bool,

    #[rust]
    current_provider_id: Option<String>,

    /// Track which providers we've already fetched models from
    #[rust]
    fetched_provider_ids: Vec<String>,

    /// List of providers to fetch models from (in order)
    #[rust]
    providers_to_fetch: Vec<String>,

    /// Index of the provider currently being fetched
    #[rust]
    fetch_index: usize,

    /// Whether we're currently waiting for a model fetch to complete
    #[rust]
    fetch_in_progress: bool,

    /// Number of bots we last saw from the current fetch
    #[rust]
    last_bots_count: usize,

    /// Track the last saved bot_id to detect changes
    #[rust]
    last_saved_bot_id: Option<String>,

    /// Whether we've restored the saved model selection
    #[rust]
    restored_saved_model: bool,

    /// Whether we need to force re-set the controller (after models load or visibility change)
    #[rust]
    needs_controller_reset: bool,
}

impl LiveHook for ChatApp {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        // Initialize the controller with basic spawner
        let mut controller = self.chat_controller.lock().unwrap();
        controller.set_basic_spawner();
    }
}

impl ChatApp {
    /// Set our controller on the Chat widget if not already done
    fn maybe_set_controller_on_widget(&mut self, cx: &mut Cx) {
        if self.controller_set_on_widget {
            return;
        }

        let mut chat_ref = self.view.chat(ids!(chat));
        chat_ref.write().set_chat_controller(cx, Some(self.chat_controller.clone()));
        self.controller_set_on_widget = true;
    }

    /// Force re-set the controller on the Chat widget
    /// This handles visibility changes and ensures bots are properly propagated
    fn force_reset_controller_on_widget(&mut self, cx: &mut Cx) {
        let mut chat_ref = self.view.chat(ids!(chat));
        // Set to None first to bypass the same-pointer check
        chat_ref.write().set_chat_controller(cx, None);
        chat_ref.write().set_chat_controller(cx, Some(self.chat_controller.clone()));
    }

    /// Called by the parent App when this view becomes visible
    /// This triggers a controller reset to ensure the model list is populated
    pub fn on_become_visible(&mut self) {
        self.needs_controller_reset = true;
    }
}

impl Widget for ChatApp {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // Set controller on Chat widget early (required for Messages widget)
        self.maybe_set_controller_on_widget(cx);

        // Handle pending controller reset (e.g., after models load or view becomes visible)
        // This ensures the model list is properly populated after visibility changes
        if self.needs_controller_reset {
            let bots_count = self.chat_controller.lock().unwrap().state().bots.len();
            if bots_count > 0 {
                self.force_reset_controller_on_widget(cx);
                // Re-dispatch bots mutation so the new plugin sees them
                let all_bots: Vec<_> = self.chat_controller.lock().unwrap().state().bots.clone();
                self.chat_controller.lock().unwrap().dispatch_mutation(VecMutation::Set(all_bots));
                self.view.redraw(cx);
                self.view.chat(ids!(chat)).redraw(cx);
            }
            self.needs_controller_reset = false;
        }

        // Check and configure providers from Store
        self.maybe_configure_providers(cx, scope);

        // Check for loaded bots from the ChatController
        self.check_for_loaded_bots(cx, scope);

        // Track model selection changes and save to preferences
        self.track_model_selection(scope);

        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Access Store from Scope to get dark mode state
        if let Some(store) = scope.data.get::<Store>() {
            let dark_mode_value = if store.is_dark_mode() { 1.0 } else { 0.0 };

            // Apply dark mode to background
            self.view.apply_over(cx, live! {
                draw_bg: { dark_mode: (dark_mode_value) }
            });

            // Apply dark mode to header labels
            self.view.label(ids!(title_label)).apply_over(cx, live! {
                draw_text: { dark_mode: (dark_mode_value) }
            });
            self.view.label(ids!(status_label)).apply_over(cx, live! {
                draw_text: { dark_mode: (dark_mode_value) }
            });

            // Update status label based on provider configuration
            if self.providers_configured {
                let num_providers = self.fetched_provider_ids.len();
                if num_providers == 1 {
                    let provider_name = self.current_provider_id.as_deref().unwrap_or("Unknown");
                    self.view.label(ids!(status_label)).set_text(cx,
                        &format!("Connected to {}", provider_name));
                } else if num_providers > 1 {
                    self.view.label(ids!(status_label)).set_text(cx,
                        &format!("Connected to {} providers", num_providers));
                }
            }
        }

        self.view.draw_walk(cx, scope, walk)
    }
}

impl ChatApp {
    /// Configure all enabled providers and start fetching models sequentially
    fn maybe_configure_providers(&mut self, cx: &mut Cx, scope: &mut Scope) {
        // If we're already fetching, don't restart
        if self.fetch_in_progress {
            return;
        }

        let Some(store) = scope.data.get_mut::<Store>() else { return };

        // Get all enabled providers with API keys - clone to avoid borrow issues
        let enabled_providers: Vec<_> = store.preferences.get_enabled_providers()
            .iter()
            .map(|p| (*p).clone())
            .collect();

        // Check if we need to reconfigure (new providers added or removed)
        let current_provider_ids: Vec<_> = enabled_providers.iter().map(|p| p.id.clone()).collect();
        let mut needs_reconfigure = false;

        // Check if the set of providers has changed
        if self.providers_configured {
            let old_set: std::collections::HashSet<_> = self.fetched_provider_ids.iter().collect();
            let new_set: std::collections::HashSet<_> = current_provider_ids.iter().collect();
            needs_reconfigure = old_set != new_set;
        }

        // Skip if already configured and no changes
        if self.providers_configured && !needs_reconfigure {
            return;
        }

        // Handle case when all providers are disabled
        if enabled_providers.is_empty() {
            if self.providers_configured {
                ::log::info!("All providers disabled, clearing models");
                // Clear all bots
                store.providers_manager.clear_all_bots();
                {
                    let mut ctrl = self.chat_controller.lock().unwrap();
                    ctrl.dispatch_mutation(VecMutation::<Bot>::Set(vec![]));
                    ctrl.dispatch_mutation(ChatStateMutation::SetBotId(None));
                }
                self.fetched_provider_ids.clear();
                self.providers_configured = false;
                self.restored_saved_model = false;
                self.last_saved_bot_id = None;
                self.view.redraw(cx);
            }
            return;
        }

        ::log::info!("Configuring {} providers for multi-provider support", enabled_providers.len());

        // Clear previous state if reconfiguring
        if needs_reconfigure {
            ::log::info!("Provider configuration changed, clearing existing models");
            store.providers_manager.clear_all_bots();
            self.restored_saved_model = false;  // Allow model selection after reload
        }
        self.fetched_provider_ids.clear();
        self.providers_to_fetch.clear();
        self.fetch_index = 0;

        // Configure all provider clients in ProvidersManager
        store.reconfigure_providers();

        // Build list of providers to fetch
        for provider in &enabled_providers {
            let api_key = provider.api_key.clone().unwrap_or_default();
            let api_key = api_key.trim().to_string();
            if api_key.is_empty() {
                ::log::warn!("API key is empty for provider {}", provider.id);
                continue;
            }

            // Debug: show key length and first/last chars
            let key_preview = if api_key.len() > 8 {
                format!("{}...{} (len={})", &api_key[..4], &api_key[api_key.len()-4..], api_key.len())
            } else {
                format!("(len={})", api_key.len())
            };
            ::log::info!("Will fetch models from provider {} with API key: {}", provider.id, key_preview);

            self.providers_to_fetch.push(provider.id.clone());
        }

        self.providers_configured = true;

        // Start fetching from the first provider
        if !self.providers_to_fetch.is_empty() {
            self.start_fetch_for_provider(cx, scope, 0);
        }
    }

    /// Start fetching models from a specific provider by index
    fn start_fetch_for_provider(&mut self, cx: &mut Cx, scope: &mut Scope, index: usize) {
        if index >= self.providers_to_fetch.len() {
            ::log::info!("Finished fetching from all {} providers", self.fetched_provider_ids.len());
            self.fetch_in_progress = false;
            self.view.redraw(cx);
            return;
        }

        let provider_id = &self.providers_to_fetch[index];
        ::log::info!("Starting fetch for provider {} (index {})", provider_id, index);

        let Some(store) = scope.data.get::<Store>() else { return };

        // Get client for this provider from ProvidersManager
        let Some(client) = store.providers_manager.clone_client(provider_id) else {
            ::log::warn!("No client for provider {}, skipping", provider_id);
            // Skip to next provider
            self.start_fetch_for_provider(cx, scope, index + 1);
            return;
        };

        // Get provider URL for BotId
        let provider_url = store.preferences.get_provider(provider_id)
            .map(|p| p.url.clone())
            .unwrap_or_default();

        // Set up the ChatController with this provider's client
        {
            let mut ctrl = self.chat_controller.lock().unwrap();
            ctrl.set_client(Some(Box::new(client)));

            // Don't set a default bot_id here - we'll restore the saved model
            // or select first available after models are loaded

            // Dispatch Load task to fetch models
            ::log::info!("Dispatching ChatTask::Load for provider {}", provider_id);
            ctrl.dispatch_task(ChatTask::Load);
        }

        self.current_provider_id = Some(provider_id.clone());
        self.fetch_index = index;
        self.fetch_in_progress = true;
        self.last_bots_count = 0;

        self.view.redraw(cx);
    }

    /// Check for loaded bots and continue sequential fetching
    fn check_for_loaded_bots(&mut self, cx: &mut Cx, scope: &mut Scope) {
        if !self.fetch_in_progress {
            return;
        }
        // Get the bots from the controller state
        let bots: Vec<Bot> = {
            let ctrl = self.chat_controller.lock().unwrap();
            ctrl.state().bots.clone()
        };

        // Check if we have new bots (fetch completed)
        if bots.is_empty() || bots.len() == self.last_bots_count {
            return;
        }

        self.last_bots_count = bots.len();

        // Update the ProvidersManager with the loaded bots
        let Some(store) = scope.data.get_mut::<Store>() else { return };

        // Store bots for current provider
        if let Some(ref current_provider) = self.current_provider_id {
            ::log::info!("Loaded {} bots from provider {}", bots.len(), current_provider);
            store.providers_manager.set_provider_bots(current_provider, bots.clone());

            if !self.fetched_provider_ids.contains(current_provider) {
                self.fetched_provider_ids.push(current_provider.clone());
            }
        }

        // Move to next provider
        let next_index = self.fetch_index + 1;
        if next_index < self.providers_to_fetch.len() {
            self.start_fetch_for_provider(cx, scope, next_index);
        } else {
            // All providers fetched - combine bots into ChatController
            ::log::info!("All providers fetched, {} total bots available", store.providers_manager.get_all_bots().len());
            self.fetch_in_progress = false;

            // Update ChatController with combined bots
            let all_bots = store.providers_manager.get_all_bots().to_vec();
            let num_bots = all_bots.len();
            ::log::info!("Setting {} bots on ChatController", num_bots);
            {
                let mut ctrl = self.chat_controller.lock().unwrap();
                // VecMutation::Set automatically converts to ChatStateMutation::MutateBots
                ctrl.dispatch_mutation(VecMutation::Set(all_bots));

                // Verify bots were set
                let controller_bots = ctrl.state().bots.len();
                ::log::info!("ChatController now has {} bots", controller_bots);
            }

            // Get bots before restore (restore may clear them due to set_client)
            let all_bots_for_reset = store.providers_manager.get_all_bots().to_vec();

            // Restore the saved model selection (this may switch client which clears bots)
            self.restore_saved_model(scope);

            // Force re-setting the controller on the Chat widget now that bots are loaded
            // The Chat widget's set_chat_controller has an early return if the Arc pointer
            // is the same, so we need to set it to None first to force re-propagation
            // IMPORTANT: Do this BEFORE dispatching mutations so the new plugin receives them
            {
                let mut chat_ref = self.view.chat(ids!(chat));
                // First set to None to clear the existing controller
                chat_ref.write().set_chat_controller(cx, None);
                // Then set to our controller again to force propagation to child widgets
                chat_ref.write().set_chat_controller(cx, Some(self.chat_controller.clone()));
            }

            // Re-set the bots after restore (set_client clears them)
            // Do this AFTER force re-setting controller so the new plugin sees the mutation
            {
                let mut ctrl = self.chat_controller.lock().unwrap();
                ctrl.dispatch_mutation(VecMutation::Set(all_bots_for_reset.clone()));
            }

            // Redraw both the view and explicitly the chat widget
            self.view.redraw(cx);
            self.view.chat(ids!(chat)).redraw(cx);
        }
    }

    /// Parse a BotId string into (model_name, provider) tuple
    /// BotId format: <id_len>;<model_id>@<provider>
    fn parse_bot_id_string(bot_id_str: &str) -> (String, String) {
        // Split on first ';' to get id_length and rest
        if let Some((id_length_str, rest)) = bot_id_str.split_once(';') {
            if let Ok(id_length) = id_length_str.parse::<usize>() {
                if rest.len() >= id_length + 1 {
                    let model_name = &rest[..id_length];
                    // Skip the '@' separator
                    let provider = &rest[id_length + 1..];
                    return (model_name.to_string(), provider.to_string());
                }
            }
        }
        // Fallback: return empty strings if parsing fails
        (String::new(), String::new())
    }

    /// Track model selection changes and save to preferences
    /// Only tracks changes after the saved model has been restored
    fn track_model_selection(&mut self, scope: &mut Scope) {
        // Don't track until we've restored the saved model
        // This prevents the initial load from overwriting the user's saved selection
        if !self.restored_saved_model {
            return;
        }

        // Get current bot_id from controller
        let current_bot_id: Option<BotId> = {
            let ctrl = self.chat_controller.lock().unwrap();
            ctrl.state().bot_id.clone()
        };

        let current_bot_id_str = current_bot_id.as_ref().map(|id| id.as_str().to_string());

        // Check if it changed from what we last saved
        if current_bot_id_str != self.last_saved_bot_id {
            if let Some(ref bot_id) = current_bot_id {
                let bot_id_str = bot_id.as_str().to_string();
                ::log::info!("Model selection changed to: {}", bot_id_str);

                // Switch to the correct provider's client for this model
                self.switch_to_provider_for_bot(bot_id, scope);

                // Save to preferences
                if let Some(store) = scope.data.get_mut::<Store>() {
                    store.preferences.set_current_chat_model(Some(bot_id_str.clone()));
                }

                self.last_saved_bot_id = Some(bot_id_str);
            } else {
                self.last_saved_bot_id = None;
            }
        }
    }

    /// Switch to the correct provider's client for a given bot
    fn switch_to_provider_for_bot(&mut self, bot_id: &BotId, scope: &mut Scope) {
        let Some(store) = scope.data.get::<Store>() else { return };

        // Find which provider this bot belongs to
        if let Some(provider_id) = store.providers_manager.get_provider_for_bot(bot_id) {
            // Only switch if it's a different provider
            if self.current_provider_id.as_deref() != Some(provider_id) {
                if let Some(client) = store.providers_manager.clone_client(provider_id) {
                    // Get all bots before switching (set_client clears them)
                    let all_bots = store.providers_manager.get_all_bots().to_vec();

                    {
                        let mut ctrl = self.chat_controller.lock().unwrap();
                        ctrl.set_client(Some(Box::new(client)));
                    }

                    self.current_provider_id = Some(provider_id.to_string());
                    ::log::info!("Switched to provider: {} for model", provider_id);

                    // Re-set the bots after set_client cleared them
                    {
                        let mut ctrl = self.chat_controller.lock().unwrap();
                        ctrl.dispatch_mutation(VecMutation::Set(all_bots));
                    }
                }
            }
        } else {
            ::log::warn!("Could not find provider for bot: {}", bot_id.as_str());
        }
    }

    /// Restore the saved model selection from preferences
    fn restore_saved_model(&mut self, scope: &mut Scope) {
        if self.restored_saved_model {
            return;
        }

        let Some(store) = scope.data.get::<Store>() else { return };

        // Get the saved model from preferences
        let saved_model = store.preferences.get_current_chat_model();
        let all_bots = store.providers_manager.get_all_bots();

        if all_bots.is_empty() {
            self.restored_saved_model = true;
            return;
        }

        // If no saved model, select the first available model
        if saved_model.is_none() {
            let first_bot_id = all_bots[0].id.clone();
            let first_bot_name = all_bots[0].name.clone();
            let _ = store;  // Release the borrow on store

            ::log::info!("No saved model, selecting first available: {}", first_bot_name);

            // Switch to the correct provider for this bot
            self.switch_to_provider_for_bot(&first_bot_id, scope);

            {
                let mut ctrl = self.chat_controller.lock().unwrap();
                ctrl.dispatch_mutation(ChatStateMutation::SetBotId(Some(first_bot_id.clone())));
            }
            self.last_saved_bot_id = Some(first_bot_id.as_str().to_string());
            self.restored_saved_model = true;
            return;
        }

        let saved_model = saved_model.unwrap().to_string();
        ::log::info!("Restoring saved model: {}", saved_model);

        // Parse the saved model to extract model name and provider
        // BotId format: <id_len>;<model_id>@<provider>
        let (saved_model_name, saved_provider) = Self::parse_bot_id_string(&saved_model);

        // Check if this model exists in the available bots
        let all_bots = store.providers_manager.get_all_bots();

        // First try exact match
        let mut matching_bot = all_bots.iter().find(|bot| bot.id.as_str() == saved_model);

        // If no exact match, try matching by model name (handling models/ prefix)
        if matching_bot.is_none() {
            matching_bot = all_bots.iter().find(|bot| {
                let bot_model_name = bot.id.id();
                let bot_provider = bot.id.provider();
                // Match if providers are the same and either:
                // 1. Model names match exactly
                // 2. Bot model is "models/<saved_model>"
                // 3. Saved model is "models/<bot_model>"
                bot_provider == saved_provider && (
                    bot_model_name == saved_model_name ||
                    bot_model_name == format!("models/{}", saved_model_name) ||
                    saved_model_name == format!("models/{}", bot_model_name)
                )
            });
        }

        if let Some(bot) = matching_bot {
            ::log::info!("Found saved model, selecting: {}", bot.name);

            let matched_bot_id = bot.id.clone();
            let matched_bot_id_str = bot.id.as_str().to_string();

            // Switch to the correct provider for this bot
            self.switch_to_provider_for_bot(&matched_bot_id, scope);

            // Set the bot_id on the controller
            {
                let mut ctrl = self.chat_controller.lock().unwrap();
                ctrl.dispatch_mutation(ChatStateMutation::SetBotId(Some(matched_bot_id)));
            }

            // Update our tracking with the actual matched bot ID (for future exact matching)
            self.last_saved_bot_id = Some(matched_bot_id_str.clone());

            // Also save the correct ID to preferences for future exact matching
            if let Some(store) = scope.data.get_mut::<Store>() {
                if matched_bot_id_str != saved_model {
                    store.preferences.set_current_chat_model(Some(matched_bot_id_str));
                }
            }
        } else {
            // Saved model not found, select first available
            ::log::warn!("Saved model '{}' not found, selecting first available", saved_model);
            let first_bot_id = all_bots[0].id.clone();

            // Switch to the correct provider for this bot
            self.switch_to_provider_for_bot(&first_bot_id, scope);

            {
                let mut ctrl = self.chat_controller.lock().unwrap();
                ctrl.dispatch_mutation(ChatStateMutation::SetBotId(Some(first_bot_id.clone())));
            }
            self.last_saved_bot_id = Some(first_bot_id.as_str().to_string());
        }

        self.restored_saved_model = true;
    }
}
