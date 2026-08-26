#[path = "configuration/client.rs"]
mod example_client;

use financial_api::{AShareCode, Adjustment, UnixMillis};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = example_client::from_env()?;
    let target = AShareCode::new("600519.SH")?;
    let start = UnixMillis::new(1_716_105_600_000)?;
    let end = UnixMillis::new(1_716_192_000_000)?;
    let response = client
        .prices_historical(&target, start, end, Adjustment::None, 0)
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
