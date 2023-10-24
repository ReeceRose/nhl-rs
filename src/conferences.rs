use serde::{Deserialize, Serialize};

use crate::http::get;

#[derive(Default, Debug)]
pub struct ConferencesClient {
    id: Option<i32>,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConferencesResponse {
    pub copyright: String,
    pub conferences: Vec<Conference>,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Conference {
    pub id: i64,
    pub name: String,
    pub link: String,
    pub abbreviation: String,
    pub short_name: String,
    pub active: bool,
}

impl ConferencesClient {
    pub fn new() -> ConferencesClient {
        ConferencesClient { id: None }
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

    pub async fn send(&self) -> Result<ConferencesResponse, u16> {
        let mut append = "".to_string();
        if let Some(award_id) = self.id {
            append = award_id.to_string();
        }

        let url = format!("https://statsapi.web.nhl.com/api/v1/conferences/{}", append);
        get::<ConferencesResponse>(url).await
    }
}
