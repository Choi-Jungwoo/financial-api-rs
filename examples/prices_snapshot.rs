use financial_api::{Client, Error, PriceSnapshotSelection};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::from_env()?;
    let selection = PriceSnapshotSelection::targets(["600519.SH"])?;
    let response = client.prices_snapshot(&selection).await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
