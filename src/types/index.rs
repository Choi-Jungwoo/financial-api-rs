use super::wire::wire_enum;

wire_enum! {
    /// Tonghuashun index catalog tag.
    pub enum IndexTag {
        Concept => "cn_concept",
        Region => "region",
        Featured => "tszs",
        Industry => "industry",
    }
}
