#[path = "configuration/client.rs"]
mod example_client;

use financial_api::{Thscode, UnixMillis};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = example_client::from_env()?;
    let fund = Thscode::new("510300.SH")?;
    let start = UnixMillis::new(1_716_105_600_000)?;
    let end = UnixMillis::new(1_716_192_000_000)?;
    let response = client.fund_market_historical(&fund, start, end).await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
