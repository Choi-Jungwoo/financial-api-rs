mod company;
mod corporate_actions;
mod diagnostics;
mod financials;
mod holders;
mod managers;
mod market;
mod news;
mod offerings;
mod performance;
mod portfolio;
mod profile;

use serde::de::DeserializeOwned;
use time::OffsetDateTime;

use crate::{Client, Error, FundType, Response, Thscode, UnixMillis, ValidationError};

const HISTORY_YEARS: i32 = 5;
const NANOS_PER_MILLISECOND: i128 = 1_000_000;

impl Client {
    async fn fund_detail<T: DeserializeOwned>(
        &self,
        path: &str,
        fund_type: FundType,
        thscode: &Thscode,
    ) -> Result<Response<T>, Error> {
        let query = FundTarget::new(fund_type, thscode)?.query();
        self.get(path, &query).await
    }
}

struct FundTarget<'a> {
    fund_type: FundType,
    thscode: &'a Thscode,
}

impl<'a> FundTarget<'a> {
    fn new(fund_type: FundType, thscode: &'a Thscode) -> Result<Self, ValidationError> {
        let suffix = thscode
            .as_str()
            .rsplit_once('.')
            .map(|(_, suffix)| suffix)
            .expect("validated thscode contains a suffix");
        let valid = match fund_type {
            FundType::Otc => suffix == "OF",
            FundType::Exchange | FundType::Reits => matches!(suffix, "SH" | "SZ"),
        };
        if !valid {
            return Err(ValidationError::new(
                "thscode",
                "market suffix does not match fund_type",
            ));
        }
        Ok(Self { fund_type, thscode })
    }

    fn query(&self) -> Vec<(&'static str, String)> {
        vec![
            ("fund_type", self.fund_type.to_string()),
            ("thscode", self.thscode.to_string()),
        ]
    }
}

fn validate_history_range(start: UnixMillis, end: UnixMillis) -> Result<(), ValidationError> {
    if end < start {
        return Err(ValidationError::new(
            "end",
            "must not be earlier than start",
        ));
    }

    let start_datetime =
        OffsetDateTime::from_unix_timestamp_nanos(i128::from(start.get()) * NANOS_PER_MILLISECOND)
            .map_err(|_| {
                ValidationError::new("start", "timestamp is outside the supported range")
            })?;
    let target_year = start_datetime
        .year()
        .checked_add(HISTORY_YEARS)
        .ok_or_else(|| ValidationError::new("end", "timestamp year overflowed"))?;
    let limit = start_datetime
        .replace_year(target_year)
        .or_else(|_| start_datetime.replace_day(28)?.replace_year(target_year))
        .map_err(|_| ValidationError::new("end", "could not calculate the year limit"))?;
    if i128::from(end.get()) * NANOS_PER_MILLISECOND > limit.unix_timestamp_nanos() {
        return Err(ValidationError::new(
            "end",
            "requested time window exceeds the endpoint year limit",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{FundTarget, validate_history_range};
    use crate::{FundType, Thscode, UnixMillis};

    #[test]
    fn fund_type_must_match_the_target_suffix() {
        let otc = Thscode::new("025480.OF").unwrap();
        let exchange = Thscode::new("510300.SH").unwrap();

        assert!(FundTarget::new(FundType::Otc, &otc).is_ok());
        assert!(FundTarget::new(FundType::Exchange, &exchange).is_ok());
        assert!(FundTarget::new(FundType::Reits, &exchange).is_ok());
        assert!(FundTarget::new(FundType::Otc, &exchange).is_err());
        assert!(FundTarget::new(FundType::Exchange, &otc).is_err());
        assert!(FundTarget::new(FundType::Reits, &otc).is_err());
    }

    #[test]
    fn history_range_uses_calendar_years_at_a_leap_day_boundary() {
        let start = UnixMillis::new(1_582_934_400_000).unwrap();
        let five_year_limit = UnixMillis::new(1_740_700_800_000).unwrap();
        let after_limit = UnixMillis::new(1_740_787_200_000).unwrap();

        assert!(validate_history_range(start, five_year_limit).is_ok());
        let error = validate_history_range(start, after_limit).unwrap_err();
        assert_eq!(error.field(), "end");
        assert_eq!(
            error.problem(),
            "requested time window exceeds the endpoint year limit"
        );
    }

    #[test]
    fn history_range_rejects_an_end_before_the_start() {
        let start = UnixMillis::new(2).unwrap();
        let end = UnixMillis::new(1).unwrap();

        let error = validate_history_range(start, end).unwrap_err();

        assert_eq!(error.field(), "end");
        assert_eq!(error.problem(), "must not be earlier than start");
    }
}
