use chrono::{DateTime, Utc};
use moly_kit::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub type ChatId = u128;

const CHATS_DIR: &str = "chats";

/// Serializable chat data for persistence
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatData {
    pub id: ChatId,
    pub title: String,
    pub bot_id: Option<BotId>,
    pub messages: Vec<Message>,
    pub created_at: DateTime<Utc>,
    pub accessed_at: DateTime<Utc>,
}

impl ChatData {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            id: now.timestamp_millis() as u128,
            title: "New Chat".to_string(),
            bot_id: None,
            messages: Vec::new(),
            created_at: now,
            accessed_at: now,
        }
    }

    /// Get the filename for this chat
    fn file_name(&self) -> String {
        format!("{}.chat.json", self.id)
    }

    /// Save this chat to disk
    pub fn save(&self, chats_dir: &PathBuf) {
        let path = chats_dir.join(self.file_name());

        match serde_json::to_string_pretty(self) {
            Ok(json) => {
                if let Err(e) = std::fs::write(&path, &json) {
                    log::error!("Failed to save chat {}: {:?}", self.id, e);
                } else {
                    log::debug!("Saved chat {} to {:?}", self.id, path);
                }
            }
            Err(e) => {
                log::error!("Failed to serialize chat {}: {:?}", self.id, e);
            }
        }
    }

    /// Load a chat from disk
    pub fn load(path: &PathBuf) -> Option<Self> {
        match std::fs::read_to_string(path) {
            Ok(contents) => {
                match serde_json::from_str::<ChatData>(&contents) {
                    Ok(chat) => {
                        log::debug!("Loaded chat {} from {:?}", chat.id, path);
                        Some(chat)
                    }
                    Err(e) => {
                        log::error!("Failed to parse chat from {:?}: {:?}", path, e);
                        None
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to read chat from {:?}: {:?}", path, e);
                None
            }
        }
    }

    /// Delete the chat file from disk
    pub fn delete_file(&self, chats_dir: &PathBuf) {
        let path = chats_dir.join(self.file_name());
        if let Err(e) = std::fs::remove_file(&path) {
            log::warn!("Failed to delete chat file {:?}: {:?}", path, e);
        } else {
            log::debug!("Deleted chat file {:?}", path);
        }
    }

    /// Update the accessed_at timestamp
    pub fn update_accessed_at(&mut self) {
        self.accessed_at = Utc::now();
    }

    /// Generate a title from the first message if title is default
    pub fn maybe_update_title_from_messages(&mut self) {
        use moly_kit::aitk::protocol::EntityId;

        if self.title == "New Chat" && !self.messages.is_empty() {
            // Find the first user message
            if let Some(msg) = self.messages.iter().find(|m| matches!(m.from, EntityId::User)) {
                let text = msg.content.text.trim();
                if !text.is_empty() {
                    // Truncate to first 50 chars
                    let title = if text.len() > 50 {
                        format!("{}...", &text[..50])
                    } else {
                        text.to_string()
                    };
                    self.title = title;
                }
            }
        }
    }
}

impl Default for ChatData {
    fn default() -> Self {
        Self::new()
    }
}

/// Manages chat sessions with persistence
pub struct Chats {
    pub saved_chats: Vec<ChatData>,
    pub current_chat_id: Option<ChatId>,
    chats_dir: PathBuf,
}

impl Chats {
    /// Create a new Chats manager (does not load from disk)
    pub fn new() -> Self {
        Self {
            saved_chats: Vec::new(),
            current_chat_id: None,
            chats_dir: Self::get_chats_dir(),
        }
    }

    /// Get the chats directory path (~/.moly/chats/)
    fn get_chats_dir() -> PathBuf {
        if let Some(home) = dirs::home_dir() {
            home.join(".moly").join(CHATS_DIR)
        } else {
            PathBuf::from(CHATS_DIR)
        }
    }

    /// Load all chats from disk
    pub fn load() -> Self {
        let chats_dir = Self::get_chats_dir();
        log::info!("Loading chats from {:?}", chats_dir);

        let mut chats = Chats {
            saved_chats: Vec::new(),
            current_chat_id: None,
            chats_dir: chats_dir.clone(),
        };

        // Ensure directory exists
        if let Err(e) = std::fs::create_dir_all(&chats_dir) {
            log::error!("Failed to create chats directory: {:?}", e);
            return chats;
        }

        // Load all .chat.json files
        match std::fs::read_dir(&chats_dir) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().map_or(false, |e| e == "json") {
                        if let Some(chat) = ChatData::load(&path) {
                            chats.saved_chats.push(chat);
                        }
                    }
                }
                log::info!("Loaded {} chats from disk", chats.saved_chats.len());

                // Sort by accessed_at descending (most recent first)
                chats.saved_chats.sort_by(|a, b| b.accessed_at.cmp(&a.accessed_at));

                // Set current chat to most recently accessed
                if let Some(first) = chats.saved_chats.first() {
                    chats.current_chat_id = Some(first.id);
                }
            }
            Err(e) => {
                log::warn!("Could not read chats directory: {:?}", e);
            }
        }

        chats
    }

    pub fn get_current_chat(&self) -> Option<&ChatData> {
        self.current_chat_id
            .and_then(|id| self.saved_chats.iter().find(|c| c.id == id))
    }

    pub fn get_current_chat_mut(&mut self) -> Option<&mut ChatData> {
        self.current_chat_id
            .and_then(|id| self.saved_chats.iter_mut().find(|c| c.id == id))
    }

    /// Set the current chat and save the access time
    pub fn set_current_chat(&mut self, chat_id: Option<ChatId>) {
        self.current_chat_id = chat_id;
        let chats_dir = self.chats_dir.clone();
        if let Some(chat) = self.get_current_chat_mut() {
            chat.update_accessed_at();
            chat.save(&chats_dir);
        }
    }

    /// Create a new chat and save it to disk
    pub fn create_chat(&mut self, bot_id: Option<BotId>) -> ChatId {
        let mut chat = ChatData::new();

        // Use provided bot_id or inherit from last chat
        if let Some(bid) = bot_id {
            chat.bot_id = Some(bid);
        } else if let Some(last_chat) = self.saved_chats.first() {
            chat.bot_id = last_chat.bot_id.clone();
        }

        let id = chat.id;
        chat.save(&self.chats_dir);
        self.saved_chats.insert(0, chat); // Insert at front (most recent)
        self.current_chat_id = Some(id);
        log::info!("Created new chat {}", id);
        id
    }

    pub fn get_chat_by_id(&self, chat_id: ChatId) -> Option<&ChatData> {
        self.saved_chats.iter().find(|c| c.id == chat_id)
    }

    pub fn get_chat_by_id_mut(&mut self, chat_id: ChatId) -> Option<&mut ChatData> {
        self.saved_chats.iter_mut().find(|c| c.id == chat_id)
    }

    /// Delete a chat from memory and disk
    pub fn delete_chat(&mut self, chat_id: ChatId) {
        // Find and remove the chat, get it for file deletion
        if let Some(pos) = self.saved_chats.iter().position(|c| c.id == chat_id) {
            let chat = self.saved_chats.remove(pos);
            chat.delete_file(&self.chats_dir);
            log::info!("Deleted chat {}", chat_id);
        }

        // Update current chat if needed
        if self.current_chat_id == Some(chat_id) {
            self.current_chat_id = self.saved_chats.first().map(|c| c.id);
        }
    }

    /// Save the current chat to disk
    pub fn save_current_chat(&self) {
        if let Some(chat) = self.get_current_chat() {
            chat.save(&self.chats_dir);
        }
    }

    /// Save a specific chat by ID
    pub fn save_chat(&self, chat_id: ChatId) {
        if let Some(chat) = self.get_chat_by_id(chat_id) {
            chat.save(&self.chats_dir);
        }
    }

    /// Get chats sorted by most recently accessed
    pub fn get_sorted_chats(&self) -> Vec<&ChatData> {
        let mut chats: Vec<_> = self.saved_chats.iter().collect();
        chats.sort_by(|a, b| b.accessed_at.cmp(&a.accessed_at));
        chats
    }

    /// Update a chat's messages and save
    pub fn update_chat_messages(&mut self, chat_id: ChatId, mut messages: Vec<Message>) {
        let chats_dir = self.chats_dir.clone();
        if let Some(chat) = self.get_chat_by_id_mut(chat_id) {
            // Reset is_writing flag on all messages before storing
            // This ensures the in-memory copy is also clean (is_writing is not persisted via serde skip)
            for msg in &mut messages {
                msg.metadata.is_writing = false;
            }
            chat.messages = messages;
            chat.maybe_update_title_from_messages();
            chat.save(&chats_dir);
        }
    }

    /// Update a chat's bot and save
    pub fn update_chat_bot(&mut self, chat_id: ChatId, bot_id: Option<BotId>) {
        let chats_dir = self.chats_dir.clone();
        if let Some(chat) = self.get_chat_by_id_mut(chat_id) {
            chat.bot_id = bot_id;
            chat.save(&chats_dir);
        }
    }

    /// Get the chats directory path
    pub fn chats_dir(&self) -> &PathBuf {
        &self.chats_dir
    }
}

impl Default for Chats {
    fn default() -> Self {
        Self::new()
    }
}
