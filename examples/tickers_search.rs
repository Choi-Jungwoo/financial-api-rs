use financial_api::{Client, Error, TickerSearchRequest};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::from_env()?;
    let request = TickerSearchRequest::new("贵州茅台")?;
    let response = client.tickers_search(&request).await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
