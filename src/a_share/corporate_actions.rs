use crate::endpoints;
use crate::{
    AShareCode, AdjustmentFactorsData, Client, Error, NaturalDate, OptionalInput, Response,
    ValidationError,
};

use super::validate_date_order;

impl Client {
    /// 获取原始复权事件。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/corp_actions_adjustment_factors.rs"),
        "\n```"
    )]
    pub async fn corp_actions_adjustment_factors(
        &self,
        thscode: impl TryInto<AShareCode, Error: Into<ValidationError>>,
        from: impl TryInto<OptionalInput<NaturalDate>, Error: Into<ValidationError>>,
        to: impl TryInto<OptionalInput<NaturalDate>, Error: Into<ValidationError>>,
    ) -> Result<Response<AdjustmentFactorsData>, Error> {
        let thscode: AShareCode = thscode.try_into().map_err(Into::into)?;
        let from: OptionalInput<NaturalDate> = from.try_into().map_err(Into::into)?;
        let to: OptionalInput<NaturalDate> = to.try_into().map_err(Into::into)?;
        let from = from.into_inner();
        let to = to.into_inner();
        let mut query = vec![("thscode", thscode.into_string())];
        if let (Some(from), Some(to)) = (from, to) {
            validate_date_order(from, to, "to")?;
        }
        if let Some(from) = from {
            query.push(("from", from.to_string()));
        }
        if let Some(to) = to {
            query.push(("to", to.to_string()));
        }
        self.get(endpoints::ADJUSTMENT_FACTORS, &query).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ApiKey;

    #[tokio::test]
    async fn adjustment_factors_rejects_reversed_dates_before_transport() {
        let client = Client::builder(ApiKey::new("test-api-key").unwrap())
            .base_url("http://127.0.0.1:9")
            .build()
            .unwrap();
        let stock = AShareCode::new("600519.SH").unwrap();
        let from = NaturalDate::parse("2026-08-25").unwrap();
        let to = NaturalDate::parse("2026-08-24").unwrap();

        let error = client
            .corp_actions_adjustment_factors(&stock, Some(from), Some(to))
            .await
            .unwrap_err();

        let Error::InvalidInput(error) = error else {
            panic!("expected an invalid corporate-action date range");
        };
        assert_eq!(error.field(), "to");
    }
}
