pub mod component_season;

use crate::{http::get, Client};

pub use crate::season::component_season::{ComponentSeason, ComponentSeasonResponse};

impl Client {
    /// Get component season information.
    ///
    /// # Example
    /// ```no_run
    /// use nhl_rs::ClientBuilder;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), u16> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let response = client.get_component_season().await?;
    ///
    /// println!("Current season ID {}", response[0].season_id);
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn get_component_season(&self) -> Result<Vec<ComponentSeason>, u16> {
        let url = format!("{}/{}/componentSeason", self.stats_base_url, self.language);
        let result = get::<ComponentSeasonResponse>(url).await;

        match result {
            Ok(response) => Ok(response.data),
            Err(status_code) => Err(status_code),
        }
    }
}
