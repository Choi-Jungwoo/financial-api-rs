#[path = "configuration/client.rs"]
mod example_client;

use financial_api::{LimitUpSortField, Page, SortDirection};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = example_client::from_env()?;
    let response = client
        .special_data_limit_up_pool(
            None,
            Page::default(),
            LimitUpSortField::LastPrice,
            SortDirection::Descending,
        )
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
