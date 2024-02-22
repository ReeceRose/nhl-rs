use std::time::Instant;

use nhl_rs::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), u16> {
    let now = Instant::now();

    let client = ClientBuilder::new().build();

    let response = client.get_games().await?;

    println!("Request took {}ms", now.elapsed().as_millis());

    println!("Got {:?} NHL games", response.len());

    Ok(())
}
