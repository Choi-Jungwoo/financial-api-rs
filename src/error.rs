/// A request-domain or client configuration value was invalid.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("invalid {field}: {problem}")]
pub struct ValidationError {
    field: &'static str,
    problem: &'static str,
}

impl ValidationError {
    pub(crate) const fn new(field: &'static str, problem: &'static str) -> Self {
        Self { field, problem }
    }

    /// The invalid field.
    #[must_use]
    pub const fn field(&self) -> &'static str {
        self.field
    }

    /// A stable description of the violated constraint.
    #[must_use]
    pub const fn problem(&self) -> &'static str {
        self.problem
    }
}

/// Stable recovery category for an upstream non-zero business result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum BusinessErrorKind {
    InvalidRequest,
    Authentication,
    PermissionDenied,
    TargetNotFound,
    DataNotReady,
    UnsupportedTarget,
    RateLimited,
    UpstreamUnavailable,
    Other,
}

/// A decoded non-zero business result from the response envelope.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("upstream business request failed with code {code}")]
pub struct BusinessError {
    code: i64,
    message: String,
    request_id: String,
}

impl BusinessError {
    pub(crate) fn new(code: i64, message: String, request_id: String) -> Self {
        Self {
            code,
            message,
            request_id,
        }
    }

    /// Numeric business result code returned by the service.
    #[must_use]
    pub const fn code(&self) -> i64 {
        self.code
    }

    /// Upstream result description.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Upstream request trace identifier.
    #[must_use]
    pub fn request_id(&self) -> &str {
        &self.request_id
    }

    /// Recovery-oriented classification of [`Self::code`].
    #[must_use]
    pub const fn kind(&self) -> BusinessErrorKind {
        match self.code {
            1001..=1004 => BusinessErrorKind::InvalidRequest,
            2001 | 2002 | 2004 => BusinessErrorKind::Authentication,
            2003 => BusinessErrorKind::PermissionDenied,
            3001 => BusinessErrorKind::TargetNotFound,
            3002 | 4040 => BusinessErrorKind::DataNotReady,
            3004 => BusinessErrorKind::UnsupportedTarget,
            4001 => BusinessErrorKind::RateLimited,
            5001..=5003 => BusinessErrorKind::UpstreamUnavailable,
            _ => BusinessErrorKind::Other,
        }
    }
}

/// Failures produced while configuring or calling the API.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error(transparent)]
    InvalidInput(#[from] ValidationError),

    #[error("invalid client configuration")]
    InvalidConfiguration {
        #[source]
        source: reqwest::Error,
    },

    #[error("HTTP transport failed")]
    Transport {
        #[source]
        source: reqwest::Error,
    },

    #[error("unexpected HTTP status {status}")]
    HttpStatus { status: u16 },

    #[error("response did not match the API envelope")]
    InvalidResponse {
        #[source]
        source: Option<serde_json::Error>,
    },

    #[error(transparent)]
    Business(#[from] BusinessError),

    #[error("environment variable HITHINK_FINANCE_API_KEY is not configured")]
    MissingApiKey,
}
