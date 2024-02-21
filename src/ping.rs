use crate::{http::get, Client};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PingResponse {
    pub success: bool,
}

impl Client {
    /// Ping the NHL API.
    ///
    /// # Example
    /// ```no_run
    /// use nhl_rs::ClientBuilder;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), u16> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let response = client.ping().await?;
    ///
    /// println!("Ping result: {}", response.success);
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn ping(&self) -> Result<PingResponse, u16> {
        let url = format!("{}/ping", self.stats_base_url);
        get::<PingResponse>(url).await
    }
}
