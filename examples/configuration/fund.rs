use financial_api::{Client, FundType, ManagerId, Thscode};

pub async fn manager_id(client: &Client) -> Result<ManagerId, Box<dyn std::error::Error>> {
    let fund = Thscode::new("025480.OF")?;
    let profile = client.fund_profile_detail(FundType::Otc, &fund).await?;
    profile
        .data()
        .item
        .first()
        .and_then(|item| item.manager_info.first())
        .map(|manager| manager.manager_id.clone())
        .ok_or_else(|| std::io::Error::other("fund profile did not return a manager").into())
}
