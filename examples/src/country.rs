use std::time::Instant;

use nhl_rs::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), u16> {
    let now = Instant::now();

    let client = ClientBuilder::new().build();

    let response = client.get_countries().await?;

    println!("Request took {}ms", now.elapsed().as_millis());

    println!("Got {:?} associated countries", response.len());

    let canada = client.get_country_by_id("CAN").await?.unwrap();

    println!("Canada thumbnail URL {:?}", canada.thumbnail_url);

    let usa = client.get_country_by_name("united states").await?.unwrap();

    println!("USA thumbnail URL {:?}", usa.thumbnail_url);

    let active = client.get_active_countries().await?;

    println!("There are currently {} active countries", active.len());

    let inactive = client.get_inactive_countries().await?;

    println!("There are currently {} inactive countries", inactive.len());

    Ok(())
}
