/// 请求领域值或客户端配置值无效。
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

    /// 无效字段。
    #[must_use]
    pub const fn field(&self) -> &'static str {
        self.field
    }

    /// 对违反约束的稳定描述。
    #[must_use]
    pub const fn problem(&self) -> &'static str {
        self.problem
    }
}

/// 上游非零业务结果对应的稳定恢复类别。
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

/// 从响应信封中解码出的非零业务结果。
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

    /// 服务返回的数字业务结果码。
    #[must_use]
    pub const fn code(&self) -> i64 {
        self.code
    }

    /// 上游结果描述。
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// 上游请求追踪标识。
    #[must_use]
    pub fn request_id(&self) -> &str {
        &self.request_id
    }

    /// 面向恢复动作的 [`Self::code`] 分类。
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

/// 配置或调用 API 时产生的失败。
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
        source: serde_json::Error,
    },

    #[error(transparent)]
    Business(#[from] BusinessError),

    #[error("environment variable HITHINK_FINANCE_API_KEY is not configured")]
    MissingApiKey,
}
