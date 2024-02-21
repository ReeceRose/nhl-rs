use nhl_rs::ClientBuilder;

const LANGUAGE_CODE: &str = "fr";

#[tokio::main]
async fn main() -> Result<(), u16> {
    let client = ClientBuilder::new()
        .language(LANGUAGE_CODE.to_string())
        .build();
    let response = client.get_glossary().await?;

    println!("Got {:?} glossary terms", response.len());
    println!("First term {:?}", response[0].definition);

    Ok(())
}
