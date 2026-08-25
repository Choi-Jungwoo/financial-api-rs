use time::OffsetDateTime;
use time::macros::offset;

use super::Client;
use crate::NaturalDate;

impl Client {
    pub(crate) fn shanghai_today(&self) -> NaturalDate {
        self.reference_date.unwrap_or_else(|| {
            OffsetDateTime::now_utc()
                .to_offset(offset!(+8))
                .date()
                .into()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Client;
    use crate::{ApiKey, NaturalDate};

    #[test]
    fn configured_reference_date_owns_relative_date_validation_time() {
        let date = NaturalDate::parse("2026-08-25").unwrap();
        let client = Client::builder(ApiKey::new("test-api-key").unwrap())
            .reference_date(date)
            .build()
            .unwrap();

        assert_eq!(client.shanghai_today(), date);
    }
}
