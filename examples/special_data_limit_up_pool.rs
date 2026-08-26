include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/configuration/client.rs"
));

use financial_api::{LimitUpSortField, Page, SortDirection};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = from_env()?;
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
