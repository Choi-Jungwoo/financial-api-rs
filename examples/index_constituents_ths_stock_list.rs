#[path = "configuration/client.rs"]
mod example_client;

use financial_api::Thscode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = example_client::from_env()?;
    let index = Thscode::new("000300.SH")?;
    let response = client.index_constituents_ths_stock_list(&index).await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
