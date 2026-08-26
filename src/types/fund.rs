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
                let value = value.trim();
                if value.is_empty() {
                    return Err(ValidationError::new($field, "must not be empty"));
                }
                Ok(Self(value.to_owned()))
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

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let value = String::deserialize(deserializer)?;
                Self::new(value).map_err(serde::de::Error::custom)
            }
        }
    };
}

identifier_type!(
    /// Opaque fund-manager identifier discovered from fund profile data.
    ManagerId,
    "manager_id"
);
identifier_type!(
    /// Opaque fund-company identifier discovered from fund profile data.
    CompanyId,
    "company_id"
);
identifier_type!(
    /// Fund disclosure report type discovered from a report-dates endpoint.
    ReportType,
    "report_type"
);
identifier_type!(
    /// Opaque fund category code observed in diagnostics data.
    FundCategoryCode,
    "fund_type"
);

/// Opaque pagination cursor returned by a cursor-paged endpoint.
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
}

impl FromStr for Cursor {
    type Err = ValidationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl<'de> Deserialize<'de> for Cursor {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

wire_enum! {
    /// Fund target universe.
    pub enum FundType {
        Otc => "otc",
        Exchange => "exchange",
        Reits => "reits",
    }
}

wire_enum! {
    /// Holder record merge policy.
    pub enum HolderMergeScope {
        All => "all",
        Merged => "merged",
        Separate => "separate",
    }
}

wire_enum! {
    /// Fund NAV history range.
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
    /// Fund NAV series selection.
    pub enum FundNavType {
        Unit => "unit",
        Adjusted => "adj",
        UnitAndAdjusted => "unit,adj",
    }
}

wire_enum! {
    /// Fund manager performance range.
    pub enum ManagerPerformanceRange {
        Month => "month",
        ThreeMonths => "tmonth",
        Year => "year",
        CurrentYear => "nowyear",
        SinceInception => "now",
    }
}

wire_enum! {
    /// New fund offering subscription state.
    pub enum OfferingStatus {
        Active => "active",
        Upcoming => "upcoming",
    }
}

wire_enum! {
    /// Actual disclosure scope returned for a fund-holder record.
    pub enum HolderRecordScope {
        Merged => "merged",
        Separate => "separate",
    }
}

wire_enum! {
    /// Asset class used in a disclosed fund portfolio.
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
