include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/configuration/client.rs"
));

use financial_api::{Thscode, UnixMillis};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = from_env()?;
    let fund = Thscode::new("510050.SH")?;
    let start = UnixMillis::new(1_716_105_600_000)?;
    let end = UnixMillis::new(1_716_192_000_000)?;
    let response = client.fund_market_historical(&fund, start, end).await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
