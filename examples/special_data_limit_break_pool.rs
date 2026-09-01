use financial_api::{Client, Error, LimitBreakSortField, Page, SortDirection};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::from_env()?;
    let response = client
        .special_data_limit_break_pool(
            None,
            Page::default(),
            LimitBreakSortField::LastPrice,
            SortDirection::Descending,
        )
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
