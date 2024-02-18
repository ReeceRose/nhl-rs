use std::time::Instant;

use nhl_rs::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), u16> {
    let now = Instant::now();

    let client = ClientBuilder::new().build();

    let response = client.get_franchises().await?;

    println!("Request took {}ms", now.elapsed().as_millis());

    println!("Franchise with the ID of 1");
    println!("{:?}", response.data[0].full_name);

    println!("The NHL has {} total confrences", response.total);

    Ok(())
}
