// The IFPA API returns numeric and boolean values inconsistently — sometimes as
// native JSON types, sometimes as strings, and sometimes as null. These helpers
// let serde handle all three representations transparently.
#![allow(dead_code)]

use serde::{Deserialize, Deserializer};

pub(crate) fn string_or_i64<'de, D: Deserializer<'de>>(d: D) -> Result<i64, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum V {
        Str(String),
        Num(i64),
    }
    match Option::<V>::deserialize(d)? {
        Some(V::Str(s)) if s.is_empty() => Ok(0),
        Some(V::Str(s)) => s.parse().map_err(serde::de::Error::custom),
        Some(V::Num(n)) => Ok(n),
        None => Ok(0),
    }
}

pub(crate) fn optional_string_or_i64<'de, D: Deserializer<'de>>(
    d: D,
) -> Result<Option<i64>, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum V {
        Str(String),
        Num(i64),
    }
    match Option::<V>::deserialize(d)? {
        Some(V::Str(s)) if s.is_empty() => Ok(None),
        Some(V::Str(s)) => s.parse().map(Some).map_err(serde::de::Error::custom),
        Some(V::Num(n)) => Ok(Some(n)),
        None => Ok(None),
    }
}

pub(crate) fn string_or_f64<'de, D: Deserializer<'de>>(d: D) -> Result<f64, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum V {
        Str(String),
        Num(f64),
    }
    match Option::<V>::deserialize(d)? {
        Some(V::Str(s)) if s.is_empty() => Ok(0.0),
        Some(V::Str(s)) => s.parse().map_err(serde::de::Error::custom),
        Some(V::Num(n)) => Ok(n),
        None => Ok(0.0),
    }
}

pub(crate) fn string_or_bool<'de, D: Deserializer<'de>>(d: D) -> Result<bool, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum V {
        Str(String),
        Bool(bool),
    }
    match Option::<V>::deserialize(d)? {
        Some(V::Str(s)) => match s.as_str() {
            "true" | "1" => Ok(true),
            _ => Ok(false),
        },
        Some(V::Bool(b)) => Ok(b),
        None => Ok(false),
    }
}
