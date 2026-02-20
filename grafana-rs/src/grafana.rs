use reqwest::Client;
use serde::Deserialize;
use std::fmt;

// ---------------------------------------------------------------------------
// Error
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum GrafanaError {
    /// HTTP transport failed (connection refused, timeout, etc.)
    Http(reqwest::Error),
    /// Grafana returned a non-success status code
    Api { status: u16, body: String },
}

impl fmt::Display for GrafanaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GrafanaError::Http(e) => write!(f, "HTTP error: {e}"),
            GrafanaError::Api { status, body } => {
                write!(f, "Grafana API error ({status}): {body}")
            }
        }
    }
}

impl std::error::Error for GrafanaError {}

impl From<reqwest::Error> for GrafanaError {
    fn from(e: reqwest::Error) -> Self {
        GrafanaError::Http(e)
    }
}

// ---------------------------------------------------------------------------
// Models
// ---------------------------------------------------------------------------

/// Response from `GET /api/health`
#[derive(Debug, Deserialize)]
pub struct HealthResponse {
    pub commit: String,
    pub database: String,
    pub version: String,
}

/// Response from `GET /api/org`
#[derive(Debug, Deserialize)]
pub struct CurrentOrg {
    pub id: u64,
    pub name: String,
    pub address: OrgAddress,
}

#[derive(Debug, Deserialize)]
pub struct OrgAddress {
    pub address1: String,
    pub address2: String,
    pub city: String,
    #[serde(rename = "zipCode")]
    pub zip_code: String,
    pub state: String,
    pub country: String,
}

// ---------------------------------------------------------------------------
// Client
// ---------------------------------------------------------------------------

pub struct GrafanaClient {
    http: Client,
    base_url: String,
    api_key: String,
}

impl GrafanaClient {
    /// Connect to Grafana using an API key.
    pub fn connect(base_url: &str, api_key: &str) -> Self {
        let base_url = base_url.trim_end_matches('/').to_string();

        Self {
            http: Client::new(),
            base_url,
            api_key: api_key.to_string(),
        }
    }

    /// Check if Grafana is alive.
    /// GET /api/health
    pub async fn health(&self) -> Result<HealthResponse, GrafanaError> {
        self.get("/api/health").await
    }

    /// Get the current organization (proves API key auth works).
    /// GET /api/org
    pub async fn get_current_org(&self) -> Result<CurrentOrg, GrafanaError> {
        self.get("/api/org").await
    }

    // -----------------------------------------------------------------------
    // Internal
    // -----------------------------------------------------------------------

    /// Authenticated GET request → deserialize JSON response.
    async fn get<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T, GrafanaError> {
        let url = format!("{}{}", self.base_url, path);

        let resp = self
            .http
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(GrafanaError::Api {
                status: status.as_u16(),
                body,
            });
        }

        let body = resp.json::<T>().await?;
        Ok(body)
    }
}