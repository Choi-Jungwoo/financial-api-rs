use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use time::macros::{format_description, offset};
use time::{Date, Time};

use crate::ValidationError;

/// Non-negative Unix timestamp in milliseconds.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, derive_more::Display,
)]
#[serde(transparent)]
pub struct UnixMillis(i64);

impl UnixMillis {
    pub const fn new(value: i64) -> Result<Self, ValidationError> {
        if value < 0 {
            return Err(ValidationError::new(
                "timestamp",
                "must be non-negative milliseconds since the Unix epoch",
            ));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}

impl<'de> Deserialize<'de> for UnixMillis {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = i64::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

/// Validated Gregorian natural day serialized as `YYYY-MM-DD`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::From)]
pub struct NaturalDate(Date);

impl NaturalDate {
    pub fn parse(value: &str) -> Result<Self, ValidationError> {
        if value.len() != 10 {
            return Err(ValidationError::new("date", "must use YYYY-MM-DD format"));
        }
        Date::parse(value, format_description!("[year]-[month]-[day]"))
            .map(Self)
            .map_err(|_| ValidationError::new("date", "must be a valid Gregorian date"))
    }

    pub(crate) fn checked_add_years(self, years: i32) -> Option<Self> {
        let target_year = self.0.year().checked_add(years)?;
        self.0
            .replace_year(target_year)
            .or_else(|_| self.0.replace_day(28)?.replace_year(target_year))
            .ok()
            .map(Self)
    }
}

impl fmt::Display for NaturalDate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self
            .0
            .format(format_description!("[year]-[month]-[day]"))
            .map_err(|_| fmt::Error)?;
        formatter.write_str(&value)
    }
}

impl FromStr for NaturalDate {
    type Err = ValidationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl<'de> Deserialize<'de> for NaturalDate {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).map_err(serde::de::Error::custom)
    }
}

impl Serialize for NaturalDate {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

/// Validated Gregorian natural day encoded on the wire as `YYYYMMDD`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompactDate(NaturalDate);

impl CompactDate {
    pub fn parse(value: &str) -> Result<Self, ValidationError> {
        if value.len() != 8 {
            return Err(ValidationError::new("date", "must use YYYYMMDD format"));
        }
        Date::parse(value, format_description!("[year][month][day]"))
            .map(|date| Self(NaturalDate(date)))
            .map_err(|_| ValidationError::new("date", "must be a valid Gregorian date"))
    }

    #[must_use]
    pub const fn natural_date(self) -> NaturalDate {
        self.0
    }
}

impl fmt::Display for CompactDate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self
            .0
            .0
            .format(format_description!("[year][month][day]"))
            .map_err(|_| fmt::Error)?;
        formatter.write_str(&value)
    }
}

impl FromStr for CompactDate {
    type Err = ValidationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl<'de> Deserialize<'de> for CompactDate {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).map_err(serde::de::Error::custom)
    }
}

impl Serialize for CompactDate {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

/// A natural date represented as its `Asia/Shanghai` midnight Unix milliseconds.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, derive_more::Display,
)]
#[serde(transparent)]
pub struct ShanghaiDateMillis(UnixMillis);

impl ShanghaiDateMillis {
    pub fn from_date(date: NaturalDate) -> Result<Self, ValidationError> {
        let milliseconds = date
            .0
            .with_time(Time::MIDNIGHT)
            .assume_offset(offset!(+8))
            .unix_timestamp_nanos()
            / 1_000_000;
        let milliseconds = i64::try_from(milliseconds)
            .map_err(|_| ValidationError::new("date", "is outside the timestamp range"))?;
        UnixMillis::new(milliseconds).map(Self)
    }

    #[must_use]
    pub const fn get(self) -> i64 {
        self.0.get()
    }
}

impl TryFrom<NaturalDate> for ShanghaiDateMillis {
    type Error = ValidationError;

    fn try_from(value: NaturalDate) -> Result<Self, Self::Error> {
        Self::from_date(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{NaturalDate, ShanghaiDateMillis};
    use time::macros::date;

    #[test]
    fn natural_date_validates_the_calendar() {
        assert_eq!(
            NaturalDate::parse("2024-02-29").unwrap().to_string(),
            "2024-02-29"
        );
        assert!(NaturalDate::parse("2023-02-29").is_err());
        assert!(NaturalDate::parse("2024-2-9").is_err());
    }

    #[test]
    fn natural_dates_use_standard_conversions() {
        let date_from_text: NaturalDate = "2024-02-29".parse().unwrap();
        let date_from_value = NaturalDate::from(date!(2024 - 02 - 29));

        assert_eq!(date_from_text, date_from_value);
        assert!("2023-02-29".parse::<NaturalDate>().is_err());
    }

    #[test]
    fn shanghai_date_millis_is_always_local_midnight() {
        let value =
            ShanghaiDateMillis::from_date(NaturalDate::parse("2024-05-20").unwrap()).unwrap();
        assert_eq!(value.get(), 1_716_134_400_000);
    }
}
