use serde::Deserialize;

use crate::UnixMillis;

/// 将业务记录保存在 `item` 中的带时间戳端点载荷。
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TimestampedItems<T> {
    pub timestamp: UnixMillis,
    pub item: Vec<T>,
}
