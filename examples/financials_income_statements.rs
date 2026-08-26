#[path = "configuration/client.rs"]
mod example_client;

use financial_api::{AShareCode, FinancialPeriod, FinancialRange};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = example_client::from_env()?;
    let target = AShareCode::new("600519.SH")?;
    let range = FinancialRange::recent(4)?;
    let response = client
        .financials_income_statements(&target, FinancialPeriod::Annual, range)
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
