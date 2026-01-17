//! MolyClient - HTTP client for communicating with Moly Server
//!
//! Handles model discovery, search, and download management.

use moly_protocol::data::{Model, DownloadedFile, PendingDownload};
use reqwest::Client;
use serde::Serialize;
use std::sync::{Arc, Mutex};

/// Default port for Moly Server
const DEFAULT_SERVER_PORT: u16 = 8765;

/// Connection status for the Moly Server
#[derive(Clone, Debug, Default, PartialEq)]
pub enum ServerConnectionStatus {
    #[default]
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}

/// Inner state for MolyClient
struct MolyClientInner {
    base_url: String,
    connection_status: ServerConnectionStatus,
}

/// HTTP client for Moly Server communication
#[derive(Clone)]
pub struct MolyClient {
    client: Client,
    inner: Arc<Mutex<MolyClientInner>>,
}

impl Default for MolyClient {
    fn default() -> Self {
        Self::new()
    }
}

impl MolyClient {
    /// Create a new MolyClient with default settings
    pub fn new() -> Self {
        let port = std::env::var("MOLY_SERVER_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(DEFAULT_SERVER_PORT);

        Self::with_port(port)
    }

    /// Create a new MolyClient with a specific port
    pub fn with_port(port: u16) -> Self {
        let base_url = format!("http://localhost:{}", port);

        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            inner: Arc::new(Mutex::new(MolyClientInner {
                base_url,
                connection_status: ServerConnectionStatus::Disconnected,
            })),
        }
    }

    /// Get the current connection status
    pub fn connection_status(&self) -> ServerConnectionStatus {
        self.inner.lock().unwrap().connection_status.clone()
    }

    /// Set the connection status
    fn set_connection_status(&self, status: ServerConnectionStatus) {
        self.inner.lock().unwrap().connection_status = status;
    }

    /// Get the base URL
    fn base_url(&self) -> String {
        self.inner.lock().unwrap().base_url.clone()
    }

    /// Test connection to Moly Server
    pub async fn test_connection(&self) -> Result<(), String> {
        self.set_connection_status(ServerConnectionStatus::Connecting);

        let url = format!("{}/ping", self.base_url());

        match self.client.get(&url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    self.set_connection_status(ServerConnectionStatus::Connected);
                    log::info!("Connected to Moly Server at {}", self.base_url());
                    Ok(())
                } else {
                    let error = format!("Server returned status: {}", response.status());
                    self.set_connection_status(ServerConnectionStatus::Error(error.clone()));
                    Err(error)
                }
            }
            Err(e) => {
                let error = if e.is_connect() {
                    "Failed to connect to Moly Server. Is it running?".to_string()
                } else if e.is_timeout() {
                    "Connection timed out".to_string()
                } else {
                    format!("Connection error: {}", e)
                };
                self.set_connection_status(ServerConnectionStatus::Error(error.clone()));
                Err(error)
            }
        }
    }

    /// Get featured models from the server
    pub async fn get_featured_models(&self) -> Result<Vec<Model>, String> {
        let url = format!("{}/models/featured", self.base_url());

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Server returned status: {}", response.status()));
        }

        response
            .json::<Vec<Model>>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// Search models by query
    pub async fn search_models(&self, query: &str) -> Result<Vec<Model>, String> {
        let url = format!("{}/models/search?q={}", self.base_url(), urlencoding::encode(query));

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Server returned status: {}", response.status()));
        }

        response
            .json::<Vec<Model>>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// Get list of downloaded files
    pub async fn get_downloaded_files(&self) -> Result<Vec<DownloadedFile>, String> {
        let url = format!("{}/files", self.base_url());

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Server returned status: {}", response.status()));
        }

        response
            .json::<Vec<DownloadedFile>>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// Get current pending downloads
    pub async fn get_pending_downloads(&self) -> Result<Vec<PendingDownload>, String> {
        let url = format!("{}/downloads", self.base_url());

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Server returned status: {}", response.status()));
        }

        response
            .json::<Vec<PendingDownload>>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    }

    /// Start downloading a file
    pub async fn download_file(&self, file_id: &str) -> Result<(), String> {
        let url = format!("{}/downloads", self.base_url());

        #[derive(Serialize)]
        struct DownloadRequest<'a> {
            file_id: &'a str,
        }

        let response = self.client
            .post(&url)
            .json(&DownloadRequest { file_id })
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Failed to start download: {}", error_text));
        }

        Ok(())
    }

    /// Pause a download
    pub async fn pause_download(&self, file_id: &str) -> Result<(), String> {
        let url = format!("{}/downloads/{}", self.base_url(), file_id);

        let response = self.client
            .post(&url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Failed to pause download: {}", response.status()));
        }

        Ok(())
    }

    /// Cancel a download
    pub async fn cancel_download(&self, file_id: &str) -> Result<(), String> {
        let url = format!("{}/downloads/{}", self.base_url(), file_id);

        let response = self.client
            .delete(&url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Failed to cancel download: {}", response.status()));
        }

        Ok(())
    }

    /// Delete a downloaded file
    pub async fn delete_file(&self, file_id: &str) -> Result<(), String> {
        let url = format!("{}/files/{}", self.base_url(), file_id);

        let response = self.client
            .delete(&url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Failed to delete file: {}", response.status()));
        }

        Ok(())
    }
}

// URL encoding helper
mod urlencoding {
    pub fn encode(input: &str) -> String {
        let mut result = String::new();
        for c in input.chars() {
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~' => {
                    result.push(c);
                }
                ' ' => result.push_str("%20"),
                _ => {
                    for byte in c.to_string().as_bytes() {
                        result.push_str(&format!("%{:02X}", byte));
                    }
                }
            }
        }
        result
    }
}
