use std::str::FromStr;

//use chrono::DateTime;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Aggregator {
    pub id: u8,
    pub source: String,
    pub tokens: Data,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Data {
    pub data: Vec<Token>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Token {
    pub key: String,
    pub value: TokenEntry,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct TokenEntry {
    pub name: String,
    pub symbol: String,
    pub token_details_list: Vec<TokenDetails>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct TokenDetails {
    pub decimals: u8,
    #[serde(deserialize_with = "deserialize_string")]
    pub last_update: i64,
    #[serde(deserialize_with = "deserialize_string")]
    pub price: u128,
}

impl Default for TokenDetails {
    fn default() -> Self {
        Self {
            decimals: 0,
            last_update: 0,
            price: 0,
        }
    }
}

impl Ord for TokenDetails {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (&self.last_update).cmp(&other.last_update)
    }
}

fn deserialize_string<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
    D: Deserializer<'de>,
{
    let s = <String>::deserialize(deserializer)?;

    s.trim()
        .parse::<T>()
        .map_err(<D::Error as ::serde::de::Error>::custom)
}

// fn deserialize_string_to_date<'de, D>(deserializer: D) -> Result<i64, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let s = <String>::deserialize(deserializer)?;

//     let datetime = DateTime::parse_from_rfc3339(&s).unwrap();

//     Ok(datetime.timestamp())
// }
