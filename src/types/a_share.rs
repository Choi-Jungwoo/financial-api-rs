use super::wire::wire_enum;
use crate::ValidationError;

wire_enum! {
    /// 集合竞价快照请求接受的阶段。
    pub enum AuctionStage {
        Live => "live",
        Final => "final",
    }
}

wire_enum! {
    /// 集合竞价快照响应中的阶段。
    pub enum AuctionPhase {
        Live => "live",
        Final => "final",
        Closed => "closed",
    }
}

wire_enum! {
    /// A 股异动标签。
    pub enum AnomalyTag {
        LimitUp => "LIMIT_UP",
        LimitDown => "LIMIT_DOWN",
        SharpRise => "SHARP_RISE",
        SharpFall => "SHARP_FALL",
        RapidRally => "RAPID_RALLY",
        RapidDecline => "RAPID_DECLINE",
    }
}

wire_enum! {
    /// 排名聚合周期。
    pub enum HotListPeriod {
        Day => "day",
        Hour => "hour",
    }
}

wire_enum! {
    /// 龙虎榜榜单筛选条件。
    pub enum DragonTigerBoard {
        All => "all",
        Organization => "org",
        HotMoney => "hot_money",
    }
}

wire_enum! {
    /// 特色数据分页池使用的排序方向。
    pub enum SortDirection {
        Ascending => "asc",
        Descending => "desc",
    }
}

wire_enum! {
    /// 涨停池排序字段。
    pub enum LimitUpSortField {
        LastPrice => "last_price",
        ConsecutiveDays => "continue_day_cnt",
        SealedAmount => "seal_money",
        LimitUpTime => "limit_up_time",
    }
}

wire_enum! {
    /// 跌停池排序字段。
    pub enum LimitDownSortField {
        LastLimitTime => "last_limit_time",
        FirstLimitTime => "first_limit_time",
        LastPrice => "last_price",
        PriceChangeRatio => "price_change_ratio_pct",
        TurnoverRatio => "turnover_ratio_pct",
    }
}

wire_enum! {
    /// 炸板池排序字段。
    pub enum LimitBreakSortField {
        PriceChangeRatio => "price_change_ratio_pct",
        OpenTimes => "open_times",
        LastPrice => "last_price",
        TurnoverRatio => "turnover_ratio_pct",
        Turnover => "turnover",
    }
}

/// 特色数据池使用的已校验分页参数。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Page {
    number: u32,
    size: u16,
}

impl Page {
    pub const fn new(number: u32, size: u16) -> Result<Self, ValidationError> {
        if number == 0 {
            return Err(ValidationError::new("page", "must be at least 1"));
        }
        if size == 0 || size > 200 {
            return Err(ValidationError::new("size", "must be in the range 1..=200"));
        }
        Ok(Self { number, size })
    }

    #[must_use]
    pub const fn number(self) -> u32 {
        self.number
    }

    #[must_use]
    pub const fn size(self) -> u16 {
        self.size
    }
}

impl Default for Page {
    fn default() -> Self {
        Self {
            number: 1,
            size: 50,
        }
    }
}

wire_enum! {
    /// 热榜排名变化方向。
    pub enum RankTrend {
        Up => "up",
        Down => "down",
        Flat => "flat",
        Unknown => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::Page;

    #[test]
    fn special_data_page_bounds_are_enforced() {
        assert!(Page::new(0, 50).is_err());
        assert!(Page::new(1, 0).is_err());
        assert!(Page::new(1, 1).is_ok());
        assert!(Page::new(u32::MAX, 200).is_ok());
        assert!(Page::new(1, 201).is_err());
    }
}
