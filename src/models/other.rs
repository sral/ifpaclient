use serde::Deserialize;

// GET /other/countries

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct CountriesResponse {
    pub country: Vec<Country>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Country {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub country_id: i64,
    pub country_name: String,
    pub country_code: String,
    pub active_flag: String,
}

// GET /other/stateprovs

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct StateProvsResponse {
    pub stateprov: Vec<StateProvCountry>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct StateProvCountry {
    #[serde(deserialize_with = "crate::serde_util::string_or_i64", default)]
    pub country_id: i64,
    pub country_name: String,
    pub country_code: String,
    pub regions: Vec<Region>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Region {
    pub region_name: String,
    pub region_code: String,
}
