use crate::{http::get, Client};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountriesResponse {
    pub data: Vec<Country>,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    pub id: String,
    #[serde(rename = "country3Code")]
    pub country3code: String,
    pub country_code: String,
    pub country_name: String,
    pub has_player_stats: i64,
    pub image_url: Option<String>,
    pub ioc_code: String,
    pub is_active: i64,
    pub nationality_name: String,
    pub olympic_url: Option<String>,
    pub thumbnail_url: Option<String>,
}

impl Client {
    /// Returns a list of countries assoicated with the NHL using the endpoint <https://api.nhle.com/stats/rest/{language_code}/country>.
    ///
    /// # Example
    /// ```no_run
    /// use nhl_rs::ClientBuilder;
    ///
    /// const LANGUAGE_CODE: &str = "en";
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), u16> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let response = client.get_countries().await?;
    ///
    /// println!("Got {:?} associated countries", response.total);
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn get_countries(&self) -> Result<CountriesResponse, u16> {
        let url = format!("{}/{}/country", self.stats_base_url, self.language);
        get::<CountriesResponse>(url).await
    }
}
