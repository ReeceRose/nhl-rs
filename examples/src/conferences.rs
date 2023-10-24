use nhl_rs::conferences::ConferencesResponse;
use nhl_rs::Client;

#[tokio::main]
async fn main() {
    let mut client = Client::new();

    let response: Result<ConferencesResponse, u16> = client.confrences.get_by_id(1).send().await;
    let response = response.unwrap();

    println!("Confrence with the ID of 1");
    println!("{:?}", response.conferences[0].name);

    let response = client.confrences.all().send().await;
    let response = response.unwrap();

    println!(
        "The NHL has {} total confrences",
        response.conferences.len()
    );
}
