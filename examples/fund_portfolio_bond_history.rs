#[path = "configuration/client.rs"]
mod example_client;

use financial_api::{FundType, NaturalDate, ReportType, Thscode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = example_client::from_env()?;
    let fund = Thscode::new("025480.OF")?;
    let report_type = ReportType::new("quarter")?;
    let end_date = NaturalDate::parse("2026-06-30")?;
    let response = client
        .fund_portfolio_bond_history(FundType::Otc, &fund, &report_type, end_date)
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
