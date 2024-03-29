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
    /// Get a list of countries assoicated with the NHL.
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
    /// let response = client.get_countries().await?;
    ///
    /// println!("Got {:?} associated countries", response.len());
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn get_countries(&self) -> Result<Vec<Country>, u16> {
        let url = format!("{}/{}/country", self.stats_base_url, self.language);
        let result = get::<CountriesResponse>(url).await;
        match result {
            Ok(response) => Ok(response.data),
            Err(status_code) => Err(status_code),
        }
    }

    /// Get a country by an `id`.
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
    /// let response = client.get_country_by_id("CAN").await?.unwrap();
    ///
    /// println!("Thumbnail URL {:?}", response.thumbnail_url);
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn get_country_by_id(&self, id: &str) -> Result<Option<Country>, u16> {
        let result = self.get_countries().await?;
        let country_id = id.to_uppercase();
        Ok(result
            .into_iter()
            .filter(|country| country.id.to_uppercase() == country_id)
            .collect::<Vec<_>>()
            .first()
            .cloned())
    }

    /// Get a country by a `country_name`.
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
    /// let response = client.get_country_by_country_name("canada").await?.unwrap();
    ///
    /// println!("Thumbnail URL {:?}", response.thumbnail_url);
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn get_country_by_country_name(
        &self,
        country_name: &str,
    ) -> Result<Option<Country>, u16> {
        let result = self.get_countries().await?;
        let name = country_name.to_uppercase();
        Ok(result
            .into_iter()
            .filter(|country| country.country_name.to_uppercase() == name)
            .collect::<Vec<_>>()
            .first()
            .cloned())
    }

    /// Get a country by a `country_ioc_code`.
    ///
    /// # Errors
    /// If the NHL API throws error, then the corresponding HTTP error code is returned.
    ///
    /// # Example
    /// ```no_run
    /// use nhl_rs::ClientBuilder;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), u16> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let response = client.get_country_by_ioc_code("RSA").await?.unwrap();
    ///
    /// println!("Thumbnail URL {:?}", response.thumbnail_url);
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn get_country_by_ioc_code(
        &self,
        country_ioc_code: &str,
    ) -> Result<Option<Country>, u16> {
        let result = self.get_countries().await?;
        let ioc_code = country_ioc_code.to_uppercase();
        Ok(result
            .into_iter()
            .filter(|country| country.ioc_code.to_uppercase() == ioc_code)
            .collect::<Vec<_>>()
            .first()
            .cloned())
    }

    async fn get_counties_by_activity(&self, is_active: i64) -> Result<Vec<Country>, u16> {
        let result = self.get_countries().await?;
        Ok(result
            .into_iter()
            .filter(|country| country.is_active == is_active)
            .collect::<Vec<_>>())
    }

    /// Get all active countries.
    ///
    /// # Errors
    /// If the NHL API throws error, then the corresponding HTTP error code is returned.
    ///
    /// # Example
    /// ```no_run
    /// use nhl_rs::ClientBuilder;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), u16> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let response = client.get_active_countries().await?;
    ///
    /// println!("There are currently {:?} active countries", response.len());
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn get_active_countries(&self) -> Result<Vec<Country>, u16> {
        self.get_counties_by_activity(1).await
    }

    /// Get all inactive countries.
    ///
    /// # Errors
    /// If the NHL API throws error, then the corresponding HTTP error code is returned.
    ///
    /// # Example
    /// ```no_run
    /// use nhl_rs::ClientBuilder;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), u16> {
    /// let client = ClientBuilder::new().build();
    ///
    /// let response = client.get_inactive_countries().await?;
    ///
    /// println!("There are currently {:?} inactive countries", response.len());
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn get_inactive_countries(&self) -> Result<Vec<Country>, u16> {
        self.get_counties_by_activity(0).await
    }
}
