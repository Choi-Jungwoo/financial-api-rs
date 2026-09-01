use financial_api::{Client, Error, FundType};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::from_env()?;
    let response = client
        .fund_news_article_list(FundType::Otc, "025480.OF", Some(20), None)
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
