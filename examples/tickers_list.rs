use financial_api::{AssetType, Client, Error, TickerListRequest};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::from_env()?;
    let request = TickerListRequest::new().asset_types([AssetType::AShare]);
    let response = client.tickers_list(&request).await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
