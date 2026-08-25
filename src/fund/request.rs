use crate::{FundType, Thscode, UnixMillis, ValidationError};
use time::OffsetDateTime;

const NANOS_PER_MILLISECOND: i128 = 1_000_000;
const HISTORY_YEARS: i32 = 5;

pub(super) fn validate_target(
    fund_type: FundType,
    thscode: &Thscode,
) -> Result<(), ValidationError> {
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
    Ok(())
}

pub(super) fn validate_exchange_target(thscode: &Thscode) -> Result<(), ValidationError> {
    let (ticker, suffix) = thscode
        .as_str()
        .split_once('.')
        .expect("validated thscode contains a suffix");
    let valid = ticker.len() == 6
        && ticker.bytes().all(|byte| byte.is_ascii_digit())
        && matches!(suffix, "SH" | "SZ");
    if !valid {
        return Err(ValidationError::new(
            "thscode",
            "exchange fund code must be six digits ending in SH or SZ",
        ));
    }
    Ok(())
}

pub(super) fn validate_history_range(
    start: UnixMillis,
    end: UnixMillis,
) -> Result<(), ValidationError> {
    validate_range_order(start, end)?;
    validate_five_year_window(start, end)
}

fn validate_range_order(start: UnixMillis, end: UnixMillis) -> Result<(), ValidationError> {
    if end < start {
        return Err(ValidationError::new(
            "end",
            "must not be earlier than start",
        ));
    }
    Ok(())
}

fn validate_five_year_window(start: UnixMillis, end: UnixMillis) -> Result<(), ValidationError> {
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
    use super::{validate_exchange_target, validate_history_range, validate_target};
    use crate::{FundType, Thscode, UnixMillis};

    #[test]
    fn fund_type_must_match_the_validated_code_suffix() {
        let otc = Thscode::new("025480.OF").unwrap();
        let exchange = Thscode::new("510300.SH").unwrap();

        assert!(validate_target(FundType::Otc, &otc).is_ok());
        assert!(validate_target(FundType::Exchange, &exchange).is_ok());
        assert!(validate_target(FundType::Reits, &exchange).is_ok());

        let error = validate_target(FundType::Otc, &exchange).unwrap_err();
        assert_eq!(error.field(), "thscode");
        assert_eq!(error.problem(), "market suffix does not match fund_type");
        assert!(validate_target(FundType::Exchange, &otc).is_err());
        assert!(validate_target(FundType::Reits, &otc).is_err());
    }

    #[test]
    fn exchange_market_data_requires_a_six_digit_exchange_code() {
        assert!(validate_exchange_target(&Thscode::new("510300.SH").unwrap()).is_ok());
        assert!(validate_exchange_target(&Thscode::new("159915.SZ").unwrap()).is_ok());

        for invalid in ["025480.OF", "ABCDEF.SH", "51030.SH", "510300.BJ"] {
            let error = validate_exchange_target(&Thscode::new(invalid).unwrap()).unwrap_err();
            assert_eq!(error.field(), "thscode");
            assert_eq!(
                error.problem(),
                "exchange fund code must be six digits ending in SH or SZ"
            );
        }
    }

    #[test]
    fn historical_range_must_not_end_before_it_starts() {
        let start = UnixMillis::new(2).unwrap();
        let end = UnixMillis::new(1).unwrap();

        let error = validate_history_range(start, end).unwrap_err();

        assert_eq!(error.field(), "end");
        assert_eq!(error.problem(), "must not be earlier than start");
    }

    #[test]
    fn historical_window_uses_calendar_years_at_a_leap_day_boundary() {
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
}
