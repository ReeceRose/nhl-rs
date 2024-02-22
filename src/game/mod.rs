pub mod game;
pub mod metadata;

use crate::{http::get, Client};

use crate::game::{
    game::{Game, GameResponse},
    metadata::GameMetadataResponse,
};

impl Client {
    /// Retrieve a metadata struct for games.
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

    /// Get all games in NHL existance (both played and scheduled). Contains an entry for each period.
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
}
