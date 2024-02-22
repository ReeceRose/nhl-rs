use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameResponse {
    pub data: Vec<Game>,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub id: i64,
    pub eastern_start_time: String,
    pub game_date: String,
    pub game_number: i64,
    pub game_schedule_state_id: i64,
    pub game_state_id: i64,
    pub game_type: i64,
    pub home_score: i64,
    pub home_team_id: i64,
    pub period: Option<i64>,
    pub season: i64,
    pub visiting_score: i64,
    pub visiting_team_id: i64,
}
