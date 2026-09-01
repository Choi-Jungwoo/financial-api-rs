use financial_api::{Client, Error, FundType, HolderMergeScope};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::from_env()?;
    let response = client
        .fund_holders_detail(FundType::Otc, "000037.OF", HolderMergeScope::All)
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
