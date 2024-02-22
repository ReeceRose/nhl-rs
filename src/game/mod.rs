pub mod game;
pub mod metadata;

use crate::{http::get, Client};

pub use crate::game::{
    game::{Game, GameResponse},
    metadata::GameMetadataResponse,
};

impl Client {
    /// Get a metadata struct for games.
    ///
    /// # Example
    /// ```no_run
    /// use nhl_rs::ClientBuilder;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), u16> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let response = client.get_game_metadata().await?;
    ///
    /// println!("Got {:?} metadata properties", response.entity.properties.len());
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn get_game_metadata(&self) -> Result<GameMetadataResponse, u16> {
        let url = format!("{}/{}/game/meta", self.stats_base_url, self.language);
        get::<GameMetadataResponse>(url).await
    }

    /// Get most games in existance (both played and scheduled).
    ///
    /// Note, `game_number` is reset multiple times per season.
    /// Querying based off of that may result in multiple games.
    ///
    /// # Example
    /// ```no_run
    /// use nhl_rs::ClientBuilder;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), u16> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let response = client.get_games().await?;
    ///
    /// println!("Got {:?} NHL games", response.len());
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn get_games(&self) -> Result<Vec<Game>, u16> {
        let url = format!("{}/{}/game", self.stats_base_url, self.language);
        let result = get::<GameResponse>(url).await;
        match result {
            Ok(response) => Ok(response.data),
            Err(status_code) => Err(status_code),
        }
    }

    /// Get a game by the `id`.
    ///
    /// # Example
    /// ```no_run
    /// use nhl_rs::ClientBuilder;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), u16> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let response = client.get_game_by_id(2017020120).await?;
    ///
    /// println!("Game with id 2017020120: {:?}", response);
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn get_game_by_id(&self, id: i64) -> Result<Option<Game>, u16> {
        let result = self.get_games().await?;
        Ok(result
            .into_iter()
            .filter(|game| game.id == id)
            .collect::<Vec<_>>()
            .first()
            .cloned())
    }

    /// Get all games for a team by their team `id`.
    ///
    /// # Example
    /// ```no_run
    /// use nhl_rs::ClientBuilder;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), u16> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let response = client.get_games_for_team_by_team_id(12).await?;
    ///
    /// println!("Team with id of 12 has: {} games", response.len());
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn get_games_for_team_by_team_id(&self, id: i64) -> Result<Vec<Game>, u16> {
        let result = self.get_games().await?;
        Ok(result
            .into_iter()
            .filter(|game| game.home_team_id == id || game.visiting_team_id == id)
            .collect::<Vec<_>>())
    }

    /// Get all games for a season by the season `id`.
    ///
    /// # Example
    /// ```no_run
    /// use nhl_rs::ClientBuilder;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), u16> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let response = client.get_games_for_season_by_id(20222023).await?;
    ///
    /// println!("2022/2023 had: {} games", response.len());
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn get_games_for_season_by_id(&self, id: i64) -> Result<Vec<Game>, u16> {
        let result = self.get_games().await?;
        Ok(result
            .into_iter()
            .filter(|game| game.season == id)
            .collect::<Vec<_>>())
    }
}
