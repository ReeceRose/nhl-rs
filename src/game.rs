use crate::{http::get, Client};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameMetadataResponse {
    pub entity: Entity,
    pub links: Vec<Link>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub name: String,
    pub properties: Vec<Property>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub format: Option<String>,
    pub relationship: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub href: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub operations: Vec<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    pub method: String,
}

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
}
