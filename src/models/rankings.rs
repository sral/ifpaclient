use serde::{Deserialize, Serialize};

// Shared pagination params for ranking endpoints

#[derive(Debug, Clone, Default, Serialize)]
pub struct RankingsParams {
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
    pub count: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CountryRankingsParams {
    pub country: String,
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
    pub count: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CustomRankingsParams {
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
    pub count: Option<i64>,
}

// GET /rankings/wppr

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct WpprRankingsResponse {
    pub ranking_type: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub start_position: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub return_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub total_count: i64,
    pub sort_order: String,
    pub rankings: Vec<WpprRanking>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct WpprRanking {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_id: i64,
    pub name: String,
    #[serde(
        deserialize_with = "crate::serde_util::optional_string_or_i64",
        default
    )]
    pub age: Option<i64>,
    pub profile_photo: Option<String>,
    pub country_name: String,
    pub country_code: String,
    pub stateprov: String,
    pub city: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub wppr_points: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub current_rank: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub last_month_rank: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub rating_value: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub rating_deviation: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub efficiency_percent: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub event_count: i64,
    pub best_finish: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub best_finish_position: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub best_tournament_id: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub total_wins_last_3_years: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub top_3_last_3_years: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub top_10_last_3_years: i64,
}

// GET /rankings/youth

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct YouthRankingsResponse {
    pub ranking_type: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub start_position: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub return_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub total_count: i64,
    pub rankings: Vec<YouthRanking>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct YouthRanking {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_id: i64,
    pub name: String,
    #[serde(
        deserialize_with = "crate::serde_util::optional_string_or_i64",
        default
    )]
    pub age: Option<i64>,
    pub profile_photo: Option<String>,
    pub country_name: String,
    pub country_code: String,
    pub stateprov: String,
    pub city: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub wppr_points: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub current_wppr_rank: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub current_rank: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub last_month_rank: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub rating_value: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub rating_deviation: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub efficiency_percent: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub event_count: i64,
    pub best_finish: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub best_finish_position: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub best_tournament_id: i64,
}

// GET /rankings/virtual (same structure as wppr)

pub type VirtualRankingsResponse = WpprRankingsResponse;

// GET /rankings/women/{tournament_type}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct WomenRankingsResponse {
    pub ranking_type: String,
    pub tournament_type: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub start_pos: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub return_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub total_count: i64,
    pub rankings: Vec<WomenRanking>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct WomenRanking {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_id: i64,
    pub name: String,
    #[serde(
        deserialize_with = "crate::serde_util::optional_string_or_i64",
        default
    )]
    pub age: Option<i64>,
    pub profile_photo: Option<String>,
    pub country_name: String,
    pub country_code: String,
    pub stateprov: String,
    pub city: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub wppr_points: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub current_rank: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub rating_value: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub rating_deviation: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub efficiency_percent: f64,
    pub best_finish: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub best_finish_position: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub best_tournament_id: i64,
}

// GET /rankings/pro/{ranking_system}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct ProRankingsResponse {
    pub ranking_type: String,
    pub rankings: Vec<ProRanking>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct ProRanking {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_id: i64,
    pub name: String,
    pub profile_photo: Option<String>,
    pub country_name: String,
    pub country_code: String,
    pub stateprov: String,
    pub city: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub current_rank: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub pro_points: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub original_wppr_points: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub efficiency_percent: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub adj_efficiency_percent: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub excess_percent: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub wpprtunity: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub wppr_adjustment: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub sos_percent: f64,
}

// GET /rankings/country

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct CountryRankingsResponse {
    pub ranking_type: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub start_position: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub return_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub total_count: i64,
    pub rankings: Vec<CountryRanking>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct CountryRanking {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_id: i64,
    pub name: String,
    #[serde(
        deserialize_with = "crate::serde_util::optional_string_or_i64",
        default
    )]
    pub age: Option<i64>,
    pub profile_photo_url: Option<String>,
    pub country_name: String,
    pub country_code: String,
    pub stateprov: String,
    pub city: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub wppr_points: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub current_wppr_rank: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub current_rank: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub last_month_rank: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub rating: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub rating_deviation: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub efficiency_percent: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub event_count: i64,
    pub best_finish: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub best_finish_position: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub best_tournament_id: i64,
}

// GET /rankings/country_list

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct CountryListResponse {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub count: i64,
    pub country: Vec<CountryListEntry>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct CountryListEntry {
    pub country_name: String,
    pub country_code: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_count: i64,
}

// GET /rankings/custom/list

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct CustomRankingsListResponse {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub total_count: i64,
    pub custom_view: Vec<CustomRankingView>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct CustomRankingView {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub view_id: i64,
    pub title: String,
    pub description: String,
}

// GET /rankings/custom/{id}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct CustomRankingsResponse {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub view_id: i64,
    pub title: String,
    pub description: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub start_position: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub return_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub total_count: i64,
    pub custom_view: Vec<CustomRanking>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct CustomRanking {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_id: i64,
    pub name: String,
    pub country_code: String,
    pub country_name: String,
    pub city: String,
    pub stateprov: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub wppr_rank: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub wppr_points: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub event_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub position: i64,
}
