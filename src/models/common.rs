use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimePeriod {
    Past,
    Future,
}

impl fmt::Display for TimePeriod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Past => f.write_str("PAST"),
            Self::Future => f.write_str("FUTURE"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RankingSystem {
    Main,
    Women,
    Youth,
}

impl fmt::Display for RankingSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Main => f.write_str("MAIN"),
            Self::Women => f.write_str("WOMEN"),
            Self::Youth => f.write_str("YOUTH"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultType {
    Active,
    Nonactive,
    Inactive,
}

impl fmt::Display for ResultType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => f.write_str("ACTIVE"),
            Self::Nonactive => f.write_str("NONACTIVE"),
            Self::Inactive => f.write_str("INACTIVE"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WomenTournamentType {
    Open,
    Women,
}

impl fmt::Display for WomenTournamentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => f.write_str("OPEN"),
            Self::Women => f.write_str("WOMEN"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeagueTimePeriod {
    Active,
    History,
    Upcoming,
}

impl fmt::Display for LeagueTimePeriod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => f.write_str("ACTIVE"),
            Self::History => f.write_str("HISTORY"),
            Self::Upcoming => f.write_str("UPCOMING"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum RankType {
    #[serde(rename = "OPEN")]
    Open,
    #[serde(rename = "WOMEN")]
    Women,
}

impl fmt::Display for RankType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => f.write_str("OPEN"),
            Self::Women => f.write_str("WOMEN"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum EventType {
    Tournament,
    League,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DistanceUnit {
    #[serde(rename = "k")]
    Kilometers,
    #[serde(rename = "m")]
    Miles,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PaginationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_pos: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
