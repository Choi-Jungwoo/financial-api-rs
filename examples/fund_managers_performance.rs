use financial_api::{Client, Error, ManagerPerformanceRange};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::from_env()?;
    let response = client
        .fund_managers_performance("H002417139", ManagerPerformanceRange::Month)
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
