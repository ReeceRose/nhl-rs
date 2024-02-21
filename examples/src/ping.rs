use std::time::Instant;

use nhl_rs::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), u16> {
    let now = Instant::now();

    let client = ClientBuilder::new().build();

    let response = client.ping().await?;

    println!("Request took {}ms", now.elapsed().as_millis());

    println!("Ping result: {}", response.success);

    Ok(())
}
