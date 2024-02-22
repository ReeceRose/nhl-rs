use std::time::Instant;

use nhl_rs::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), u16> {
    let now = Instant::now();

    let client = ClientBuilder::new().build();

    let response = client.get_franchises().await?;

    println!("Request took {}ms", now.elapsed().as_millis());

    println!("The NHL has {} total confrences", response.len());

    let response = client.get_franchise_by_id(1).await?.unwrap();
    println!("Franchise with the ID of 1: {:?}", response);

    let response = client
        .get_franchise_by_team_place_name("Hamilton")
        .await?
        .unwrap();
    println!(
        "Franchise with the place name of 'Hamilton': {:?}",
        response
    );

    Ok(())
}
