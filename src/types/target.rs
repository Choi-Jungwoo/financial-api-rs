use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};

use super::wire::wire_enum;
use crate::ValidationError;

/// 包含市场后缀的完整标的代码。
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, derive_more::Display)]
#[serde(transparent)]
pub struct Thscode(String);

impl Thscode {
    pub fn new(value: impl AsRef<str>) -> Result<Self, ValidationError> {
        let value = value.as_ref().trim().to_ascii_uppercase();
        if value.contains(',') {
            return Err(ValidationError::new(
                "thscode",
                "must contain exactly one target",
            ));
        }
        let Some((ticker, suffix)) = value.split_once('.') else {
            return Err(ValidationError::new(
                "thscode",
                "must include a market suffix",
            ));
        };
        if ticker.is_empty()
            || suffix.is_empty()
            || suffix.contains('.')
            || !ticker.bytes().all(|byte| byte.is_ascii_alphanumeric())
            || !suffix.bytes().all(|byte| byte.is_ascii_alphanumeric())
        {
            return Err(ValidationError::new(
                "thscode",
                "must use TICKER.SUFFIX with ASCII letters or digits",
            ));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn into_string(self) -> String {
        self.0
    }

    pub(crate) fn ticker_and_suffix(&self) -> (&str, &str) {
        self.0
            .split_once('.')
            .expect("validated thscode always contains a market suffix")
    }
}

impl FromStr for Thscode {
    type Err = ValidationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::try_from(value)
    }
}

impl TryFrom<&str> for Thscode {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for Thscode {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&String> for Thscode {
    type Error = ValidationError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<&Thscode> for Thscode {
    fn from(value: &Thscode) -> Self {
        value.clone()
    }
}

impl AsRef<str> for Thscode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'de> Deserialize<'de> for Thscode {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

/// 限定在 A 股标的宇宙内的完整标的代码。
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, derive_more::Display)]
#[serde(transparent)]
pub struct AShareCode(Thscode);

impl AShareCode {
    pub fn new(value: impl AsRef<str>) -> Result<Self, ValidationError> {
        let code = Thscode::new(value)?;
        Self::try_from(code)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub(crate) fn into_string(self) -> String {
        self.0.into_string()
    }
}

impl TryFrom<Thscode> for AShareCode {
    type Error = ValidationError;

    fn try_from(code: Thscode) -> Result<Self, Self::Error> {
        let (ticker, suffix) = code.ticker_and_suffix();
        if ticker.len() != 6
            || !ticker.bytes().all(|byte| byte.is_ascii_digit())
            || !matches!(suffix, "SH" | "SZ" | "BJ")
        {
            return Err(ValidationError::new(
                "thscode",
                "must be a six-digit A-share code ending in SH, SZ, or BJ",
            ));
        }
        Ok(Self(code))
    }
}

impl FromStr for AShareCode {
    type Err = ValidationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::try_from(value)
    }
}

impl TryFrom<&str> for AShareCode {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for AShareCode {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&String> for AShareCode {
    type Error = ValidationError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<&AShareCode> for AShareCode {
    fn from(value: &AShareCode) -> Self {
        value.clone()
    }
}

impl TryFrom<&Thscode> for AShareCode {
    type Error = ValidationError;

    fn try_from(value: &Thscode) -> Result<Self, Self::Error> {
        Self::try_from(value.clone())
    }
}

impl AsRef<str> for AShareCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'de> Deserialize<'de> for AShareCode {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

impl From<AShareCode> for Thscode {
    fn from(value: AShareCode) -> Self {
        value.0
    }
}

impl From<&AShareCode> for Thscode {
    fn from(value: &AShareCode) -> Self {
        value.0.clone()
    }
}

wire_enum! {
    /// 元数据 API 支持的交易所筛选条件。
    pub enum Exchange {
        Shanghai => "SH",
        Shenzhen => "SZ",
        Beijing => "BJ",
    }
}

wire_enum! {
    /// 规范化的最细粒度资产类型。
    pub enum AssetType {
        AShare => "a-share",
        AShareIndex => "a-share-index",
        FundOtc => "fund-otc",
        FundEtf => "fund-etf",
        FundLof => "fund-lof",
    }
}

#[cfg(test)]
mod tests {
    use super::{AShareCode, Thscode};

    #[test]
    fn complete_target_code_is_normalized_and_rejects_tickers() {
        let code = Thscode::new(" 600519.sh ").unwrap();
        assert_eq!(code.as_str(), "600519.SH");
        let error = Thscode::new("600519").unwrap_err();
        assert_eq!(error.field(), "thscode");
        assert_eq!(error.problem(), "must include a market suffix");
    }

    #[test]
    fn a_share_code_rejects_non_a_share_suffixes() {
        assert!(AShareCode::new("600519.SH").is_ok());
        assert!(AShareCode::new("510300.OF").is_err());
        assert!(AShareCode::new("886042.TI").is_err());
    }

    #[test]
    fn target_values_use_standard_fallible_conversions() {
        let text = "600519.sh".to_owned();
        let a_share = AShareCode::try_from(&text).unwrap();
        let target = Thscode::try_from(text).unwrap();

        assert_eq!(target, Thscode::from(a_share));
        assert!(AShareCode::try_from("510300.OF").is_err());
    }
}
