use financial_api::{Client, Error, FundNavType, FundRange, FundType};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::from_env()?;
    let response = client
        .fund_performance_nav(
            FundType::Otc,
            "025480.OF",
            Some(FundRange::Month),
            FundNavType::Unit,
        )
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
