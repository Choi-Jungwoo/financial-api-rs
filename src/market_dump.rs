use std::fmt;

use serde::{Deserialize, Deserializer};
use time::OffsetDateTime;

use crate::endpoints;
use crate::{Client, Error, Response, ValidationError};

/// 调试表示始终脱敏的短期预签名 URL。
#[derive(Clone, PartialEq, Eq)]
pub struct SecretUrl(String);

impl SecretUrl {
    /// 校验绝对 HTTP(S) 下载 URL。
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        let value = value.into();
        let url = reqwest::Url::parse(&value)
            .map_err(|_| ValidationError::new("presigned_url", "must be a valid absolute URL"))?;
        if !matches!(url.scheme(), "http" | "https")
            || url.host().is_none()
            || !url.username().is_empty()
            || url.password().is_some()
        {
            return Err(ValidationError::new(
                "presigned_url",
                "must be an absolute HTTP(S) URL without userinfo",
            ));
        }
        Ok(Self(value))
    }

    /// 借用 URL 以立即下载。
    #[must_use]
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for SecretUrl {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

impl fmt::Debug for SecretUrl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SecretUrl([REDACTED])")
    }
}

/// 短期有效的市场数据包下载授权。
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct MarketDumpUrl {
    pub presigned_url: SecretUrl,
    #[serde(with = "time::serde::rfc3339")]
    pub presigned_url_expires_at: OffsetDateTime,
}

impl Client {
    /// 获取完整十年日 K 线数据包的预签名下载地址。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../examples/market_dump_daily_k.rs"),
        "\n```"
    )]
    pub async fn market_dump_daily_k(&self) -> Result<Response<MarketDumpUrl>, Error> {
        self.market_dump_url(endpoints::DUMP_DAILY_K).await
    }

    /// 获取最近十个交易日日 K 线数据包的预签名下载地址。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../examples/market_dump_daily_k_10d.rs"),
        "\n```"
    )]
    pub async fn market_dump_daily_k_10d(&self) -> Result<Response<MarketDumpUrl>, Error> {
        self.market_dump_url(endpoints::DUMP_DAILY_K_10D).await
    }

    /// 获取完整复权事件数据包的预签名下载地址。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../examples/market_dump_adjustment_factors.rs"),
        "\n```"
    )]
    pub async fn market_dump_adjustment_factors(&self) -> Result<Response<MarketDumpUrl>, Error> {
        self.market_dump_url(endpoints::DUMP_ADJUSTMENT_FACTORS)
            .await
    }

    async fn market_dump_url(&self, path: &str) -> Result<Response<MarketDumpUrl>, Error> {
        self.get(path, &[] as &[(&str, &str)]).await
    }
}
