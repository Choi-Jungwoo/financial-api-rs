use financial_api::{Client, Error, FundType};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::from_env()?;
    let response = client
        .fund_portfolio_stock_history(FundType::Otc, "025480.OF", "quarter", "2026-06-30")
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
