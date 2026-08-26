use std::fmt;

use reqwest::header::HeaderValue;

use super::ApiKey;
use crate::ValidationError;

impl ApiKey {
    /// 校验 API Key，且不在错误或调试输出中暴露其内容。
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

#[cfg(test)]
mod tests {
    use super::ApiKey;

    #[test]
    fn debug_output_redacts_the_credential() {
        let key = ApiKey::new("super-secret-test-key").unwrap();

        assert!(!format!("{key:?}").contains("super-secret-test-key"));
    }
}
