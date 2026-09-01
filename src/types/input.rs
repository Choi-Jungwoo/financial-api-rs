//! 可省略端点参数的转换边界。

use super::{NaturalDate, ReportType};
use crate::ValidationError;

/// 可省略端点参数的转换目标。
///
/// 端点通过 `TryInto<OptionalInput<T>>` 同时接受外部表示、领域值和裸 `None`，
/// 转换完成后只保留 `Option<T>`。
pub struct OptionalInput<T>(Option<T>);

impl<T> OptionalInput<T> {
    pub(crate) fn into_inner(self) -> Option<T> {
        self.0
    }
}

impl<T> From<T> for OptionalInput<T> {
    fn from(value: T) -> Self {
        Self(Some(value))
    }
}

impl<T> From<Option<T>> for OptionalInput<T> {
    fn from(value: Option<T>) -> Self {
        Self(value)
    }
}

impl From<&NaturalDate> for OptionalInput<NaturalDate> {
    fn from(value: &NaturalDate) -> Self {
        Self::from(*value)
    }
}

impl TryFrom<&str> for OptionalInput<NaturalDate> {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        NaturalDate::try_from(value).map(Self::from)
    }
}

impl TryFrom<String> for OptionalInput<NaturalDate> {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        NaturalDate::try_from(value).map(Self::from)
    }
}

impl TryFrom<&String> for OptionalInput<NaturalDate> {
    type Error = ValidationError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        NaturalDate::try_from(value).map(Self::from)
    }
}

impl From<&ReportType> for OptionalInput<ReportType> {
    fn from(value: &ReportType) -> Self {
        Self::from(value.clone())
    }
}

impl TryFrom<&str> for OptionalInput<ReportType> {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        ReportType::try_from(value).map(Self::from)
    }
}

impl TryFrom<String> for OptionalInput<ReportType> {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        ReportType::try_from(value).map(Self::from)
    }
}

impl TryFrom<&String> for OptionalInput<ReportType> {
    type Error = ValidationError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        ReportType::try_from(value).map(Self::from)
    }
}
