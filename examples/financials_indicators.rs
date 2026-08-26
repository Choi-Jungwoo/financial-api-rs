#[path = "configuration/client.rs"]
mod example_client;

use financial_api::{AShareCode, FinancialReport};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = example_client::from_env()?;
    let target = AShareCode::new("600519.SH")?;
    let report = FinancialReport::parse("2025-4")?;
    let response = client.financials_indicators(&target, &report).await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
