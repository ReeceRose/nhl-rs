use std::error::Error;

use nhl_rs::Client;

#[tokio::main]
async fn main() -> Result<(), u16> {
    let mut client = Client::new();

    let response = client.confrences.get_by_id(1).send().await?;

    println!("Confrence with the ID of 1");
    println!("{:?}", response.conferences[0].name);

    let response = client.confrences.all().send().await?;

    println!(
        "The NHL has {} total confrences",
        response.conferences.len()
    );

    Ok(())
}
