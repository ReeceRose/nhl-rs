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
    /// Get a list of franchises.
    ///
    /// # Errors
    /// If the NHL API throws an error, then the corresponding HTTP error code is returned.
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), u16> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let response = client.get_franchises().await?;
    ///
    /// println!("Franchise with the ID of 1");
    /// println!("{:?}", response[0].full_name);
    ///
    /// println!("The NHL has {} total confrences", response.total);
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn get_franchises(&self) -> Result<Vec<Franchise>, u16> {
        let url = format!("{}/{}/franchise", self.stats_base_url, self.language);
        let result = get::<FranchiseResponse>(url).await;
        match result {
            Ok(response) => Ok(response.data),
            Err(status_code) => Err(status_code),
        }
    }

    /// Get a franchise by an `id`.
    ///
    /// # Errors
    /// If the NHL API throws an error, then the corresponding HTTP error code is returned.
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), u16> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let response = client.get_franchise_by_id(12).await?;
    ///
    /// println!("Franchise with the id of 12: {:?}", response);
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn get_franchise_by_id(&self, id: i64) -> Result<Option<Franchise>, u16> {
        let result = self.get_franchises().await?;
        Ok(result
            .into_iter()
            .filter(|franchise| franchise.id == id)
            .collect::<Vec<_>>()
            .first()
            .cloned())
    }

    /// Get a franchise by a `full_name`.
    ///
    /// # Errors
    /// If the NHL API throws an error, then the corresponding HTTP error code is returned.
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), u16> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let response = client.get_franchise_by_full_name("St. Louis Eagles".to_string()).await?;
    ///
    /// println!("Franchise with the name of 'St. Louis': {:?}", response);
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn get_franchise_by_full_name(
        &self,
        full_name: &str,
    ) -> Result<Option<Franchise>, u16> {
        let result = self.get_franchises().await?;
        let name = full_name.to_uppercase();
        Ok(result
            .into_iter()
            .filter(|franchise| franchise.full_name.to_uppercase() == name)
            .collect::<Vec<_>>()
            .first()
            .cloned())
    }

    /// Get a franchise by a `team_common_name`.
    ///
    /// # Errors
    /// If the NHL API throws an error, then the corresponding HTTP error code is returned.
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), u16> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let response = client.get_franchise_by_team_common_name("Eagles".to_string()).await?;
    ///
    /// println!("Franchise with the common name of 'Eagles': {:?}", response);
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn get_franchise_by_team_common_name(
        &self,
        team_common_name: &str,
    ) -> Result<Option<Franchise>, u16> {
        let result = self.get_franchises().await?;
        let name = team_common_name.to_uppercase();
        Ok(result
            .into_iter()
            .filter(|franchise| franchise.team_common_name.to_uppercase() == name)
            .collect::<Vec<_>>()
            .first()
            .cloned())
    }

    /// Get a franchise by a `team_place_name`.
    ///
    /// # Errors
    /// If the NHL API throws an error, then the corresponding HTTP error code is returned.
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), u16> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let response = client.get_franchise_by_team_place_name("Hamilton".to_string()).await?;
    ///
    /// println!("Franchise with the place name of 'Hamilton': {:?}", response);
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn get_franchise_by_team_place_name(
        &self,
        team_place_name: &str,
    ) -> Result<Option<Franchise>, u16> {
        let result = self.get_franchises().await?;
        let name = team_place_name.to_uppercase();
        Ok(result
            .into_iter()
            .filter(|franchise| franchise.team_place_name.to_uppercase() == name)
            .collect::<Vec<_>>()
            .first()
            .cloned())
    }
}
