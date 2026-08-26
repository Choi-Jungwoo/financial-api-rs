#[path = "configuration/client.rs"]
mod example_client;

use financial_api::{SearchQuery, TickerSearchRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = example_client::from_env()?;
    let request = TickerSearchRequest::new(SearchQuery::new("贵州茅台")?);
    let response = client.tickers_search(&request).await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
