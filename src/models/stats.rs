use serde::{Deserialize, Serialize};

// Shared params

#[derive(Debug, Clone, Default, Serialize)]
pub struct StatsRankTypeParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_type: Option<super::common::RankType>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct StatsCountryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_type: Option<super::common::RankType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PeriodParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_type: Option<super::common::RankType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(
        deserialize_with = "crate::serde_util::optional_string_or_i64",
        default
    )]
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct LucrativeTournamentsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_type: Option<super::common::RankType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct OverallStatsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_code: Option<super::common::RankType>,
}

// GET /stats/country_players

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct CountryPlayersResponse {
    #[serde(rename = "type")]
    pub stat_type: String,
    pub rank_type: String,
    pub stats: Vec<CountryPlayerStat>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct CountryPlayerStat {
    pub country_name: String,
    pub country_code: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub stats_rank: i64,
}

// GET /stats/state_players

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct StatePlayersResponse {
    #[serde(rename = "type")]
    pub stat_type: String,
    pub rank_type: String,
    pub stats: Vec<StatePlayerStat>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct StatePlayerStat {
    pub stateprov_code: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub stats_rank: i64,
}

// GET /stats/state_tournaments

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct StateTournamentsResponse {
    #[serde(rename = "type")]
    pub stat_type: String,
    pub rank_type: String,
    pub stats: Vec<StateTournamentStat>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct StateTournamentStat {
    pub stateprov_code: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub total_points_all: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub total_points_tournament_value: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub stats_rank: i64,
}

// GET /stats/events_by_year

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct EventsByYearResponse {
    #[serde(rename = "type")]
    pub stat_type: String,
    pub rank_type: String,
    pub stats: Vec<EventsByYearStat>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct EventsByYearStat {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub year: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub country_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub tournament_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub stats_rank: i64,
}

// GET /stats/players_by_year

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PlayersByYearResponse {
    #[serde(rename = "type")]
    pub stat_type: String,
    pub rank_type: String,
    pub stats: Vec<PlayersByYearStat>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PlayersByYearStat {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub year: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub current_year_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub previous_year_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub previous_2_year_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub stats_rank: i64,
}

// GET /stats/largest_tournaments

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct LargestTournamentsResponse {
    #[serde(rename = "type")]
    pub stat_type: String,
    pub rank_type: String,
    pub stats: Vec<LargestTournamentStat>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct LargestTournamentStat {
    pub country_name: String,
    pub country_code: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub tournament_id: i64,
    pub tournament_name: String,
    pub event_name: String,
    pub tournament_date: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub stats_rank: i64,
}

// GET /stats/lucrative_tournaments

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct LucrativeTournamentsResponse {
    #[serde(rename = "type")]
    pub stat_type: String,
    pub rank_type: String,
    pub stats: Vec<LucrativeTournamentStat>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct LucrativeTournamentStat {
    pub country_name: String,
    pub country_code: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub tournament_id: i64,
    pub tournament_name: String,
    pub event_name: String,
    pub tournament_date: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub tournament_value: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub stats_rank: i64,
}

// GET /stats/points_given_period

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PointsGivenPeriodResponse {
    #[serde(rename = "type")]
    pub stat_type: String,
    pub start_date: String,
    pub end_date: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub return_count: i64,
    pub rank_type: String,
    pub stats: Vec<PointsGivenPeriodStat>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PointsGivenPeriodStat {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_id: i64,
    pub first_name: String,
    pub last_name: String,
    pub country_name: String,
    pub country_code: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub wppr_points: f64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub stats_rank: i64,
}

// GET /stats/events_attended_period

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct EventsAttendedPeriodResponse {
    #[serde(rename = "type")]
    pub stat_type: String,
    pub start_date: String,
    pub end_date: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub return_count: i64,
    pub system_code: String,
    pub stats: Vec<EventsAttendedPeriodStat>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct EventsAttendedPeriodStat {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub player_id: i64,
    pub first_name: String,
    pub last_name: String,
    pub country_name: String,
    pub country_code: String,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub tournament_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub stats_rank: i64,
}

// GET /stats/overall

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct OverallStatsResponse {
    #[serde(rename = "type")]
    pub stat_type: String,
    pub system_code: String,
    pub stats: OverallStat,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct OverallStat {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub overall_player_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub active_player_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub tournament_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub tournament_count_last_month: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub tournament_count_this_year: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub tournament_player_count: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_f64", default)]
    pub tournament_player_count_average: f64,
    pub age_gender: Option<AgeGender>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct AgeGender {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub age_under_18: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub age_18_to_29: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub age_30_to_39: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub age_40_to_49: i64,
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub age_50_to_99: i64,
}
