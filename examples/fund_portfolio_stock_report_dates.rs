include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/configuration/client.rs"
));

use financial_api::{FundType, ReportType, Thscode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = from_env()?;
    let fund = Thscode::new("025480.OF")?;
    let report_type = ReportType::new("quarter")?;
    let response = client
        .fund_portfolio_stock_report_dates(FundType::Otc, &fund, Some(&report_type))
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
