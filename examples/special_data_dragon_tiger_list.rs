use financial_api::{Client, DragonTigerBoard, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::from_env()?;
    let response = client
        .special_data_dragon_tiger_list(DragonTigerBoard::All, Some("2026-08-25"))
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
