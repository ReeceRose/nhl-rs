use std::time::Instant;

use nhl_rs::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), u16> {
    let now = Instant::now();

    let client = ClientBuilder::new().build();

    let response = client.get_glossary().await?;

    println!("Request took {}ms", now.elapsed().as_millis());

    println!("Got {:?} glossary terms", response.len());

    let response = client.get_glossary_item_by_id(1315).await?.unwrap();

    println!("Glossary item with ID of 1315: {:?}", response);

    let response = client
        .get_glossary_item_by_abbreviation("GR W")
        .await?
        .unwrap();

    println!("Glossary item with abbreviation 'GR W': {:?}", response);

    Ok(())
}
