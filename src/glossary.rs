use crate::Client;

use crate::http::get;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetGlossaryResponse {
    pub data: Vec<GlossaryItem>,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlossaryItem {
    pub id: i64,
    pub abbreviation: String,
    pub definition: String,
    pub first_season_for_stat: Option<i64>,
    pub full_name: String,
    pub language_code: String,
    pub last_updated: String,
}

impl Client {
    /// Returns a list of glossary terms using the endpoint <https://api.nhle.com/stats/rest/{language_code}/glossary>.
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
    /// let response = client.get_glossary().await?;
    ///
    /// println!("Got {:?} glossary terms", response.total);
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn get_glossary(&self) -> Result<GetGlossaryResponse, u16> {
        let url = format!("{}/{}/glossary", self.stats_base_url, self.language);
        get::<GetGlossaryResponse>(url).await
    }
}