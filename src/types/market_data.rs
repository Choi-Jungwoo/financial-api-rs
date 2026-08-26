use super::wire::wire_enum;

wire_enum! {
    /// 历史行情复权口径。
    pub enum Adjustment {
        None => "none",
        Forward => "forward",
        Backward => "backward",
    }
}

wire_enum! {
    /// 基金历史行情返回的固定日 K 线周期。
    pub enum DailyInterval {
        OneDay => "1d",
    }
}
