use serde::de::DeserializeOwned;

use crate::endpoints;
use crate::{
    Client, Error, FundManagerDetailData, FundManagerExperienceData, FundManagerPerformanceData,
    FundManagerStyleData, ManagerId, ManagerPerformanceRange, Response, ValidationError,
};

impl Client {
    /// 获取基金经理的投资风格。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/fund_managers_investment_style.rs"),
        "\n```"
    )]
    pub async fn fund_managers_investment_style(
        &self,
        manager_id: impl TryInto<ManagerId, Error: Into<ValidationError>>,
    ) -> Result<Response<FundManagerStyleData>, Error> {
        self.fund_manager(endpoints::FUND_MANAGER_STYLE, manager_id, None)
            .await
    }

    /// 获取基金经理的业绩序列。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/fund_managers_performance.rs"),
        "\n```"
    )]
    pub async fn fund_managers_performance(
        &self,
        manager_id: impl TryInto<ManagerId, Error: Into<ValidationError>>,
        range: ManagerPerformanceRange,
    ) -> Result<Response<FundManagerPerformanceData>, Error> {
        self.fund_manager(endpoints::FUND_MANAGER_PERFORMANCE, manager_id, Some(range))
            .await
    }

    /// 获取基金经理的从业经历。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/fund_managers_experience.rs"),
        "\n```"
    )]
    pub async fn fund_managers_experience(
        &self,
        manager_id: impl TryInto<ManagerId, Error: Into<ValidationError>>,
    ) -> Result<Response<FundManagerExperienceData>, Error> {
        self.fund_manager(endpoints::FUND_MANAGER_EXPERIENCE, manager_id, None)
            .await
    }

    /// 获取基金经理的详细资料。
    ///
    /// # 示例
    #[doc = concat!(
        "```no_run\n",
        include_str!("../../examples/fund_managers_detail.rs"),
        "\n```"
    )]
    pub async fn fund_managers_detail(
        &self,
        manager_id: impl TryInto<ManagerId, Error: Into<ValidationError>>,
    ) -> Result<Response<FundManagerDetailData>, Error> {
        self.fund_manager(endpoints::FUND_MANAGER_DETAIL, manager_id, None)
            .await
    }

    async fn fund_manager<T: DeserializeOwned>(
        &self,
        path: &str,
        manager_id: impl TryInto<ManagerId, Error: Into<ValidationError>>,
        range: Option<ManagerPerformanceRange>,
    ) -> Result<Response<T>, Error> {
        let manager_id: ManagerId = manager_id.try_into().map_err(Into::into)?;
        if let Some(range) = range {
            return self
                .get(
                    path,
                    &[
                        ("manager_id", manager_id.as_str()),
                        ("range", range.as_str()),
                    ],
                )
                .await;
        }
        self.get(path, &[("manager_id", manager_id.as_str())]).await
    }
}
