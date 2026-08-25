mod auction;
mod calendar;
mod corporate_actions;
mod financials;
mod prices;
mod special_data;
mod valuations;

pub use prices::PriceSnapshotSelection;

use crate::{NaturalDate, UnixMillis, ValidationError};

const TEN_YEARS_MS: i64 = 315_576_000_000;

fn validate_millis_window(
    start: UnixMillis,
    end: UnixMillis,
    maximum: Option<i64>,
) -> Result<(), ValidationError> {
    if end < start {
        return Err(ValidationError::new(
            "end",
            "must not be earlier than start",
        ));
    }
    if maximum.is_some_and(|maximum| end.get() - start.get() > maximum) {
        return Err(ValidationError::new(
            "end",
            "requested time window exceeds the endpoint limit",
        ));
    }
    Ok(())
}

fn validate_date_order(
    start: NaturalDate,
    end: NaturalDate,
    end_field: &'static str,
) -> Result<(), ValidationError> {
    if end < start {
        return Err(ValidationError::new(
            end_field,
            "must not be earlier than the start date",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{TEN_YEARS_MS, validate_date_order, validate_millis_window};
    use crate::{NaturalDate, UnixMillis};

    #[test]
    fn shared_ranges_preserve_endpoint_order_and_limit_rules() {
        let start = UnixMillis::new(1_000_000_000_000).unwrap();
        let before_start = UnixMillis::new(start.get() - 1).unwrap();
        let limit = UnixMillis::new(start.get() + TEN_YEARS_MS).unwrap();
        let after_limit = UnixMillis::new(limit.get() + 1).unwrap();

        assert!(validate_millis_window(start, before_start, None).is_err());
        assert!(validate_millis_window(start, limit, Some(TEN_YEARS_MS)).is_ok());
        assert!(validate_millis_window(start, after_limit, Some(TEN_YEARS_MS)).is_err());

        let first = NaturalDate::parse("2026-08-24").unwrap();
        let second = NaturalDate::parse("2026-08-25").unwrap();
        assert!(validate_date_order(first, second, "to").is_ok());
        let error = validate_date_order(second, first, "to").unwrap_err();
        assert_eq!(error.field(), "to");
    }
}
