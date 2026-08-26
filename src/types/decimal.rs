use serde::{Deserialize, Serialize};

/// 保留源数字且不转换为二进制浮点数的 JSON 十进制数。
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, derive_more::Display, derive_more::From,
)]
#[serde(transparent)]
#[display("{_0}")]
pub struct PreciseDecimal(serde_json::Number);

impl PreciseDecimal {
    /// 借用无损 JSON 数字表示。
    #[must_use]
    pub const fn as_number(&self) -> &serde_json::Number {
        &self.0
    }
}
