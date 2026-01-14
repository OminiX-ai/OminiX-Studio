use chrono::{DateTime, Utc};
use moly_kit::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub type ChatId = u128;

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
}

/// Manages chat sessions
pub struct Chats {
    pub saved_chats: Vec<ChatData>,
    pub current_chat_id: Option<ChatId>,
    chats_dir: PathBuf,
}

impl Chats {
    pub fn new() -> Self {
        Self {
            saved_chats: Vec::new(),
            current_chat_id: None,
            chats_dir: PathBuf::from("chats"),
        }
    }

    pub fn get_current_chat(&self) -> Option<&ChatData> {
        self.current_chat_id
            .and_then(|id| self.saved_chats.iter().find(|c| c.id == id))
    }

    pub fn get_current_chat_mut(&mut self) -> Option<&mut ChatData> {
        self.current_chat_id
            .and_then(|id| self.saved_chats.iter_mut().find(|c| c.id == id))
    }

    pub fn set_current_chat(&mut self, chat_id: Option<ChatId>) {
        self.current_chat_id = chat_id;
        if let Some(chat) = self.get_current_chat_mut() {
            chat.accessed_at = Utc::now();
        }
    }

    pub fn create_chat(&mut self) -> ChatId {
        let chat = ChatData::new();
        let id = chat.id;
        self.saved_chats.push(chat);
        self.current_chat_id = Some(id);
        id
    }

    pub fn get_chat_by_id(&self, chat_id: ChatId) -> Option<&ChatData> {
        self.saved_chats.iter().find(|c| c.id == chat_id)
    }

    pub fn delete_chat(&mut self, chat_id: ChatId) {
        self.saved_chats.retain(|c| c.id != chat_id);
        if self.current_chat_id == Some(chat_id) {
            self.current_chat_id = self.saved_chats.first().map(|c| c.id);
        }
    }

    /// Get chats sorted by most recently accessed
    pub fn get_sorted_chats(&self) -> Vec<&ChatData> {
        let mut chats: Vec<_> = self.saved_chats.iter().collect();
        chats.sort_by(|a, b| b.accessed_at.cmp(&a.accessed_at));
        chats
    }
}
