use std::num::NonZeroU32;

use crate::endpoints;
use crate::endpoints::join_values;
use crate::{
    AShareCode, Adjustment, Client, Error, HistoricalData, PriceSnapshotData, Response, UnixMillis,
    ValidationError,
};

use super::{TEN_YEARS_MS, validate_millis_window};

/// 显式指定的标的，或完整 A 股标的宇宙中的一页。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PriceSnapshotSelection(PriceSnapshotSelectionKind);

#[derive(Debug, Clone, PartialEq, Eq)]
enum PriceSnapshotSelectionKind {
    Targets(Vec<AShareCode>),
    MarketPage { limit: NonZeroU32, offset: u32 },
}

impl PriceSnapshotSelection {
    pub fn targets(
        targets: impl IntoIterator<Item = impl TryInto<AShareCode, Error: Into<ValidationError>>>,
    ) -> Result<Self, ValidationError> {
        let targets = targets
            .into_iter()
            .map(|target| target.try_into().map_err(Into::into))
            .collect::<Result<Vec<_>, _>>()?;
        if targets.is_empty() {
            return Err(ValidationError::new("thscodes", "must not be empty"));
        }
        Ok(Self(PriceSnapshotSelectionKind::Targets(targets)))
    }

    pub const fn market_page(limit: u32, offset: u32) -> Result<Self, ValidationError> {
        let Some(limit) = NonZeroU32::new(limit) else {
            return Err(ValidationError::new("limit", "must be at least 1"));
        };
        Ok(Self(PriceSnapshotSelectionKind::MarketPage {
            limit,
            offset,
        }))
    }
}

impl Client {
    /// 获取当前 A 股行情快照。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/prices_snapshot.rs"),
        "\n```"
    )]
    pub async fn prices_snapshot(
        &self,
        selection: &PriceSnapshotSelection,
    ) -> Result<Response<PriceSnapshotData>, Error> {
        let query = match &selection.0 {
            PriceSnapshotSelectionKind::Targets(codes) => {
                vec![("thscodes", join_values("thscodes", codes, None)?)]
            }
            PriceSnapshotSelectionKind::MarketPage { limit, offset } => vec![
                ("limit", limit.get().to_string()),
                ("offset", offset.to_string()),
            ],
        };
        self.get(endpoints::PRICES_SNAPSHOT, &query).await
    }

    /// 获取指定标的的历史日 K 线数据。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/prices_historical.rs"),
        "\n```"
    )]
    pub async fn prices_historical(
        &self,
        thscode: impl TryInto<AShareCode, Error: Into<ValidationError>> + Send,
        start: impl TryInto<UnixMillis, Error: Into<ValidationError>> + Send,
        end: impl TryInto<UnixMillis, Error: Into<ValidationError>> + Send,
        adjustment: Adjustment,
        offset: u32,
    ) -> Result<Response<HistoricalData>, Error> {
        let thscode: AShareCode = thscode.try_into().map_err(Into::into)?;
        let start: UnixMillis = start.try_into().map_err(Into::into)?;
        let end: UnixMillis = end.try_into().map_err(Into::into)?;
        validate_millis_window(start, end, Some(TEN_YEARS_MS))?;
        let query = vec![
            ("thscode", thscode.into_string()),
            ("interval", "1d".to_owned()),
            ("start", start.to_string()),
            ("end", end.to_string()),
            ("adjust", adjustment.to_string()),
            ("offset", offset.to_string()),
        ];
        self.get(endpoints::PRICES_HISTORICAL, &query).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ApiKey;

    #[test]
    fn snapshot_selection_rejects_empty_targets_and_zero_page_size() {
        assert!(PriceSnapshotSelection::targets(Vec::<&str>::new()).is_err());
        assert!(PriceSnapshotSelection::market_page(0, 0).is_err());
        assert!(PriceSnapshotSelection::market_page(10_001, 0).is_ok());

        let stock = AShareCode::new("600519.SH").unwrap();
        assert!(PriceSnapshotSelection::targets(vec![stock]).is_ok());
    }

    #[tokio::test]
    async fn historical_prices_keeps_the_shared_window_validation_on_its_call_edge() {
        let client = Client::builder(ApiKey::new("test-api-key").unwrap())
            .base_url("http://127.0.0.1:9")
            .build()
            .unwrap();
        let stock = AShareCode::new("600519.SH").unwrap();
        let start = UnixMillis::new(1_000_000_000_000).unwrap();
        let invalid_ends = [
            UnixMillis::new(start.get() - 1).unwrap(),
            UnixMillis::new(start.get() + TEN_YEARS_MS + 1).unwrap(),
        ];

        for end in invalid_ends {
            let error = client
                .prices_historical(&stock, start.get(), end.get(), Adjustment::None, 0)
                .await
                .unwrap_err();
            assert!(matches!(error, Error::InvalidInput(_)));
        }
    }
}
