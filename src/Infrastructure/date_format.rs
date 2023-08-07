use chrono::NaiveDate;
use serde::{self, Deserialize, Deserializer};

pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let date_format =
        super::environment::get_env("CSV_DATE_FORMAT").map_err(serde::de::Error::custom)?;

    let s = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, &date_format).map_err(serde::de::Error::custom)
}
