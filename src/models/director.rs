use serde::{Deserialize, Serialize};

// GET /director/{id}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Director {
    pub name: String,
    pub profile_photo: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub director_id: i64,
    pub city: String,
    pub stateprov: String,
    pub country_name: String,
    pub country_code: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub country_id: i64,
    pub twitch_username: Option<String>,
    pub stats: DirectorStats,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct DirectorStats {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub tournament_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub unique_location_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub women_tournament_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub league_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub highest_value: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub average_value: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub total_player_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub unique_player_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub first_time_player_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub repeat_player_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub largest_event_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub single_format_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub multiple_format_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub unknown_format_count: i64,
    pub formats: Vec<DirectorFormat>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct DirectorFormat {
    pub name: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub count: i64,
}

// GET /director/{id}/tournaments/{time_period}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct DirectorTournamentsResponse {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub director_id: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub tournament_count: i64,
    pub tournaments: Vec<DirectorTournament>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct DirectorTournament {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub tournament_id: i64,
    pub tournament_name: String,
    pub event_name: String,
    pub ranking_system: String,
    pub city: String,
    pub stateprov_code: String,
    pub country_code: String,
    pub country_name: String,
    pub event_start_date: String,
    pub event_end_date: String,
    pub qualifying_format: String,
    pub finals_format: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_count: i64,
}

// GET /director/country

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct CountryDirectorsResponse {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub count: i64,
    pub country_directors: Vec<CountryDirector>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct CountryDirector {
    pub player_profile: CountryDirectorProfile,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct CountryDirectorProfile {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_id: i64,
    pub name: String,
    pub country_code: String,
    pub country_name: String,
    pub profile_photo: String,
}

// GET /director/search

#[derive(Debug, Clone, Default, Serialize)]
pub struct DirectorSearchParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(
        deserialize_with = "crate::serde_util::optional_string_or_i64",
        default
    )]
    pub count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct DirectorSearchResponse {
    pub search_term: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub count: i64,
    pub directors: Vec<DirectorSearchResult>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct DirectorSearchResult {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub director_id: i64,
    pub name: String,
    pub city: Option<String>,
    pub stateprov: Option<String>,
    pub country_name: Option<String>,
    pub country_code: Option<String>,
    pub profile_photo: Option<String>,
    pub stats: DirectorSearchStats,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct DirectorSearchStats {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub event_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub future_event_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub unique_player_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub total_player_count: i64,
    pub last_event_date: Option<String>,
}
