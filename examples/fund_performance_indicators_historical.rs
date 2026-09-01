use financial_api::{Client, Error, FundType};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::from_env()?;
    let response = client
        .fund_performance_indicators_historical(
            FundType::Otc,
            "025480.OF",
            1_716_105_600_000,
            1_716_192_000_000,
        )
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
