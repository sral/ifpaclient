use serde::{Deserialize, Serialize};

// GET /series/list

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct SeriesListResponse {
    pub series: Vec<Series>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Series {
    pub code: String,
    pub title: String,
    pub years: Vec<String>,
}

// GET /series/{series_code}/regions

#[derive(Debug, Clone, Default, Serialize)]
pub struct SeriesYearParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(
        deserialize_with = "crate::serde_util::optional_string_or_i64",
        default
    )]
    pub year: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct SeriesRegionsResponse {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub year: i64,
    pub series_code: String,
    pub active_regions: Vec<ActiveRegion>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct ActiveRegion {
    pub region_name: String,
    pub region_code: String,
}

// GET /series/{series_code}/player_card/{player_id}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PlayerCardParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(
        deserialize_with = "crate::serde_util::optional_string_or_i64",
        default
    )]
    pub year: Option<i64>,
    pub region_code: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PlayerCardResponse {
    pub series_code: serde_json::Value,
    pub region_code: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub year: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_id: i64,
    pub player_name: String,
    pub player_card: Vec<PlayerCardEntry>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PlayerCardEntry {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub tournament_id: i64,
    pub tournament_name: String,
    pub event_name: String,
    pub event_end_date: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub wppr_points: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub region_event_rank: i64,
}

// GET /series/{series_code}/region_reps

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct RegionRepsResponse {
    pub series_code: serde_json::Value,
    pub representative: Vec<Representative>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Representative {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_id: i64,
    pub name: String,
    pub region_code: String,
    pub region_name: String,
    pub profile_photo: Option<String>,
}

// GET /series/{series_code}/overall_standings

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct OverallStandingsResponse {
    pub series_code: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub year: i64,
    pub overall_results: Vec<OverallResult>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct OverallResult {
    pub region_code: String,
    pub region_name: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub unique_player_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub tournament_count: i64,
    pub current_leader: Option<CurrentLeader>,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub prize_fund: f64,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct CurrentLeader {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_id: i64,
    pub player_name: String,
}

// GET /series/{series_code}/standings

#[derive(Debug, Clone, Default, Serialize)]
pub struct SeriesRegionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(
        deserialize_with = "crate::serde_util::optional_string_or_i64",
        default
    )]
    pub year: Option<i64>,
    pub region_code: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct StandingsResponse {
    pub series_code: String,
    pub region_code: String,
    pub region_name: Option<String>,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub prize_fund: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub year: i64,
    pub standings: Vec<Standing>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Standing {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub series_rank: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_id: i64,
    pub player_name: String,
    pub city: String,
    pub stateprov_code: String,
    pub country_code: String,
    pub country_name: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub wppr_points: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub event_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub win_count: i64,
}

// GET /series/{series_code}/stats

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct SeriesStatsResponse {
    pub series_code: String,
    pub region_code: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub year: i64,
    pub monthly_stats: Vec<MonthlyStat>,
    pub yearly_stats: Option<YearlyStat>,
    pub payouts: Vec<Payout>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct MonthlyStat {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub month: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub tournament_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub unique_player_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub prize_fund: f64,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct YearlyStat {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub unique_player_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub tournament_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub prize_fund: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub field_size: f64,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Payout {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub position: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub prize_fund: f64,
}

// GET /series/{series_code}/tournaments

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct SeriesTournamentsResponse {
    pub series_code: String,
    pub region_code: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub year: i64,
    pub submitted_tournaments: Vec<SubmittedTournament>,
    pub unsubmitted_tournaments: Vec<UnsubmittedTournament>,
    pub future_tournament: Vec<FutureTournament>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct SubmittedTournament {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub tournament_id: i64,
    pub event_end_date: String,
    pub tournament_name: String,
    pub event_name: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub wppr_points: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_count: i64,
    pub winner: Option<TournamentWinner>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct TournamentWinner {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct UnsubmittedTournament {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub tournament_id: i64,
    pub event_end_date: String,
    pub tournament_name: String,
    pub event_name: String,
    pub status: String,
    pub director_name: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct FutureTournament {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub tournament_id: i64,
    pub event_start_date: String,
    pub event_end_date: String,
    pub tournament_name: String,
    pub event_name: String,
    pub status: Option<String>,
    pub director_name: String,
    pub city: String,
}
