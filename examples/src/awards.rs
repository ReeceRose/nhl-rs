use nhl_rs::awards::AwardsResponse;
use nhl_rs::Client;

#[tokio::main]
async fn main() {
    let mut client = Client::new();

    let response: Result<AwardsResponse, u16> = client.awards.get_by_id(1).send().await;
    let response = response.unwrap();

    println!("Awards with the ID of 1");
    println!("{:?}", response.awards[0]);

    let response = client.awards.all().send().await;
    let response = response.unwrap();
    // response
    println!("The NHL has {} total awards", response.awards.len());
}
