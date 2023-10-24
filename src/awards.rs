use serde::{Deserialize, Serialize};

use crate::http::get;

#[derive(Default, Debug)]
pub struct AwardsClient {
    id: Option<i32>,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AwardsResponse {
    pub copyright: String,
    pub awards: Vec<Award>,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Award {
    pub name: String,
    pub short_name: String,
    pub description: String,
    pub recipient_type: String,
    pub history: Option<String>,
    pub image_url: String,
    pub home_page_url: String,
    pub link: String,
}

impl AwardsClient {
    pub fn new() -> AwardsClient {
        AwardsClient { id: None }
    }

    /// Get all NHL awards.
    pub fn all(&mut self) -> &mut Self {
        self.id = None;
        self
    }

    /// Get an NHL award by ID.
    pub fn get_by_id(&mut self, id: i32) -> &mut Self {
        self.id = Some(id);
        self
    }

    pub async fn send(&self) -> Result<AwardsResponse, u16> {
        let mut append = "".to_string();
        if let Some(award_id) = self.id {
            append = award_id.to_string();
        }

        let url = format!("https://statsapi.web.nhl.com/api/v1/awards/{}", append);
        get::<AwardsResponse>(url).await
    }
}
