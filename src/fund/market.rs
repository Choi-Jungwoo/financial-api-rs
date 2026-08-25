use crate::endpoints;
use crate::{
    Client, Error, FundMarketHistoricalData, FundMarketSnapshotData, Response, Thscode, UnixMillis,
    ValidationError,
};

use super::validate_history_range;

impl Client {
    /// Fetch the latest exchange-traded fund snapshot.
    pub async fn fund_market_snapshot(
        &self,
        thscode: &Thscode,
    ) -> Result<Response<FundMarketSnapshotData>, Error> {
        validate_exchange_fund(thscode)?;
        self.get(
            endpoints::FUND_MARKET_SNAPSHOT,
            &[("thscode", thscode.as_str())],
        )
        .await
    }

    /// Fetch exchange-traded fund historical daily prices.
    pub async fn fund_market_historical(
        &self,
        thscode: &Thscode,
        start: UnixMillis,
        end: UnixMillis,
    ) -> Result<Response<FundMarketHistoricalData>, Error> {
        validate_exchange_fund(thscode)?;
        validate_history_range(start, end)?;
        let query = [
            ("thscode", thscode.to_string()),
            ("interval", "1d".to_owned()),
            ("start", start.to_string()),
            ("end", end.to_string()),
        ];
        self.get(endpoints::FUND_MARKET_HISTORICAL, &query).await
    }
}

fn validate_exchange_fund(thscode: &Thscode) -> Result<(), ValidationError> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ApiKey;

    #[test]
    fn exchange_market_data_requires_a_six_digit_exchange_code() {
        assert!(validate_exchange_fund(&Thscode::new("510300.SH").unwrap()).is_ok());
        assert!(validate_exchange_fund(&Thscode::new("159915.SZ").unwrap()).is_ok());

        for invalid in ["025480.OF", "ABCDEF.SH", "51030.SH", "510300.BJ"] {
            let error = validate_exchange_fund(&Thscode::new(invalid).unwrap()).unwrap_err();
            assert_eq!(error.field(), "thscode");
        }
    }

    #[tokio::test]
    async fn snapshot_keeps_exchange_validation_on_its_call_edge() {
        let client = Client::builder(ApiKey::new("test-api-key").unwrap())
            .base_url("http://127.0.0.1:9")
            .build()
            .unwrap();
        let otc_fund = Thscode::new("025480.OF").unwrap();

        let error = client.fund_market_snapshot(&otc_fund).await.unwrap_err();

        assert!(matches!(error, Error::InvalidInput(_)));
    }
}
