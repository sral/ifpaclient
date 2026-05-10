use serde::{Deserialize, Serialize};

// GET /player and GET /player/{id}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PlayersResponse {
    pub player: Vec<Player>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PlayerResponse {
    pub player: Vec<Player>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Player {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_id: i64,
    pub first_name: String,
    pub last_name: String,
    pub initials: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_bool", default)]
    pub excluded_flag: bool,
    #[serde(
        deserialize_with = "crate::serde_util::optional_string_or_i64",
        default
    )]
    pub age: Option<i64>,
    pub city: String,
    pub stateprov: String,
    pub country_name: String,
    pub country_code: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_bool", default)]
    pub ifpa_registered: bool,
    #[serde(deserialize_with = "crate::serde_util::string_or_bool", default)]
    pub womens_flag: bool,
    pub profile_photo: Option<String>,
    pub matchplay_events: Option<MatchplayEvents>,
    pub twitch_username: Option<String>,
    pub pinside_username: Option<String>,
    pub player_stats: Option<PlayerStats>,
    pub series: Vec<PlayerSeries>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct MatchplayEvents {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub id: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub rating: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub rank: i64,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PlayerStats {
    pub system: Option<PlayerStatsSystems>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PlayerStatsSystems {
    #[serde(rename = "MAIN")]
    pub main: Option<SystemStats>,
    #[serde(rename = "WOMEN")]
    pub women: Option<SystemStats>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct SystemStats {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub current_rank: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub last_month_rank: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub last_year_rank: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub highest_rank: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub current_points: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub all_time_points: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub active_points: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub inactive_points: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub best_finish: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub best_finish_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub average_finish: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub average_finish_last_year: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub total_events_all_time: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub total_active_events: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub total_events_away: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub total_wins_last_3_years: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub top_3_last_3_years: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub top_10_last_3_years: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub ratings_rank: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub ratings_value: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub efficiency_rank: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub efficiency_value: f64,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PlayerSeries {
    pub series_code: String,
    pub region_code: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub year: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub total_points: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub series_rank: i64,
}

// GET /player/{id}/pvp

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PlayerPvpResponse {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_id: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub total_competitors: i64,
    pub system: String,
    pub pvp: Vec<PvpEntry>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PvpEntry {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_id: i64,
    pub first_name: String,
    pub last_name: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub win_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub loss_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub tie_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub current_rank: i64,
}

// GET /player/{id}/pvp/{id2}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PvpComparisonResponse {
    pub player_1: PvpPlayer,
    pub player_2: PvpPlayer,
    pub pvp: Vec<PvpEvent>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PvpPlayer {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_id: i64,
    pub first_name: String,
    pub last_name: String,
    pub country_code: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_bool", default)]
    pub excluded_flag: bool,
    pub profile_photo: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PvpEvent {
    pub tournament_name: String,
    pub event_name: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub tournament_id: i64,
    pub event_end_date: String,
    pub country_code: String,
    pub finish_position: FinishPosition,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct FinishPosition {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_1: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_2: i64,
}

// GET /player/{id}/rank_history

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct RankHistoryResponse {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_id: i64,
    pub system: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_bool", default)]
    pub active_flag: bool,
    pub rank_history: Vec<RankHistoryEntry>,
    pub rating_history: Vec<RatingHistoryEntry>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct RankHistoryEntry {
    pub rank_date: String,
    #[serde(
        alias = "rank_postiion",
        deserialize_with = "crate::serde_util::string_or_i64",
        default
    )]
    pub rank_position: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub wppr_points: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub tournaments_played_count: i64,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct RatingHistoryEntry {
    pub rating_date: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub rating: f64,
}

// GET /player/{id}/results/{ranking_system}/{type}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PlayerResultsResponse {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_id: i64,
    pub results_type: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub results_count: i64,
    pub system: String,
    pub results: Vec<PlayerResult>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PlayerResult {
    pub tournament_name: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub tournament_id: i64,
    pub event_name: String,
    pub event_date: String,
    pub country_name: String,
    pub country_code: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub position: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub original_points: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub current_points: f64,
}

// GET /player/search

#[derive(Debug, Clone, Default, Serialize)]
pub struct PlayerSearchParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateprov: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tournament: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(
        deserialize_with = "crate::serde_util::optional_string_or_i64",
        default
    )]
    pub tourpos: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PlayerSearchResponse {
    pub search_filter: PlayerSearchFilter,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub total_results: i64,
    pub results: Vec<PlayerSearchResult>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PlayerSearchFilter {
    pub name: Option<String>,
    pub country: Option<String>,
    pub stateprov: Option<String>,
    pub tournament: Option<String>,
    #[serde(
        deserialize_with = "crate::serde_util::optional_string_or_i64",
        default
    )]
    pub tournament_position: Option<i64>,
    pub include_unranked: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PlayerSearchResult {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_id: i64,
    pub first_name: String,
    pub last_name: String,
    pub country_name: String,
    pub country_code: String,
    pub city: String,
    pub stateprov: String,
    pub wppr_rank: String,
    pub rating_value: String,
    pub profile_photo_url: Option<String>,
}
