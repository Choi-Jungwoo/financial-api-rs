use super::wire::wire_enum;

wire_enum! {
    /// 同花顺指数目录标签。
    pub enum IndexTag {
        Concept => "cn_concept",
        Region => "region",
        Featured => "tszs",
        Industry => "industry",
    }
}
