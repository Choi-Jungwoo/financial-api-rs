use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};

use super::wire::wire_enum;
use crate::ValidationError;

/// Non-empty search text used for cross-market target lookup.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, derive_more::Display)]
#[serde(transparent)]
pub struct SearchQuery(String);

impl SearchQuery {
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(ValidationError::new("q", "must not be empty"));
        }
        Ok(Self(value))
    }
}

/// Complete target code including its market suffix.
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
}

impl FromStr for Thscode {
    type Err = ValidationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl<'de> Deserialize<'de> for Thscode {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

/// Complete target code restricted to the A-share target universe.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, derive_more::Display)]
#[serde(transparent)]
pub struct AShareCode(Thscode);

impl AShareCode {
    pub fn new(value: impl AsRef<str>) -> Result<Self, ValidationError> {
        let code = Thscode::new(value)?;
        let (ticker, suffix) = code
            .as_str()
            .split_once('.')
            .expect("validated thscode always contains a dot");
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

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl FromStr for AShareCode {
    type Err = ValidationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
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

wire_enum! {
    /// Exchange filter supported by the metadata API.
    pub enum Exchange {
        Shanghai => "SH",
        Shenzhen => "SZ",
        Beijing => "BJ",
    }
}

wire_enum! {
    /// Normalized leaf asset type.
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
        let target: Thscode = "600519.sh".parse().unwrap();
        let a_share: AShareCode = "600519.sh".parse().unwrap();

        assert_eq!(target, a_share.clone().into());
        assert!("510300.OF".parse::<AShareCode>().is_err());
    }
}
