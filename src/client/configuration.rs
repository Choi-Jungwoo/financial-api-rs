use std::env;
use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderName};

use super::{ApiKey, Client, ClientBuilder};
use crate::{Error, NaturalDate, ValidationError};

const DEFAULT_BASE_URL: &str = "https://fuyao.aicubes.cn/";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
const API_KEY_HEADER: HeaderName = HeaderName::from_static("x-api-key");

impl ClientBuilder {
    /// 覆盖 API 根地址，主要用于私有部署和测试。
    #[must_use]
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// 设置完整请求的超时时间。
    #[must_use]
    pub const fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// 覆盖相对日期校验使用的上海自然日。
    ///
    /// 适用于确定性测试和历史请求重放。
    #[must_use]
    pub const fn reference_date(mut self, date: NaturalDate) -> Self {
        self.reference_date = Some(date);
        self
    }

    /// 校验配置并构造可复用客户端。
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

impl Client {
    /// 使用显式 API Key 开始配置客户端。
    #[must_use]
    pub fn builder(api_key: ApiKey) -> ClientBuilder {
        ClientBuilder {
            api_key,
            base_url: DEFAULT_BASE_URL.to_owned(),
            timeout: DEFAULT_TIMEOUT,
            reference_date: None,
        }
    }

    /// 使用环境变量 `HITHINK_FINANCE_API_KEY` 构造客户端。
    pub fn from_env() -> Result<Self, Error> {
        let value = env::var("HITHINK_FINANCE_API_KEY").map_err(|_| Error::MissingApiKey)?;
        Self::builder(ApiKey::new(value)?).build()
    }
}

#[cfg(test)]
mod tests {
    use super::{ApiKey, Client};
    use crate::Error;

    #[test]
    fn base_url_rejects_userinfo_credentials() {
        let error = Client::builder(ApiKey::new("test-api-key").unwrap())
            .base_url("https://user:secret@fuyao.aicubes.cn/")
            .build()
            .unwrap_err();

        let Error::InvalidInput(error) = error else {
            panic!("expected invalid base URL");
        };
        assert_eq!(error.field(), "base_url");
    }
}
