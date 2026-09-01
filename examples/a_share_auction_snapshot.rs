use financial_api::{AuctionStage, Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::from_env()?;
    let response = client
        .a_share_auction_snapshot(["600519.SH"], AuctionStage::Final)
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
