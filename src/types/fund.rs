use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};

use super::wire::wire_enum;
use crate::ValidationError;

macro_rules! identifier_type {
    ($(#[$meta:meta])* $name:ident, $field:literal) => {
        $(#[$meta])*
        #[derive(
            Debug,
            Clone,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            Serialize,
            derive_more::Display,
        )]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
                let value = value.into();
                let trimmed = value.trim();
                if trimmed.is_empty() {
                    return Err(ValidationError::new($field, "must not be empty"));
                }
                if trimmed.len() == value.len() {
                    Ok(Self(value))
                } else {
                    Ok(Self(trimmed.to_owned()))
                }
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl FromStr for $name {
            type Err = ValidationError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let value = String::deserialize(deserializer)?;
                Self::new(value).map_err(serde::de::Error::custom)
            }
        }
    };
}

identifier_type!(
    /// 从基金资料数据中发现的不透明基金经理标识。
    ManagerId,
    "manager_id"
);
identifier_type!(
    /// 从基金资料数据中发现的不透明基金公司标识。
    CompanyId,
    "company_id"
);
identifier_type!(
    /// 从报告期端点中发现的基金披露报告类型。
    ReportType,
    "report_type"
);
identifier_type!(
    /// 在诊断数据中观察到的不透明基金分类代码。
    FundCategoryCode,
    "fund_type"
);

impl ReportType {
    pub(crate) fn into_string(self) -> String {
        self.0
    }
}

/// 游标分页端点返回的不透明分页游标。
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, derive_more::Display)]
#[serde(transparent)]
pub struct Cursor(String);

impl Cursor {
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(ValidationError::new("offset", "must not be empty"));
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
}

impl FromStr for Cursor {
    type Err = ValidationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl AsRef<str> for Cursor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'de> Deserialize<'de> for Cursor {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

wire_enum! {
    /// 基金标的宇宙。
    pub enum FundType {
        Otc => "otc",
        Exchange => "exchange",
        Reits => "reits",
    }
}

wire_enum! {
    /// 持有人记录合并策略。
    pub enum HolderMergeScope {
        All => "all",
        Merged => "merged",
        Separate => "separate",
    }
}

wire_enum! {
    /// 基金历史净值区间。
    pub enum FundRange {
        Week => "week",
        Month => "month",
        ThreeMonths => "tmonth",
        HalfYear => "hyear",
        Year => "year",
        TwoYears => "twoyear",
        ThreeYears => "tyear",
        FiveYears => "fyear",
    }
}

wire_enum! {
    /// 基金净值序列选择。
    pub enum FundNavType {
        Unit => "unit",
        Adjusted => "adj",
        UnitAndAdjusted => "unit,adj",
    }
}

wire_enum! {
    /// 基金经理业绩区间。
    pub enum ManagerPerformanceRange {
        Month => "month",
        ThreeMonths => "tmonth",
        Year => "year",
        CurrentYear => "nowyear",
        SinceInception => "now",
    }
}

wire_enum! {
    /// 新发基金认购状态。
    pub enum OfferingStatus {
        Active => "active",
        Upcoming => "upcoming",
    }
}

wire_enum! {
    /// 基金持有人记录返回的实际披露范围。
    pub enum HolderRecordScope {
        Merged => "merged",
        Separate => "separate",
    }
}

wire_enum! {
    /// 基金持仓披露使用的资产类别。
    pub enum PortfolioAssetType {
        Stock => "stock",
        Bond => "bond",
        Fund => "fund",
    }
}

#[cfg(test)]
mod tests {
    use super::{CompanyId, Cursor, FundCategoryCode, ManagerId, ReportType};

    #[test]
    fn endpoint_identifiers_have_distinct_validated_types() {
        assert_eq!(ManagerId::new("manager-1").unwrap().as_str(), "manager-1");
        assert_eq!(CompanyId::new("company-1").unwrap().as_str(), "company-1");
        assert_eq!(
            Cursor::new("opaque+/cursor==").unwrap().as_str(),
            "opaque+/cursor=="
        );
        assert_eq!(ReportType::new("quarter").unwrap().as_str(), "quarter");
        assert_eq!(
            FundCategoryCode::new("282001003").unwrap().as_str(),
            "282001003"
        );
        assert_eq!(ManagerId::new(" manager-1 ").unwrap().as_str(), "manager-1");
        assert_eq!(
            CompanyId::new("\tcompany-1\n").unwrap().as_str(),
            "company-1"
        );
        assert_eq!(ReportType::new(" quarter ").unwrap().as_str(), "quarter");
        assert_eq!(
            Cursor::new(" opaque+/cursor== ").unwrap().as_str(),
            " opaque+/cursor== "
        );

        assert!(ManagerId::new(" ").is_err());
        assert!(CompanyId::new("").is_err());
        assert!(Cursor::new("\t").is_err());
        assert!(ReportType::new("\n").is_err());
        assert!(FundCategoryCode::new(" ").is_err());
    }
}
