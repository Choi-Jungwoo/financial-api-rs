use crate::endpoints;
use crate::{Client, Error, Response, TradingDaysData};

impl Client {
    /// Fetch the service's recent A-share trading-day sequence.
    pub async fn calendar_trading_days(&self) -> Result<Response<TradingDaysData>, Error> {
        self.get(endpoints::TRADING_DAYS, &[] as &[(&str, &str)])
            .await
    }
}
