use serde::{Deserialize, Serialize};

use super::common::{DistanceUnit, EventType, RankType};

// GET /tournament/formats

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct TournamentFormatsResponse {
    pub qualifying_formats: Vec<Format>,
    pub finals_formats: Vec<Format>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Format {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub format_id: i64,
    pub name: String,
    pub description: String,
}

// GET /tournament/{id}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Tournament {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub tournament_id: i64,
    pub tournament_name: String,
    pub tournament_type: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_bool", default)]
    pub private_flag: bool,
    pub address1: String,
    pub address2: String,
    pub city: String,
    pub stateprov: String,
    pub postal_code: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub latitude: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub longitude: f64,
    pub country_name: String,
    pub country_code: String,
    pub raw_address: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub director_id: i64,
    pub director_name: String,
    pub website: String,
    pub profile_photo: Option<String>,
    pub event_name: String,
    pub event_start_date: String,
    pub event_end_date: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub ratings_strength: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub rankings_strength: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub base_value: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub tournament_percentage_grade: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub tournament_value: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_bool", default)]
    pub qualify_flag: bool,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub qualify_hours: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_bool", default)]
    pub unlimited_qualify_flag: bool,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub eligible_player_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_count: i64,
    pub ranking_system: String,
    pub details: String,
    pub qualifying_format: String,
    pub finals_format: String,
    #[serde(
        deserialize_with = "crate::serde_util::optional_string_or_i64",
        default
    )]
    pub player_limit: Option<i64>,
    pub registration_date: Option<String>,
    #[serde(
        deserialize_with = "crate::serde_util::optional_string_or_i64",
        default
    )]
    pub matchplay_id: Option<i64>,
}

// GET /tournament/{id}/related

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct RelatedTournamentsResponse {
    pub tournament: Vec<RelatedTournament>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct RelatedTournament {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub tournament_id: i64,
    pub tournament_name: String,
    pub tournament_type: String,
    pub event_name: String,
    pub event_start_date: String,
    pub event_end_date: String,
    pub ranking_system: String,
    pub winner: Option<RelatedTournamentWinner>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct RelatedTournamentWinner {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_id: i64,
    pub name: String,
    pub country_name: String,
    pub country_code: String,
}

// GET /tournament/{id}/results

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct TournamentResultsResponse {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub tournament_id: i64,
    pub ranking_system: String,
    pub results: Vec<TournamentResult>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct TournamentResult {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_id: i64,
    pub name: String,
    pub profile_photo: Option<String>,
    pub country_name: String,
    pub country_code: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub position: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub points: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub wppr_rank: i64,
    pub ratings_value: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_bool", default)]
    pub excluded_flag: bool,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_tournament_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub wppr_pro_rank: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub efficiency_value: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub post_rank_pos: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub post_rating_value: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub post_efficiency_value: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub post_wppr_pro_rank: i64,
}

// GET /tournament/leagues/{time_period}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct LeaguesResponse {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub total_entries: i64,
    pub status: String,
    pub results: Vec<LeagueEntry>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct LeagueEntry {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub tournament_id: i64,
    pub tournament_name: String,
    pub event_name: String,
    pub city: String,
    pub stateprov: String,
    pub country_name: String,
    pub country_code: String,
    pub latitude: String,
    pub longitude: String,
    pub event_start_date: String,
    pub event_end_date: serde_json::Value,
    #[serde(deserialize_with = "crate::serde_util::string_or_bool", default)]
    pub private_flag: bool,
    pub director_name: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub director_id: i64,
}

// GET /tournament/search

#[derive(Debug, Clone, Default, Serialize)]
pub struct TournamentSearchParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateprov: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_type: Option<RankType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<EventType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(
        deserialize_with = "crate::serde_util::optional_string_or_i64",
        default
    )]
    pub start_pos: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(
        deserialize_with = "crate::serde_util::optional_string_or_i64",
        default
    )]
    pub total: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub director_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_registration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_with_results: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_unit: Option<DistanceUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_points: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_points: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_filter: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct TournamentSearchResponse {
    pub search_filter: TournamentSearchFilter,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub total_results: i64,
    pub tournaments: Vec<TournamentSearchResult>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct TournamentSearchFilter {
    pub name: Option<String>,
    pub country: Option<String>,
    pub stateprov: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub rank_type: Option<String>,
    pub sort_mode: Option<String>,
    pub sort_order: Option<String>,
    pub director_name: Option<String>,
    pub event_type: Option<String>,
    pub distance_unit: Option<serde_json::Value>,
    pub radius: Option<serde_json::Value>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct TournamentSearchResult {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub tournament_id: i64,
    pub tournament_name: String,
    pub event_name: String,
    pub event_type: String,
    pub address1: String,
    pub city: String,
    pub stateprov: String,
    pub country_name: String,
    pub country_code: String,
    pub event_start_date: String,
    pub event_end_date: String,
    pub latitude: String,
    pub longitude: String,
    pub raw_address: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_bool", default)]
    pub private_flag: bool,
    pub ranking_system: String,
    pub preregistration_date: Option<String>,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_count: i64,
    pub qualifying_format: String,
    pub finals_format: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub director_id: i64,
    pub director_name: String,
    pub website: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_bool", default)]
    pub certified_flag: bool,
    pub winner: Option<SearchWinner>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct SearchWinner {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_id: i64,
    pub player_name: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub wppr_points: f64,
    pub profile_photo: Option<String>,
}
