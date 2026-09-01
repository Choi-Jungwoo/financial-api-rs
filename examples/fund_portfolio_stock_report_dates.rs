use financial_api::{Client, Error, FundType};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::from_env()?;
    let response = client
        .fund_portfolio_stock_report_dates(FundType::Otc, "025480.OF", "quarter")
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
