use crate::{http::get, Client};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlossaryResponse {
    pub data: Vec<GlossaryItem>,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlossaryItem {
    pub id: i64,
    pub abbreviation: Option<String>,
    pub definition: String,
    pub first_season_for_stat: Option<i64>,
    pub full_name: String,
    pub language_code: String,
    pub last_updated: String,
}

impl Client {
    /// Retrieve a list of glossary terms.
    ///
    /// # Example
    /// ```no_run
    /// use nhl_rs::ClientBuilder;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), u16> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let response = client.get_glossary().await?;
    ///
    /// println!("Got {:?} glossary terms", response.len());
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn get_glossary(&self) -> Result<Vec<GlossaryItem>, u16> {
        let url = format!("{}/{}/glossary", self.stats_base_url, self.language);
        let result = get::<GlossaryResponse>(url).await;
        match result {
            Ok(response) => Ok(response.data),
            Err(status_code) => Err(status_code),
        }
    }

    /// Retrieve a glossary item by a given `glossary_id`.
    ///
    /// # Errors
    /// If the NHL API throws an error, then the corresponding HTTP error code is returned.
    ///
    /// # Example
    /// ```no_run
    /// use nhl_rs::ClientBuilder;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), u16> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let response = client.get_glossary_item_by_id(1000).await?.unwrap();
    ///
    /// println!("Glossary item {:?}", response);
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn get_glossary_item_by_id(
        &self,
        glossary_id: i64,
    ) -> Result<Option<GlossaryItem>, u16> {
        let result = self.get_glossary().await?;
        Ok(result
            .into_iter()
            .filter(|item| item.id == glossary_id)
            .collect::<Vec<_>>()
            .first()
            .cloned())
    }

    /// Retrieve a glossary item by a given `glossary_abbreviation`.
    ///
    /// # Errors
    /// If the NHL API throws an error, then the corresponding HTTP error code is returned.
    ///
    /// # Example
    /// ```no_run
    /// use nhl_rs::ClientBuilder;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), u16> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let response = client.get_glossary_item_by_abbreviation("GR W").await?.unwrap();
    ///
    /// println!("Glossary item {:?}", response);
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn get_glossary_item_by_abbreviation(
        &self,
        glossary_abbreviation: &str,
    ) -> Result<Option<GlossaryItem>, u16> {
        let result = self.get_glossary().await?;
        let abbreviation = glossary_abbreviation.to_uppercase();
        Ok(result
            .into_iter()
            .filter(|item| {
                item.abbreviation
                    .clone()
                    .unwrap_or(String::from(""))
                    .to_uppercase()
                    == abbreviation
            })
            .collect::<Vec<_>>()
            .first()
            .cloned())
    }
}
