use serde::de::DeserializeOwned;

use crate::endpoints;
use crate::endpoints::{join_a_share_codes, join_values};
use crate::{
    AShareCode, AnomalyData, AnomalyTag, Client, DragonTigerBoard, DragonTigerData, Error,
    HotListPeriod, HotStockData, HotStockHistoryData, HotStockTrendData, LadderData,
    LimitBreakData, LimitBreakSortField, LimitDownData, LimitDownSortField, LimitUpData,
    LimitUpSortField, NaturalDate, OptionalInput, Page, Response, ShanghaiDateMillis,
    SortDirection, ValidationError,
};

use super::validate_date_order;

impl Client {
    /// 获取当日异动分析记录，可按“或”关系组合标签筛选。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/special_data_anomaly_analysis_list.rs"),
        "\n```"
    )]
    pub async fn special_data_anomaly_analysis_list(
        &self,
        tag_codes: &[AnomalyTag],
    ) -> Result<Response<AnomalyData>, Error> {
        let query = if tag_codes.is_empty() {
            Vec::new()
        } else {
            vec![("tag_codes", join_values("tag_codes", tag_codes, None)?)]
        };
        self.get(endpoints::ANOMALY_LIST, &query).await
    }

    /// 获取最多 50 个 A 股标的的当日异动分析记录。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/special_data_anomaly_analysis_stock.rs"),
        "\n```"
    )]
    pub async fn special_data_anomaly_analysis_stock(
        &self,
        thscodes: impl IntoIterator<Item = impl TryInto<AShareCode, Error: Into<ValidationError>>>
        + Send,
    ) -> Result<Response<AnomalyData>, Error> {
        let query = [("thscodes", join_a_share_codes(thscodes, Some(50))?)];
        self.get(endpoints::ANOMALY_STOCK, &query).await
    }

    /// 获取龙虎榜。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/special_data_dragon_tiger_list.rs"),
        "\n```"
    )]
    pub async fn special_data_dragon_tiger_list(
        &self,
        board_type: DragonTigerBoard,
        date: impl TryInto<OptionalInput<NaturalDate>, Error: Into<ValidationError>> + Send,
    ) -> Result<Response<DragonTigerData>, Error> {
        let date: OptionalInput<NaturalDate> = date.try_into().map_err(Into::into)?;
        let date = date.into_inner();
        if let Some(date) = date {
            self.validate_recent_date(date, "date")?;
        }
        let mut query = vec![("board_type", board_type.to_string())];
        if let Some(date) = date {
            query.push(("date", date.to_string()));
        }
        self.get(endpoints::DRAGON_TIGER, &query).await
    }

    /// 获取当前个股热榜。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/special_data_hot_stock_list.rs"),
        "\n```"
    )]
    pub async fn special_data_hot_stock_list(
        &self,
        period: HotListPeriod,
    ) -> Result<Response<HotStockData>, Error> {
        self.get(endpoints::HOT_STOCK_LIST, &[("period", period.to_string())])
            .await
    }

    /// 获取指定自然日的历史个股热榜。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/special_data_hot_stock_list_history.rs"),
        "\n```"
    )]
    pub async fn special_data_hot_stock_list_history(
        &self,
        date: impl TryInto<NaturalDate, Error: Into<ValidationError>> + Send,
    ) -> Result<Response<HotStockHistoryData>, Error> {
        let date: NaturalDate = date.try_into().map_err(Into::into)?;
        self.validate_recent_date(date, "date")?;
        self.get(endpoints::HOT_STOCK_HISTORY, &[("date", date.to_string())])
            .await
    }

    /// 获取指定 A 股标的的热榜排名趋势。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/special_data_hot_stock_rank_trend.rs"),
        "\n```"
    )]
    pub async fn special_data_hot_stock_rank_trend(
        &self,
        thscode: impl TryInto<AShareCode, Error: Into<ValidationError>> + Send,
        start_date: impl TryInto<NaturalDate, Error: Into<ValidationError>> + Send,
        end_date: impl TryInto<NaturalDate, Error: Into<ValidationError>> + Send,
    ) -> Result<Response<HotStockTrendData>, Error> {
        let thscode: AShareCode = thscode.try_into().map_err(Into::into)?;
        let start_date: NaturalDate = start_date.try_into().map_err(Into::into)?;
        let end_date: NaturalDate = end_date.try_into().map_err(Into::into)?;
        validate_date_order(start_date, end_date, "end_date")?;
        self.validate_recent_date(start_date, "start_date")?;
        self.validate_recent_date(end_date, "end_date")?;
        if start_date
            .checked_add_years(1)
            .is_none_or(|limit| end_date > limit)
        {
            return Err(
                ValidationError::new("end_date", "date range must not exceed one year").into(),
            );
        }
        let query = [
            ("thscode", thscode.into_string()),
            ("start_date", start_date.to_string()),
            ("end_date", end_date.to_string()),
        ];
        self.get(endpoints::HOT_STOCK_TREND, &query).await
    }

    /// 获取涨停池。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/special_data_limit_up_pool.rs"),
        "\n```"
    )]
    pub async fn special_data_limit_up_pool(
        &self,
        date: impl TryInto<OptionalInput<NaturalDate>, Error: Into<ValidationError>> + Send,
        page: Page,
        sort_field: LimitUpSortField,
        sort_dir: SortDirection,
    ) -> Result<Response<LimitUpData>, Error> {
        self.special_data_pool(endpoints::LIMIT_UP_POOL, date, page, sort_field, sort_dir)
            .await
    }

    /// 获取跌停池。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/special_data_limit_down_pool.rs"),
        "\n```"
    )]
    pub async fn special_data_limit_down_pool(
        &self,
        date: impl TryInto<OptionalInput<NaturalDate>, Error: Into<ValidationError>> + Send,
        page: Page,
        sort_field: LimitDownSortField,
        sort_dir: SortDirection,
    ) -> Result<Response<LimitDownData>, Error> {
        self.special_data_pool(endpoints::LIMIT_DOWN_POOL, date, page, sort_field, sort_dir)
            .await
    }

    /// 获取炸板池。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/special_data_limit_break_pool.rs"),
        "\n```"
    )]
    pub async fn special_data_limit_break_pool(
        &self,
        date: impl TryInto<OptionalInput<NaturalDate>, Error: Into<ValidationError>> + Send,
        page: Page,
        sort_field: LimitBreakSortField,
        sort_dir: SortDirection,
    ) -> Result<Response<LimitBreakData>, Error> {
        self.special_data_pool(
            endpoints::LIMIT_BREAK_POOL,
            date,
            page,
            sort_field,
            sort_dir,
        )
        .await
    }

    async fn special_data_pool<T: DeserializeOwned>(
        &self,
        path: &str,
        date: impl TryInto<OptionalInput<NaturalDate>, Error: Into<ValidationError>> + Send,
        page: Page,
        sort_field: impl std::fmt::Display,
        sort_dir: SortDirection,
    ) -> Result<Response<T>, Error> {
        let date: OptionalInput<NaturalDate> = date.try_into().map_err(Into::into)?;
        let mut query = vec![
            ("page", page.number().to_string()),
            ("size", page.size().to_string()),
            ("sort_field", sort_field.to_string()),
            ("sort_dir", sort_dir.to_string()),
        ];
        if let Some(date) = date.into_inner() {
            let date_ms = ShanghaiDateMillis::from_date(date)?;
            query.push(("date_ms", date_ms.to_string()));
        }
        self.get(path, &query).await
    }

    fn validate_recent_date(
        &self,
        date: NaturalDate,
        field: &'static str,
    ) -> Result<(), ValidationError> {
        let today = self.shanghai_today();
        let earliest = today
            .checked_add_years(-1)
            .ok_or_else(|| ValidationError::new(field, "could not calculate one-year window"))?;
        if date < earliest || date > today {
            return Err(ValidationError::new(
                field,
                "must be within the previous natural year and not in the future",
            ));
        }
        Ok(())
    }

    /// 获取固定覆盖 30 个交易日的连板天梯。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/special_data_limit_up_ladder.rs"),
        "\n```"
    )]
    pub async fn special_data_limit_up_ladder(&self) -> Result<Response<LadderData>, Error> {
        self.get(endpoints::LIMIT_UP_LADDER, &[] as &[(&str, &str)])
            .await
    }

    /// 获取当前飙升榜。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/special_data_skyrocket_list.rs"),
        "\n```"
    )]
    pub async fn special_data_skyrocket_list(
        &self,
        period: HotListPeriod,
    ) -> Result<Response<HotStockData>, Error> {
        self.get(endpoints::SKYROCKET_LIST, &[("period", period.to_string())])
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AShareCode, ApiKey};

    fn client() -> Client {
        Client::builder(ApiKey::new("test-api-key").unwrap())
            .base_url("http://127.0.0.1:9")
            .reference_date(NaturalDate::parse("2026-08-25").unwrap())
            .build()
            .unwrap()
    }

    #[tokio::test]
    async fn stock_analysis_rejects_more_than_fifty_targets_before_transport() {
        let stock = AShareCode::new("600519.SH").unwrap();

        let error = client()
            .special_data_anomaly_analysis_stock(&vec![stock; 51])
            .await
            .unwrap_err();

        assert!(matches!(error, Error::InvalidInput(_)));
    }

    #[tokio::test]
    async fn recent_date_capabilities_enforce_their_documented_windows() {
        let client = client();
        let stock = AShareCode::new("600519.SH").unwrap();

        for error in [
            client
                .special_data_hot_stock_rank_trend(&stock, "2024-02-29", "2025-03-01")
                .await
                .unwrap_err(),
            client
                .special_data_hot_stock_list_history("2025-08-24")
                .await
                .unwrap_err(),
            client
                .special_data_dragon_tiger_list(
                    DragonTigerBoard::All,
                    Some(NaturalDate::parse("2026-08-26").unwrap()),
                )
                .await
                .unwrap_err(),
        ] {
            assert!(matches!(error, Error::InvalidInput(_)));
        }
    }
}
