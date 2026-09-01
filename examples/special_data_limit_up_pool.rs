use financial_api::{Client, Error, LimitUpSortField, Page, SortDirection};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::from_env()?;
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
