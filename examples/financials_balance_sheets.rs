use financial_api::{Client, Error, FinancialPeriod, FinancialRange};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::from_env()?;
    let range = FinancialRange::recent(4)?;
    let response = client
        .financials_balance_sheets("600519.SH", FinancialPeriod::Annual, range)
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
