use std::env;
use std::fmt;
use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Serialize;
use serde::de::DeserializeOwned;
use time::OffsetDateTime;
use time::macros::offset;

use crate::NaturalDate;
use crate::error::{BusinessError, Error, ValidationError};

const DEFAULT_BASE_URL: &str = "https://fuyao.aicubes.cn/";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
const API_KEY_HEADER: HeaderName = HeaderName::from_static("x-api-key");

/// Validated API key stored as a sensitive HTTP header value.
#[derive(Clone)]
pub struct ApiKey(HeaderValue);

impl ApiKey {
    /// Validate an API key without exposing it in errors or debug output.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ValidationError> {
        let value = value.as_ref();
        if value.is_empty() {
            return Err(ValidationError::new("api_key", "must not be empty"));
        }
        let mut header = HeaderValue::from_str(value)
            .map_err(|_| ValidationError::new("api_key", "must be a valid HTTP header value"))?;
        header.set_sensitive(true);
        Ok(Self(header))
    }
}

impl fmt::Debug for ApiKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ApiKey([REDACTED])")
    }
}

/// Successful response after the common business envelope has been checked.
#[derive(Debug, Clone, PartialEq)]
pub struct Response<T> {
    request_id: String,
    data: T,
}

impl<T> Response<T> {
    /// Upstream trace identifier.
    #[must_use]
    pub fn request_id(&self) -> &str {
        &self.request_id
    }

    /// Endpoint-specific response data.
    #[must_use]
    pub const fn data(&self) -> &T {
        &self.data
    }

    /// Consume the response and return its endpoint data.
    #[must_use]
    pub fn into_data(self) -> T {
        self.data
    }
}

impl Response<serde_json::Value> {
    /// Decode lossless JSON data into an application-owned response type.
    pub fn into_typed<T: DeserializeOwned>(self) -> Result<Response<T>, Error> {
        let data = serde_json::from_value(self.data).map_err(|source| Error::InvalidResponse {
            source: Some(source),
        })?;
        Ok(Response {
            request_id: self.request_id,
            data,
        })
    }
}

/// Builder for optional client-wide transport configuration.
#[derive(Debug)]
pub struct ClientBuilder {
    api_key: ApiKey,
    base_url: String,
    timeout: Duration,
    reference_date: Option<NaturalDate>,
}

impl ClientBuilder {
    /// Override the API root, primarily for private deployments and tests.
    #[must_use]
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Set the complete request timeout.
    #[must_use]
    pub const fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Override the Shanghai natural date used by relative-date validation.
    ///
    /// This is useful for deterministic tests and historical request replay.
    #[must_use]
    pub const fn reference_date(mut self, date: NaturalDate) -> Self {
        self.reference_date = Some(date);
        self
    }

    /// Validate configuration and construct a reusable client.
    pub fn build(self) -> Result<Client, Error> {
        let mut base_url = reqwest::Url::parse(&self.base_url).map_err(|_| {
            ValidationError::new("base_url", "must be a valid absolute hierarchical URL")
        })?;
        if base_url.cannot_be_a_base()
            || base_url.query().is_some()
            || base_url.fragment().is_some()
        {
            return Err(ValidationError::new(
                "base_url",
                "must be an absolute hierarchical URL without query or fragment",
            )
            .into());
        }
        if !matches!(base_url.scheme(), "http" | "https") {
            return Err(ValidationError::new("base_url", "scheme must be http or https").into());
        }
        if !base_url.username().is_empty() || base_url.password().is_some() {
            return Err(
                ValidationError::new("base_url", "must not contain userinfo credentials").into(),
            );
        }
        if !base_url.path().ends_with('/') {
            let path = format!("{}/", base_url.path());
            base_url.set_path(&path);
        }

        let mut headers = HeaderMap::new();
        headers.insert(API_KEY_HEADER, self.api_key.0);
        let http = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(self.timeout)
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|source| Error::InvalidConfiguration { source })?;

        Ok(Client {
            http,
            base_url,
            reference_date: self.reference_date,
        })
    }
}

/// Reusable async API client.
#[derive(Debug, Clone)]
pub struct Client {
    http: reqwest::Client,
    base_url: reqwest::Url,
    reference_date: Option<NaturalDate>,
}

impl Client {
    /// Start client configuration with an explicit API key.
    #[must_use]
    pub fn builder(api_key: ApiKey) -> ClientBuilder {
        ClientBuilder {
            api_key,
            base_url: DEFAULT_BASE_URL.to_owned(),
            timeout: DEFAULT_TIMEOUT,
            reference_date: None,
        }
    }

    /// Construct a client using `HITHINK_FINANCE_API_KEY`.
    pub fn from_env() -> Result<Self, Error> {
        let value = env::var("HITHINK_FINANCE_API_KEY").map_err(|_| Error::MissingApiKey)?;
        Self::builder(ApiKey::new(value)?).build()
    }

    pub(crate) fn shanghai_today(&self) -> NaturalDate {
        self.reference_date.unwrap_or_else(|| {
            OffsetDateTime::now_utc()
                .to_offset(offset!(+8))
                .date()
                .into()
        })
    }

    pub(crate) async fn get<Q, T>(&self, path: &str, query: &Q) -> Result<Response<T>, Error>
    where
        Q: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let url = self
            .base_url
            .join(path.trim_start_matches('/'))
            .map_err(|_| {
                ValidationError::new("endpoint_path", "could not be joined to the API base URL")
            })?;
        let response = self
            .http
            .get(url)
            .query(query)
            .send()
            .await
            .map_err(|source| Error::Transport { source })?;
        let status = response.status();
        if !status.is_success() {
            return Err(Error::HttpStatus {
                status: status.as_u16(),
            });
        }
        let bytes = response
            .bytes()
            .await
            .map_err(|source| Error::Transport { source })?;
        let envelope: Envelope =
            serde_json::from_slice(&bytes).map_err(|source| Error::InvalidResponse {
                source: Some(source),
            })?;

        if envelope.code != 0 {
            return Err(
                BusinessError::new(envelope.code, envelope.message, envelope.request_id).into(),
            );
        }

        let data =
            serde_json::from_value(envelope.data).map_err(|source| Error::InvalidResponse {
                source: Some(source),
            })?;
        Ok(Response {
            request_id: envelope.request_id,
            data,
        })
    }
}

#[derive(serde::Deserialize)]
struct Envelope {
    code: i64,
    message: String,
    request_id: String,
    data: serde_json::Value,
}
