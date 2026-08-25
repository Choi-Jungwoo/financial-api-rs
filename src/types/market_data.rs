use super::wire::wire_enum;

wire_enum! {
    /// Historical price adjustment convention.
    pub enum Adjustment {
        None => "none",
        Forward => "forward",
        Backward => "backward",
    }
}

wire_enum! {
    /// Fixed daily K-line interval returned by fund historical prices.
    pub enum DailyInterval {
        OneDay => "1d",
    }
}
