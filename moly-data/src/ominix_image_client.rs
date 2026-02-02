//! OminiX Image Client - Configurable OpenAI-compatible image generation client
//!
//! This client is designed to work with OminiX-API's `/v1/images/generations` endpoint
//! with full support for configurable parameters like size, model, strength (for img2img), etc.

use moly_kit::aitk::protocol::*;
use moly_kit::aitk::utils::asynchronous::{BoxPlatformSendFuture, BoxPlatformSendStream};
use reqwest::header::{HeaderMap, HeaderName};
use serde::Serialize;
use std::{
    str::FromStr,
    sync::{Arc, RwLock},
};

/// Image generation configuration
#[derive(Debug, Clone, Serialize)]
pub struct ImageGenerationConfig {
    /// Image size (e.g., "512x512", "1024x1024")
    #[serde(default = "default_size")]
    pub size: String,
    /// Number of images to generate
    #[serde(default = "default_n")]
    pub n: usize,
    /// Response format: "b64_json" or "url"
    #[serde(default = "default_response_format")]
    pub response_format: String,
    /// Quality setting (optional, for some models)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,
    /// Strength for img2img (0.0-1.0, higher = more change)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f32>,
}

fn default_size() -> String {
    "512x512".to_string()
}

fn default_n() -> usize {
    1
}

fn default_response_format() -> String {
    "b64_json".to_string()
}

impl Default for ImageGenerationConfig {
    fn default() -> Self {
        Self {
            size: default_size(),
            n: default_n(),
            response_format: default_response_format(),
            quality: None,
            strength: None,
        }
    }
}

impl ImageGenerationConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_size(mut self, size: impl Into<String>) -> Self {
        self.size = size.into();
        self
    }

    pub fn with_n(mut self, n: usize) -> Self {
        self.n = n;
        self
    }

    pub fn with_quality(mut self, quality: impl Into<String>) -> Self {
        self.quality = Some(quality.into());
        self
    }

    pub fn with_strength(mut self, strength: f32) -> Self {
        self.strength = Some(strength);
        self
    }
}

/// Image data from API response
enum ImageData<'a> {
    Base64(&'a str),
    Url(&'a str),
}

#[derive(Debug, Clone)]
struct OminiXImageClientInner {
    url: String,
    client: reqwest::Client,
    headers: HeaderMap,
    config: ImageGenerationConfig,
    /// Reference image for img2img (base64 encoded)
    reference_image: Option<String>,
}

/// OminiX Image Generation Client
///
/// A configurable client for local image generation APIs that follow
/// the OpenAI `/v1/images/generations` format.
///
/// ## Features
///
/// - Configurable image size (512x512, 1024x1024, etc.)
/// - Support for multiple images per request
/// - Image-to-image (img2img) with reference image and strength
/// - Works with both FLUX and Z-Image models
///
/// ## Example
///
/// ```rust
/// let mut client = OminiXImageClient::new("http://localhost:8080/v1".to_string())
///     .with_config(ImageGenerationConfig::new()
///         .with_size("512x512")
///         .with_n(1));
///
/// // For img2img
/// client.set_reference_image(Some(base64_image_data));
/// client.set_strength(0.75);
/// ```
#[derive(Debug)]
pub struct OminiXImageClient(Arc<RwLock<OminiXImageClientInner>>);

impl Clone for OminiXImageClient {
    fn clone(&self) -> Self {
        OminiXImageClient(Arc::clone(&self.0))
    }
}

impl OminiXImageClient {
    /// Create a new client pointing to the given base URL
    pub fn new(url: String) -> Self {
        let headers = HeaderMap::new();
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(600)) // 10 minute timeout for image gen
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        let inner = OminiXImageClientInner {
            url,
            client,
            headers,
            config: ImageGenerationConfig::default(),
            reference_image: None,
        };

        OminiXImageClient(Arc::new(RwLock::new(inner)))
    }

    /// Create with custom configuration
    pub fn with_config(self, config: ImageGenerationConfig) -> Self {
        self.0.write().unwrap().config = config;
        self
    }

    /// Set a custom header
    pub fn set_header(&mut self, key: &str, value: &str) -> Result<(), &'static str> {
        let header_name = HeaderName::from_str(key).map_err(|_| "Invalid header name")?;
        let header_value = value.parse().map_err(|_| "Invalid header value")?;
        self.0.write().unwrap().headers.insert(header_name, header_value);
        Ok(())
    }

    /// Set API key (for compatibility, local servers often don't need this)
    pub fn set_key(&mut self, key: &str) -> Result<(), &'static str> {
        self.set_header("Authorization", &format!("Bearer {}", key))
    }

    /// Get the base URL
    pub fn get_url(&self) -> String {
        self.0.read().unwrap().url.clone()
    }

    /// Set strength for img2img
    pub fn set_strength(&mut self, strength: f32) {
        self.0.write().unwrap().config.strength = Some(strength);
    }

    /// Get the current config (clone)
    pub fn get_config(&self) -> ImageGenerationConfig {
        self.0.read().unwrap().config.clone()
    }

    /// Set config
    pub fn set_config(&mut self, config: ImageGenerationConfig) {
        self.0.write().unwrap().config = config;
    }

    /// Set reference image for img2img (base64 encoded PNG/JPEG)
    pub fn set_reference_image(&mut self, image_base64: Option<String>) {
        self.0.write().unwrap().reference_image = image_base64;
    }

    /// Set image size
    pub fn set_size(&mut self, size: impl Into<String>) {
        self.0.write().unwrap().config.size = size.into();
    }

    /// Generate image from prompt
    async fn generate_image(
        &self,
        bot_id: &BotId,
        messages: &[Message],
    ) -> Result<MessageContent, ClientError> {
        let inner = self.0.read().unwrap().clone();

        // Extract prompt from last message
        let prompt = messages
            .last()
            .map(|msg| msg.content.text.as_str())
            .ok_or_else(|| {
                ClientError::new(ClientErrorKind::Unknown, "No messages provided".to_string())
            })?;

        let url = format!("{}/images/generations", inner.url);

        // Build request JSON
        let mut request_json = serde_json::json!({
            "model": bot_id.id(),
            "prompt": prompt,
            "size": inner.config.size,
            "n": inner.config.n,
            "response_format": inner.config.response_format,
        });

        // Add optional fields
        if let Some(quality) = &inner.config.quality {
            request_json["quality"] = serde_json::json!(quality);
        }

        // Add img2img parameters if reference image is set
        if let Some(ref_image) = &inner.reference_image {
            request_json["image"] = serde_json::json!(ref_image);
            if let Some(strength) = inner.config.strength {
                request_json["strength"] = serde_json::json!(strength);
            }
        }

        log::debug!("Image generation request to {}: model={}, size={}",
            url, bot_id.id(), inner.config.size);

        let request = inner
            .client
            .post(&url)
            .headers(inner.headers.clone())
            .json(&request_json);

        let response = request.send().await.map_err(|e| {
            ClientError::new_with_source(
                ClientErrorKind::Network,
                format!(
                    "Could not send request to {url}. Verify your connection and the server status."
                ),
                Some(e),
            )
        })?;

        let status = response.status();
        let text = response.text().await.unwrap_or_default();

        if !status.is_success() {
            return Err(ClientError::new(
                ClientErrorKind::Response,
                format!(
                    "Request to {url} failed with status {} and content: {}",
                    status, text
                ),
            ));
        }

        let response_json: serde_json::Value = serde_json::from_str(&text).map_err(|e| {
            ClientError::new_with_source(
                ClientErrorKind::Format,
                format!(
                    "Failed to parse response from {url}. Response: {}",
                    &text[..text.len().min(200)]
                ),
                Some(e),
            )
        })?;

        // Parse all images from response
        let mut attachments = Vec::new();
        if let Some(data_array) = response_json["data"].as_array() {
            for (i, data) in data_array.iter().enumerate() {
                if let Some(image_data) = image_data_from_value(data) {
                    match attachment_from_image_data(image_data, &inner.client, i).await {
                        Ok(attachment) => attachments.push(attachment),
                        Err(e) => log::warn!("Failed to process image {}: {}", i, e),
                    }
                }
            }
        }

        if attachments.is_empty() {
            return Err(ClientError::new(
                ClientErrorKind::Format,
                format!("Response from {url} does not contain image data in a recognized format."),
            ));
        }

        // Include revised prompt if available
        let revised_prompt = response_json["data"][0]["revised_prompt"]
            .as_str()
            .map(|s| s.to_string());

        let content = MessageContent {
            text: revised_prompt.unwrap_or_default(),
            attachments,
            ..Default::default()
        };

        Ok(content)
    }
}

fn image_data_from_value(value: &serde_json::Value) -> Option<ImageData<'_>> {
    value["b64_json"]
        .as_str()
        .map(ImageData::Base64)
        .or_else(|| value["url"].as_str().map(ImageData::Url))
}

async fn attachment_from_image_data(
    image_data: ImageData<'_>,
    client: &reqwest::Client,
    index: usize,
) -> Result<Attachment, ClientError> {
    let name = format!("generated_image_{}.png", index);
    match image_data {
        ImageData::Base64(b64) => attachment_from_base64(b64, &name),
        ImageData::Url(url) => attachment_from_url(url, client, &name).await,
    }
}

fn attachment_from_base64(b64: &str, name: &str) -> Result<Attachment, ClientError> {
    Attachment::from_base64(name.to_string(), Some("image/png".to_string()), b64).map_err(|e| {
        ClientError::new_with_source(
            ClientErrorKind::Format,
            "Failed to create attachment from base64 data".to_string(),
            Some(e),
        )
    })
}

async fn attachment_from_url(
    url: &str,
    client: &reqwest::Client,
    name: &str,
) -> Result<Attachment, ClientError> {
    let bytes = client
        .get(url)
        .send()
        .await
        .map_err(|e| {
            ClientError::new_with_source(
                ClientErrorKind::Network,
                format!("Failed to fetch image from URL: {}", url),
                Some(e),
            )
        })?
        .bytes()
        .await
        .map_err(|e| {
            ClientError::new_with_source(
                ClientErrorKind::Network,
                format!("Failed to read image bytes from URL: {}", url),
                Some(e),
            )
        })?;

    Ok(Attachment::from_bytes(
        name.to_string(),
        Some("image/png".to_string()),
        &bytes,
    ))
}

impl BotClient for OminiXImageClient {
    fn bots(&mut self) -> BoxPlatformSendFuture<'static, ClientResult<Vec<Bot>>> {
        Box::pin(async move {
            // Return hardcoded image generation bots for OminiX
            let bots = vec![
                Bot {
                    id: BotId::new("zimage"),
                    name: "Z-Image Turbo".to_string(),
                    avatar: EntityAvatar::Text("Z".to_string()),
                    capabilities: BotCapabilities::new().with_capability(BotCapability::TextInput),
                },
                Bot {
                    id: BotId::new("flux"),
                    name: "FLUX.2-klein".to_string(),
                    avatar: EntityAvatar::Text("F".to_string()),
                    capabilities: BotCapabilities::new().with_capability(BotCapability::TextInput),
                },
            ];

            ClientResult::new_ok(bots)
        })
    }

    fn send(
        &mut self,
        bot_id: &BotId,
        messages: &[Message],
        _tools: &[Tool],
    ) -> BoxPlatformSendStream<'static, ClientResult<MessageContent>> {
        let self_clone = self.clone();
        let bot_id = bot_id.clone();
        let messages = messages.to_vec();

        Box::pin(async_stream::stream! {
            match self_clone.generate_image(&bot_id, &messages).await {
                Ok(content) => yield ClientResult::new_ok(content),
                Err(e) => yield ClientResult::new_err(e.into()),
            }
        })
    }

    fn clone_box(&self) -> Box<dyn BotClient> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = ImageGenerationConfig::new()
            .with_size("1024x1024")
            .with_n(2)
            .with_strength(0.75);

        assert_eq!(config.size, "1024x1024");
        assert_eq!(config.n, 2);
        assert_eq!(config.strength, Some(0.75));
    }

    #[test]
    fn test_client_creation() {
        let client = OminiXImageClient::new("http://localhost:8080/v1".to_string())
            .with_config(ImageGenerationConfig::new().with_size("512x512"));

        assert_eq!(client.get_url(), "http://localhost:8080/v1");
    }
}
