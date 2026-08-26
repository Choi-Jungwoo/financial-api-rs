use crate::endpoints;
use crate::{
    Client, Error, FundDrawdownsData, FundIndicatorHistoryData, FundNavData, FundNavType,
    FundRange, FundReturnsData, FundType, Response, Thscode, UnixMillis,
};

use super::{FundTarget, validate_history_range};

impl Client {
    /// 获取基金净值序列。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/fund_performance_nav.rs"),
        "\n```"
    )]
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

    /// 获取基金多个区间的收益率及同类排名。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/fund_performance_returns.rs"),
        "\n```"
    )]
    pub async fn fund_performance_returns(
        &self,
        fund_type: FundType,
        thscode: &Thscode,
    ) -> Result<Response<FundReturnsData>, Error> {
        self.fund_detail(endpoints::FUND_RETURNS, fund_type, thscode)
            .await
    }

    /// 获取基金历史业绩指标。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/fund_performance_indicators_historical.rs"),
        "\n```"
    )]
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

    /// 获取标准区间的最大回撤。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/fund_performance_drawdowns.rs"),
        "\n```"
    )]
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
