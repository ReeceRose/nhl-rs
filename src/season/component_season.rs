use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentSeasonResponse {
    pub data: Vec<ComponentSeason>,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentSeason {
    pub id: String,
    pub component: String,
    pub game_type_id: i64,
    pub season_id: i64,
}
