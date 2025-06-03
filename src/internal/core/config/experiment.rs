use serde::{Deserialize, Deserializer, Serialize};

use crate::internal::core::log::LoggerLevel;

#[derive(Serialize, Deserialize)]
pub struct Experiment {
    pub name: String,
    pub simulation: String,
    #[serde(
        default,
        serialize_with = "option_u64_as_str",
        deserialize_with = "option_u64_from_str"
    )]
    pub seed: Option<u64>,
    pub logger_level: LoggerLevel,
    pub n_peers: Option<usize>,
    pub topology: Option<String>,
    pub arrival_time: Option<String>,
}

fn option_u64_as_str<S>(opt: &Option<u64>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match opt {
        Some(val) => s.serialize_str(&val.to_string()),
        None => s.serialize_none(),
    }
}

fn option_u64_from_str<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{Error, Unexpected};

    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) => s
            .parse::<u64>()
            .map(Some)
            .map_err(|_| D::Error::invalid_value(Unexpected::Str(&s), &"a u64 string")),
        None => Ok(None),
    }
}
