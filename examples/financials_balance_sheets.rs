include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/configuration/client.rs"
));

use financial_api::{AShareCode, FinancialPeriod, FinancialRange};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = from_env()?;
    let target = AShareCode::new("600519.SH")?;
    let range = FinancialRange::recent(4)?;
    let response = client
        .financials_balance_sheets(&target, FinancialPeriod::Annual, range)
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
