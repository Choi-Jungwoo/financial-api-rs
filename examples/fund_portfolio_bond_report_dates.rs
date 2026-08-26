#[path = "configuration/client.rs"]
mod example_client;

use financial_api::{FundType, ReportType, Thscode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = example_client::from_env()?;
    let fund = Thscode::new("025480.OF")?;
    let report_type = ReportType::new("quarter")?;
    let response = client
        .fund_portfolio_bond_report_dates(FundType::Otc, &fund, Some(&report_type))
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
