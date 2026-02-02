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
    /// OminiX local image generation (FLUX, Z-Image)
    #[serde(alias = "OminiXImage")]
    OminiXImage,
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
    /// Whether A2UI (AI-to-UI) generation is enabled for this provider
    /// Only applicable for OpenAI-compatible providers that support function calling
    #[serde(default)]
    pub a2ui_enabled: bool,
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
            a2ui_enabled: false,
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

    /// Check if this provider requires an API key to function
    pub fn requires_api_key(&self) -> bool {
        match self.provider_type {
            // Local providers don't require API keys
            ProviderType::OminiXImage => false,
            // All other providers require API keys
            _ => true,
        }
    }

    /// Check if this provider is ready to use (enabled and has required credentials)
    pub fn is_ready(&self) -> bool {
        self.enabled && (!self.requires_api_key() || self.has_api_key())
    }

    /// Check if this provider supports A2UI (must be OpenAI-compatible with function calling)
    pub fn supports_a2ui(&self) -> bool {
        matches!(self.provider_type, ProviderType::OpenAi)
    }

    /// Check if A2UI is both supported and enabled for this provider
    pub fn is_a2ui_ready(&self) -> bool {
        self.supports_a2ui() && self.a2ui_enabled && self.is_ready()
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
            id: "ominix-image".to_string(),
            name: "OminiX Image".to_string(),
            url: "http://localhost:8080/v1".to_string(),
            provider_type: ProviderType::OminiXImage,
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
            name: "Ollama".to_string(),
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
        ProviderPreferences {
            id: "kimi".to_string(),
            name: "Kimi".to_string(),
            url: "https://api.moonshot.ai/v1".to_string(),
            provider_type: ProviderType::OpenAi,
            ..Default::default()
        },
        ProviderPreferences {
            id: "nvidia".to_string(),
            name: "NVIDIA".to_string(),
            url: "https://integrate.api.nvidia.com/v1".to_string(),
            provider_type: ProviderType::OpenAi,
            ..Default::default()
        },
        ProviderPreferences {
            id: "zhipu".to_string(),
            name: "Zhipu AI (GLM)".to_string(),
            url: "https://api.z.ai/api/paas/v4".to_string(),
            provider_type: ProviderType::OpenAi,
            ..Default::default()
        },
    ]
}
