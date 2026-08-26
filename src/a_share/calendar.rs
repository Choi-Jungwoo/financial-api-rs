use crate::endpoints;
use crate::{Client, Error, Response, TradingDaysData};

impl Client {
    /// 获取服务提供的近期 A 股交易日序列。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/calendar_trading_days.rs"),
        "\n```"
    )]
    pub async fn calendar_trading_days(&self) -> Result<Response<TradingDaysData>, Error> {
        self.get(endpoints::TRADING_DAYS, &[] as &[(&str, &str)])
            .await
    }
}
