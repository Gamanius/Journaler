use std::path::{PathBuf};

use chrono::NaiveDate;
use egui::ahash::{HashMap, HashMapExt};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub mod journal_editor;
pub mod journal_sidebar;
pub mod journal_save;
pub mod journal_helper;

fn serialize_naive_date<S>(map: &HashMap<NaiveDate, JournalDay>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer, {
    // Convert HashMap<NaiveDate, JournalDay> -> HashMap<String, JournalDay>
    let string_map: HashMap<String, &JournalDay> = map
        .iter()
        .map(|(k, v)| (k.format("%d.%m.%Y").to_string(), v))
        .collect();

    string_map.serialize(serializer)
}

fn deserialize_naive_date<'de, D>(
    deserializer: D,
) -> Result<HashMap<chrono::NaiveDate, JournalDay>, D::Error>
where
    D: Deserializer<'de>,
{
    let string_map: HashMap<String, JournalDay> =
        HashMap::deserialize(deserializer)?;

    let mut result = HashMap::with_capacity(string_map.len());

    for (k, v) in string_map {
        let date = chrono::NaiveDate::parse_from_str(&k, "%d.%m.%Y")
            .map_err(serde::de::Error::custom)?;
        result.insert(date, v);
    }

    Ok(result)
}


fn serialize_naive_time<S>(time: &chrono::NaiveTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer, {
    serializer.serialize_str(&time.format("%H:%M:%S").to_string())
}

fn deserialize_naive_time<'de, D>(deserializer: D) -> Result<chrono::NaiveTime, D::Error>
where
    D: Deserializer<'de>, {
    let s = String::deserialize(deserializer)?;
    let time =
        chrono::NaiveTime::parse_from_str(&s, "%H:%M:%S").map_err(serde::de::Error::custom)?;

    Ok(time)
}


#[derive(Serialize, Deserialize)]
pub struct JournalEntry {
    #[serde(serialize_with  = "serialize_naive_time", deserialize_with="deserialize_naive_time")]
    pub time: chrono::NaiveTime,
    pub entry: String,
}

#[derive(Serialize, Deserialize)]
pub struct JournalDay {
    pub entries : Vec<JournalEntry>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Journal {
    #[serde(serialize_with  = "serialize_naive_date", deserialize_with="deserialize_naive_date")]
    pub days: HashMap<NaiveDate, JournalDay>,
    #[serde(skip)]
    pub path: Option<PathBuf>,
}