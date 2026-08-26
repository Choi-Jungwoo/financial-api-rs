#[path = "configuration/client.rs"]
mod example_client;

use financial_api::{FundType, Thscode, UnixMillis};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = example_client::from_env()?;
    let fund = Thscode::new("025480.OF")?;
    let start = UnixMillis::new(1_716_105_600_000)?;
    let end = UnixMillis::new(1_716_192_000_000)?;
    let response = client
        .fund_performance_indicators_historical(FundType::Otc, &fund, start, end)
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
