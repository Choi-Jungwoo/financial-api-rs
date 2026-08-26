use serde::de::DeserializeOwned;

use crate::endpoints;
use crate::{
    Client, Error, FundManagerDetailData, FundManagerExperienceData, FundManagerPerformanceData,
    FundManagerStyleData, ManagerId, ManagerPerformanceRange, Response,
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
        manager_id: &ManagerId,
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
        manager_id: &ManagerId,
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
        manager_id: &ManagerId,
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
        manager_id: &ManagerId,
    ) -> Result<Response<FundManagerDetailData>, Error> {
        self.fund_manager(endpoints::FUND_MANAGER_DETAIL, manager_id, None)
            .await
    }

    async fn fund_manager<T: DeserializeOwned>(
        &self,
        path: &str,
        manager_id: &ManagerId,
        range: Option<ManagerPerformanceRange>,
    ) -> Result<Response<T>, Error> {
        let mut query = vec![("manager_id", manager_id.to_string())];
        if let Some(range) = range {
            query.push(("range", range.to_string()));
        }
        self.get(path, &query).await
    }
}
