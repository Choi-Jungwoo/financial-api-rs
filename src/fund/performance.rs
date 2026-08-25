use crate::endpoints;
use crate::{
    Client, Error, FundDrawdownsData, FundIndicatorHistoryData, FundNavData, FundNavType,
    FundRange, FundReturnsData, FundType, Response, Thscode, UnixMillis,
};

use super::{FundTarget, validate_history_range};

impl Client {
    /// Fetch fund NAV series.
    pub async fn fund_performance_nav(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
        range: Option<FundRange>,
        nav_type: FundNavType,
    ) -> Result<Response<FundNavData>, Error> {
        let mut query = FundTarget::new(fund_type, thscode)?.query();
        if let Some(range) = range {
            query.push(("range", range.to_string()));
        }
        query.push(("nav_type", nav_type.to_string()));
        self.get(endpoints::FUND_NAV, &query).await
    }

    /// Fetch multi-range fund returns and peer rankings.
    pub async fn fund_performance_returns(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
    ) -> Result<Response<FundReturnsData>, Error> {
        self.fund_detail(endpoints::FUND_RETURNS, fund_type, thscode)
            .await
    }

    /// Fetch historical fund performance indicators.
    pub async fn fund_performance_indicators_historical(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
        start: UnixMillis,
        end: UnixMillis,
    ) -> Result<Response<FundIndicatorHistoryData>, Error> {
        let target = FundTarget::new(fund_type, thscode)?;
        validate_history_range(start, end)?;
        let mut query = target.query();
        query.push(("start", start.to_string()));
        query.push(("end", end.to_string()));
        self.get(endpoints::FUND_INDICATORS_HISTORICAL, &query)
            .await
    }

    /// Fetch standard-period maximum drawdowns.
    pub async fn fund_performance_drawdowns(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
    ) -> Result<Response<FundDrawdownsData>, Error> {
        self.fund_detail(endpoints::FUND_DRAWDOWNS, fund_type, thscode)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ApiKey;

    #[tokio::test]
    async fn historical_indicators_keeps_range_validation_on_its_call_edge() {
        let client = Client::builder(ApiKey::new("test-api-key").unwrap())
            .base_url("http://127.0.0.1:9")
            .build()
            .unwrap();
        let fund = Thscode::new("025480.OF").unwrap();

        let error = client
            .fund_performance_indicators_historical(
                FundType::Otc,
                &fund,
                UnixMillis::new(1_577_836_800_000).unwrap(),
                UnixMillis::new(1_735_776_000_000).unwrap(),
            )
            .await
            .unwrap_err();

        assert!(matches!(error, Error::InvalidInput(_)));
    }
}
