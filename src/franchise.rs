use crate::{http::get, Client};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FranchiseResponse {
    pub data: Vec<Franchise>,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Franchise {
    pub id: i64,
    pub full_name: String,
    pub team_common_name: String,
    pub team_place_name: String,
}

impl Client {
    /// Returns a list of franchises using the endpoint <https://api.nhle.com/stats/rest/{language_code}/franchise>.
    ///
    /// # Example
    /// ```no_run
    /// use nhl_rs::ClientBuilder;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), u16> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let response = client.get_franchises().await?;
    ///
    /// println!("Franchise with the ID of 1");
    /// println!("{:?}", response.data[0].full_name);
    ///
    /// println!("The NHL has {} total confrences", response.total);
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn get_franchises(&self) -> Result<FranchiseResponse, u16> {
        let url = format!("{}/{}/franchise", self.stats_base_url, self.language);
        get::<FranchiseResponse>(url).await
    }
}
