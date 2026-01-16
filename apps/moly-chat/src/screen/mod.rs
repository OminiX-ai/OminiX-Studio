//! Chat Screen Widget Implementation

pub mod design;

use makepad_widgets::*;
use moly_kit::prelude::*;
use moly_kit::aitk::controllers::chat::{ChatStateMutation, ChatTask};
use moly_kit::aitk::protocol::{Bot, BotId, EntityAvatar};
use moly_kit::widgets::model_selector::{BotGroup, create_lookup_grouping};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use moly_data::{ChatId, Store};

// Actions emitted by ChatHistoryPanel
#[derive(Clone, Debug, DefaultNone)]
pub enum ChatHistoryAction {
    None,
    NewChat,
    SelectChat(ChatId),
}

/// ChatHistoryItem Widget - handles its own click events
#[derive(Live, LiveHook, Widget)]
pub struct ChatHistoryItem {
    #[deref]
    view: View,

    #[rust]
    chat_id: Option<ChatId>,
}

impl Widget for ChatHistoryItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ChatHistoryItem {
    pub fn set_chat_id(&mut self, id: ChatId) {
        self.chat_id = Some(id);
    }

    /// Check if this item was clicked - similar to EntityButton pattern
    pub fn clicked(&self, actions: &Actions) -> bool {
        if let Some(item) = actions.find_widget_action(self.view.widget_uid()) {
            if let ViewAction::FingerDown(fd) = item.cast() {
                return fd.tap_count == 1;
            }
        }
        false
    }

    pub fn get_chat_id(&self) -> Option<ChatId> {
        self.chat_id
    }
}

impl ChatHistoryItemRef {
    pub fn set_chat_id(&self, id: ChatId) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_chat_id(id);
        }
    }

    pub fn clicked(&self, actions: &Actions) -> bool {
        if let Some(inner) = self.borrow() {
            inner.clicked(actions)
        } else {
            false
        }
    }

    pub fn get_chat_id(&self) -> Option<ChatId> {
        if let Some(inner) = self.borrow() {
            inner.get_chat_id()
        } else {
            None
        }
    }
}

/// Separate widget for chat history panel - handles its own PortalList drawing
#[derive(Live, LiveHook, Widget)]
pub struct ChatHistoryPanel {
    #[deref]
    view: View,

    #[rust]
    chat_count: usize,

    #[rust]
    current_chat_id: Option<ChatId>,

    #[rust]
    dark_mode: f64,
}

impl Widget for ChatHistoryPanel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // Delegate events directly to view (like moly-ai pattern)
        self.view.handle_event(cx, event, scope);

        // Use WidgetMatchEvent pattern for handling actions
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Get data from store
        if let Some(store) = scope.data.get::<Store>() {
            self.dark_mode = if store.is_dark_mode() { 1.0 } else { 0.0 };
            self.chat_count = store.chats.saved_chats.len();
        }

        // Apply dark mode to panel
        self.view.apply_over(cx, live! {
            draw_bg: { dark_mode: (self.dark_mode) }
        });
        self.view.button(ids!(new_chat_button)).apply_over(cx, live! {
            draw_bg: { dark_mode: (self.dark_mode) }
        });

        // Get the history_list PortalList
        let history_list = self.view.portal_list(ids!(history_list));
        let history_list_uid = history_list.widget_uid();

        // Draw with PortalList handling
        while let Some(widget) = self.view.draw_walk(cx, scope, walk).step() {
            if widget.widget_uid() == history_list_uid {
                if let Some(mut list) = widget.as_portal_list().borrow_mut() {
                    list.set_item_range(cx, 0, self.chat_count);

                    while let Some(item_id) = list.next_visible_item(cx) {
                        if item_id < self.chat_count {
                            // Get chat data
                            let (chat_id, title, date_str, is_selected) = if let Some(store) = scope.data.get::<Store>() {
                                if let Some(chat) = store.chats.saved_chats.get(item_id) {
                                    let id = chat.id;
                                    let title = chat.title.clone();
                                    let date = chat.accessed_at.format("%b %d").to_string();
                                    let selected = self.current_chat_id == Some(chat.id);
                                    (id, title, date, selected)
                                } else {
                                    continue;
                                }
                            } else {
                                continue;
                            };

                            // Draw the item - get as ChatHistoryItem widget
                            let item_widget = list.item(cx, item_id, live_id!(ChatHistoryItem));

                            // Set the chat_id on the item so we can retrieve it in handle_actions
                            item_widget.as_chat_history_item().set_chat_id(chat_id);

                            let selected_value = if is_selected { 1.0 } else { 0.0 };

                            item_widget.apply_over(cx, live! {
                                draw_bg: {
                                    dark_mode: (self.dark_mode),
                                    selected: (selected_value)
                                }
                            });

                            item_widget.label(ids!(title_label)).set_text(cx, &title);
                            item_widget.label(ids!(title_label)).apply_over(cx, live! {
                                draw_text: { dark_mode: (self.dark_mode) }
                            });

                            item_widget.label(ids!(date_label)).set_text(cx, &date_str);
                            item_widget.label(ids!(date_label)).apply_over(cx, live! {
                                draw_text: { dark_mode: (self.dark_mode) }
                            });

                            item_widget.draw_all(cx, scope);
                        }
                    }
                }
            }
        }

        DrawStep::done()
    }
}

impl ChatHistoryPanel {
    pub fn set_current_chat(&mut self, chat_id: Option<ChatId>) {
        self.current_chat_id = chat_id;
    }
}

impl WidgetMatchEvent for ChatHistoryPanel {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        // Handle new chat button click
        let btn = self.button(ids!(new_chat_button));
        if btn.clicked(actions) {
            ::log::info!("New chat button clicked");
            cx.action(ChatHistoryAction::NewChat);
        }

        // Handle chat history item clicks from PortalList
        // Use the ChatHistoryItem widget's clicked() method (like moly-ai's EntityButton pattern)
        let history_list = self.portal_list(ids!(history_list));
        for (_item_id, item) in history_list.items_with_actions(actions) {
            let history_item = item.as_chat_history_item();
            if history_item.clicked(actions) {
                if let Some(chat_id) = history_item.get_chat_id() {
                    ::log::info!("Chat history item clicked: {:?}", chat_id);
                    cx.action(ChatHistoryAction::SelectChat(chat_id));
                }
            }
        }
    }
}

impl ChatHistoryPanelRef {
    pub fn set_current_chat(&self, chat_id: Option<ChatId>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_current_chat(chat_id);
        }
    }
}

#[derive(Live, Widget)]
pub struct ChatApp {
    #[deref]
    pub view: View,

    /// Provider icons loaded from live_design for use in model selector and chat messages
    #[live]
    provider_icons: Vec<LiveDependency>,

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

    /// Current chat ID being edited
    #[rust]
    current_chat_id: Option<ChatId>,

    /// Last message count we synced (to detect changes)
    #[rust]
    last_synced_message_count: usize,

    /// Whether there was a message being written in the last sync check
    #[rust]
    had_writing_message: bool,

    /// Content length of last message at last sync (to detect streaming content)
    #[rust]
    last_synced_content_len: usize,

    /// Whether we've initialized the chat from persistence
    #[rust]
    chat_initialized: bool,
}

impl LiveHook for ChatApp {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        // Initialize the controller with basic spawner
        let mut controller = self.chat_controller.lock().unwrap();
        controller.set_basic_spawner();
    }
}

impl ChatApp {
    /// Get provider icon LiveDependency from the loaded list
    fn get_provider_icon(&self, provider_id: &str) -> Option<&LiveDependency> {
        // Icons are stored in order: openai, anthropic, gemini, ollama, deepseek, openrouter, siliconflow
        let index = match provider_id {
            "openai" => Some(0),
            "anthropic" => Some(1),
            "gemini" => Some(2),
            "ollama" => Some(3),
            "deepseek" => Some(4),
            "openrouter" => Some(5),
            "siliconflow" => Some(6),
            _ => None,
        };
        index.and_then(|i| self.provider_icons.get(i))
    }

    /// Get provider icon path string from the loaded LiveDependency list
    fn get_provider_icon_path(&self, provider_id: &str) -> Option<String> {
        self.get_provider_icon(provider_id).map(|dep| dep.as_str().to_string())
    }

    /// Get provider display name
    fn get_provider_display_name(provider_id: &str) -> &'static str {
        match provider_id {
            "openai" => "OpenAI",
            "anthropic" => "Anthropic",
            "gemini" => "Google Gemini",
            "ollama" => "Ollama",
            "deepseek" => "DeepSeek",
            "groq" => "Groq",
            "openrouter" => "OpenRouter",
            "siliconflow" => "SiliconFlow",
            _ => "Unknown",
        }
    }

    /// Set up the grouping function for the model selector
    fn setup_model_selector_grouping(&mut self, scope: &mut Scope) {
        let Some(store) = scope.data.get::<Store>() else { return };

        // Build lookup table: BotId -> BotGroup
        let mut bot_groups: HashMap<BotId, BotGroup> = HashMap::new();

        for bot in store.providers_manager.get_all_bots() {
            // Get provider ID from ProvidersManager (not from bot.id.provider() which returns URL)
            let provider_id = store.providers_manager.get_provider_for_bot(&bot.id)
                .unwrap_or_else(|| bot.id.provider()); // fallback to URL if not found

            let icon = self.get_provider_icon_path(provider_id)
                .map(|path| EntityAvatar::Image(path));
            let label = Self::get_provider_display_name(provider_id).to_string();

            bot_groups.insert(
                bot.id.clone(),
                BotGroup {
                    id: provider_id.to_string(),
                    label,
                    icon,
                },
            );
        }

        // Create grouping function
        let grouping_fn = create_lookup_grouping(move |bot_id: &BotId| {
            bot_groups.get(bot_id).cloned()
        });

        // Set grouping on the ModelSelector inside PromptInput
        let chat = self.view.chat(ids!(chat));
        chat.read()
            .prompt_input_ref()
            .widget(ids!(model_selector))
            .as_model_selector()
            .set_grouping(Some(grouping_fn));
    }

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

    /// Initialize the chat from persistence (load or create the current chat)
    fn maybe_initialize_chat(&mut self, cx: &mut Cx, scope: &mut Scope) {
        if self.chat_initialized {
            return;
        }

        let Some(store) = scope.data.get_mut::<Store>() else { return };

        // Get or create the current chat
        let chat_id = if let Some(id) = store.chats.current_chat_id {
            id
        } else {
            // No current chat, create one
            let current_bot_id = {
                let ctrl = self.chat_controller.lock().unwrap();
                ctrl.state().bot_id.clone()
            };
            ::log::info!("Creating new chat");
            store.chats.create_chat(current_bot_id)
        };

        self.current_chat_id = Some(chat_id);

        // Load messages from the chat into the controller
        if let Some(chat) = store.chats.get_chat_by_id(chat_id) {
            let messages = chat.messages.clone();
            let message_count = messages.len();

            if !messages.is_empty() {
                ::log::info!("Loading {} messages from chat {}", message_count, chat_id);
                let mut ctrl = self.chat_controller.lock().unwrap();
                ctrl.dispatch_mutation(VecMutation::Set(messages));
            }

            self.last_synced_message_count = message_count;

            // Also restore the bot_id if it was saved with the chat
            if let Some(ref bot_id) = chat.bot_id {
                ::log::info!("Chat {} has saved bot_id: {}", chat_id, bot_id.as_str());
                // We'll let restore_saved_model handle the bot selection
            }
        }

        self.chat_initialized = true;
        self.view.redraw(cx);
    }

    /// Sync messages from controller to persistence when they change
    fn sync_messages_to_persistence(&mut self, scope: &mut Scope) {
        let Some(chat_id) = self.current_chat_id else { return };

        // Get current messages from controller
        let (messages, message_count, has_writing_message, last_msg_content_len) = {
            let ctrl = self.chat_controller.lock().unwrap();
            let msgs = ctrl.state().messages.clone();
            let count = msgs.len();
            // Check if any message is still being written
            let writing = msgs.iter().any(|m| m.metadata.is_writing);
            // Get the content length of the last message (to detect content changes)
            let last_len = msgs.last().map(|m| m.content.text.len()).unwrap_or(0);
            (msgs, count, writing, last_len)
        };

        // Sync if:
        // 1. Message count changed (new message added)
        // 2. OR there was a writing message that just finished (content now complete)
        // 3. OR the last message content has grown (streaming in progress or just finished)
        let count_changed = message_count != self.last_synced_message_count;
        let writing_finished = self.had_writing_message && !has_writing_message;
        let content_changed = last_msg_content_len != self.last_synced_content_len;

        if !count_changed && !writing_finished && !content_changed {
            return;
        }

        if count_changed {
            ::log::debug!("Messages count changed: {} -> {}, syncing to persistence",
                self.last_synced_message_count, message_count);
        }
        if writing_finished {
            ::log::debug!("Message finished streaming, syncing to persistence");
        }
        if content_changed {
            ::log::debug!("Message content changed: {} -> {} bytes, syncing to persistence",
                self.last_synced_content_len, last_msg_content_len);
        }

        // Update the chat in persistence
        if let Some(store) = scope.data.get_mut::<Store>() {
            store.chats.update_chat_messages(chat_id, messages);
        }

        self.last_synced_message_count = message_count;
        self.had_writing_message = has_writing_message;
        self.last_synced_content_len = last_msg_content_len;
    }

    /// Sync the current bot_id to the chat when it changes
    fn sync_bot_to_chat(&mut self, scope: &mut Scope) {
        let Some(chat_id) = self.current_chat_id else { return };

        // Get current bot_id from controller
        let current_bot_id = {
            let ctrl = self.chat_controller.lock().unwrap();
            ctrl.state().bot_id.clone()
        };

        // Update the chat's bot_id
        if let Some(store) = scope.data.get_mut::<Store>() {
            if let Some(chat) = store.chats.get_chat_by_id(chat_id) {
                // Only update if different
                if chat.bot_id != current_bot_id {
                    store.chats.update_chat_bot(chat_id, current_bot_id);
                }
            }
        }
    }

    /// Create a new chat session
    pub fn create_new_chat(&mut self, cx: &mut Cx, scope: &mut Scope) {
        let Some(store) = scope.data.get_mut::<Store>() else { return };

        // Get current bot_id and all bots to use for new chat
        let (current_bot_id, all_bots) = {
            let ctrl = self.chat_controller.lock().unwrap();
            (ctrl.state().bot_id.clone(), ctrl.state().bots.clone())
        };

        // Create new chat
        let chat_id = store.chats.create_chat(current_bot_id.clone());
        self.current_chat_id = Some(chat_id);

        // Force reset the controller on the Chat widget to ensure clean state
        // This is needed because the Messages widget caches state internally
        {
            let mut chat_ref = self.view.chat(ids!(chat));
            chat_ref.write().set_chat_controller(cx, None);
            chat_ref.write().set_chat_controller(cx, Some(self.chat_controller.clone()));
        }

        // Clear messages in controller and re-set bots (since set_chat_controller may clear them)
        {
            let mut ctrl = self.chat_controller.lock().unwrap();
            ctrl.dispatch_mutation(VecMutation::<Message>::Set(vec![]));
            ctrl.dispatch_mutation(VecMutation::Set(all_bots));
            // Re-set the bot_id
            if let Some(bot_id) = current_bot_id {
                ctrl.dispatch_mutation(ChatStateMutation::SetBotId(Some(bot_id)));
            }
        }

        // Reset all sync tracking state for the new empty chat
        self.last_synced_message_count = 0;
        self.had_writing_message = false;
        self.last_synced_content_len = 0;

        // Reset scroll position
        self.view.chat(ids!(chat)).write().messages_ref().write().instant_scroll_to_bottom(cx);

        ::log::info!("Created new chat {}", chat_id);
        self.view.redraw(cx);
    }

    /// Switch to a different chat
    pub fn switch_to_chat(&mut self, cx: &mut Cx, scope: &mut Scope, chat_id: ChatId) {
        if self.current_chat_id == Some(chat_id) {
            return;
        }

        let Some(store) = scope.data.get_mut::<Store>() else { return };

        // Set as current chat in persistence
        store.chats.set_current_chat(Some(chat_id));
        self.current_chat_id = Some(chat_id);

        // Load the chat's messages into controller
        if let Some(chat) = store.chats.get_chat_by_id(chat_id) {
            // Clone messages and reset is_writing flag on all of them
            // This is needed because in-memory messages may still have is_writing: true
            // from when they were being streamed, even though it's not persisted to disk
            let mut messages = chat.messages.clone();
            for msg in &mut messages {
                msg.metadata.is_writing = false;
            }
            let message_count = messages.len();
            let last_content_len = messages.last().map(|m| m.content.text.len()).unwrap_or(0);

            ::log::info!("Switching to chat {} with {} messages", chat_id, message_count);

            {
                let mut ctrl = self.chat_controller.lock().unwrap();
                ctrl.dispatch_mutation(VecMutation::Set(messages));

                // Also restore the bot if saved with the chat
                if let Some(ref bot_id) = chat.bot_id {
                    ctrl.dispatch_mutation(ChatStateMutation::SetBotId(Some(bot_id.clone())));
                }
            }

            // Reset all sync tracking state for the loaded chat
            self.last_synced_message_count = message_count;
            self.had_writing_message = false;
            self.last_synced_content_len = last_content_len;

            // Reset the scroll position to bottom to avoid PortalList first_id > range_end errors
            // This is needed because switching from a chat with many messages to one with fewer
            // can leave the scroll position pointing to a non-existent message index
            self.view.chat(ids!(chat)).write().messages_ref().write().instant_scroll_to_bottom(cx);
        }

        self.view.redraw(cx);
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

        // Initialize chat from persistence (load or create)
        self.maybe_initialize_chat(cx, scope);

        // Track model selection changes and save to preferences
        self.track_model_selection(scope);

        // Sync messages to persistence when they change
        self.sync_messages_to_persistence(scope);

        // Sync bot selection to current chat
        self.sync_bot_to_chat(scope);

        // Delegate events directly to view (like moly-ai does)
        // Don't use capture_actions as it can interfere with nested widget event handling
        self.view.handle_event(cx, event, scope);

        // Use WidgetMatchEvent pattern for handling actions
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Get dark mode state
        let dark_mode_value = if let Some(store) = scope.data.get::<Store>() {
            if store.is_dark_mode() { 1.0 } else { 0.0 }
        } else {
            0.0
        };

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

        // Apply dark mode to separator
        self.view.view(ids!(separator)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode_value) }
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

        // Update history panel's current chat
        self.view.chat_history_panel(ids!(history_panel)).set_current_chat(self.current_chat_id);

        // Simply delegate to view's draw_walk - no step() pattern needed
        // ChatHistoryPanel handles its own PortalList, Chat handles its own
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for ChatApp {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        // Handle ChatHistoryPanel actions
        for action in actions.iter() {
            if let ChatHistoryAction::NewChat = action.cast() {
                self.create_new_chat(cx, scope);
            }
            if let ChatHistoryAction::SelectChat(chat_id) = action.cast() {
                self.switch_to_chat(cx, scope, chat_id);
            }
        }
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
        let _provider_url = store.preferences.get_provider(provider_id)
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

    /// Apply provider icon to all bots' avatars
    fn apply_provider_icon_to_bots(bots: &mut Vec<Bot>, icon_path: Option<String>) {
        if let Some(path) = icon_path {
            for bot in bots.iter_mut() {
                bot.avatar = EntityAvatar::Image(path.clone());
            }
        }
    }

    /// Check for loaded bots and continue sequential fetching
    fn check_for_loaded_bots(&mut self, cx: &mut Cx, scope: &mut Scope) {
        if !self.fetch_in_progress {
            return;
        }
        // Get the bots from the controller state
        let mut bots: Vec<Bot> = {
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
            // Apply provider icon to bot avatars before storing
            let icon_path = self.get_provider_icon_path(current_provider);
            Self::apply_provider_icon_to_bots(&mut bots, icon_path);

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

            // Set up grouping with provider icons for the model selector
            self.setup_model_selector_grouping(scope);

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
