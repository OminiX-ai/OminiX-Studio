use serde::{Deserialize, Serialize};

/// Unique identifier for a provider
pub type ProviderId = String;

/// Determines the API format used by the provider
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum ProviderType {
    #[default]
    #[serde(alias = "OpenAI")]
    OpenAi,
    #[serde(alias = "OpenAIRealtime")]
    OpenAiRealtime,
    MoFa,
    MolyServer,
}

/// Connection status of a provider
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum ProviderConnectionStatus {
    #[default]
    NotConnected,
    Connecting,
    Connected,
    Error(String),
}

/// Provider preferences stored in JSON
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProviderPreferences {
    /// Unique identifier for the provider
    #[serde(default)]
    pub id: ProviderId,
    pub name: String,
    pub url: String,
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub provider_type: ProviderType,
    /// (model_name, enabled) pairs
    #[serde(default)]
    pub models: Vec<(String, bool)>,
    #[serde(default)]
    pub was_customly_added: bool,
    /// Custom system prompt (for Realtime providers)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    /// Whether MCP tools are enabled
    #[serde(default = "default_true")]
    pub tools_enabled: bool,
}

fn default_true() -> bool {
    true
}

impl Default for ProviderPreferences {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            url: String::new(),
            api_key: None,
            enabled: true,
            provider_type: ProviderType::OpenAi,
            models: Vec::new(),
            was_customly_added: false,
            system_prompt: None,
            tools_enabled: true,
        }
    }
}

impl ProviderPreferences {
    pub fn new(id: &str, name: &str, url: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            url: url.to_string(),
            ..Default::default()
        }
    }

    pub fn has_api_key(&self) -> bool {
        self.api_key.as_ref().map_or(false, |k| !k.is_empty())
    }
}

/// Get list of supported providers with default URLs
pub fn get_supported_providers() -> Vec<ProviderPreferences> {
    vec![
        ProviderPreferences {
            id: "openai".to_string(),
            name: "OpenAI".to_string(),
            url: "https://api.openai.com/v1".to_string(),
            provider_type: ProviderType::OpenAi,
            ..Default::default()
        },
        ProviderPreferences {
            id: "anthropic".to_string(),
            name: "Anthropic".to_string(),
            url: "https://api.anthropic.com/v1".to_string(),
            provider_type: ProviderType::OpenAi,
            ..Default::default()
        },
        ProviderPreferences {
            id: "gemini".to_string(),
            name: "Google Gemini".to_string(),
            url: "https://generativelanguage.googleapis.com/v1beta/openai".to_string(),
            provider_type: ProviderType::OpenAi,
            ..Default::default()
        },
        ProviderPreferences {
            id: "ollama".to_string(),
            name: "Ollama (Local)".to_string(),
            url: "http://localhost:11434/v1".to_string(),
            provider_type: ProviderType::OpenAi,
            ..Default::default()
        },
        ProviderPreferences {
            id: "groq".to_string(),
            name: "Groq".to_string(),
            url: "https://api.groq.com/openai/v1".to_string(),
            provider_type: ProviderType::OpenAi,
            ..Default::default()
        },
        ProviderPreferences {
            id: "deepseek".to_string(),
            name: "DeepSeek".to_string(),
            url: "https://api.deepseek.com/v1".to_string(),
            provider_type: ProviderType::OpenAi,
            ..Default::default()
        },
    ]
}
