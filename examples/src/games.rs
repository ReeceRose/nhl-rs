use std::time::Instant;

use nhl_rs::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), u16> {
    let now = Instant::now();

    let client = ClientBuilder::new().build();

    let result = client.get_games().await?;

    println!("Request took {}ms", now.elapsed().as_millis());

    println!("Got {:?} NHL games", result.len());

    let result = client.get_game_by_id(2017020120).await?;

    println!("Game with id 2017020120: {:?}", result);

    let result = client.get_games_for_team_by_team_id(12).await?;

    println!("Team with id of 12 has: {} games", result.len());

    let result = client.get_games_for_season_by_id(20222023).await?;

    println!("2022/2023 had: {} games", result.len());

    Ok(())
}
