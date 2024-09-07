use crate::config::NodeConfig;
use crate::toml_file::ReadAndWriteTOMLFile;
use chrono::prelude::*;
// use serde_derive::Deserialize;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Playlist {
    pub working_hours: Option<WorkingHours>,
    pub music: Option<Music>,
    pub advertizement: Option<Advertizement>,
    pub time_announcement: Option<TimeAnnouncement>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct WorkingHours {
    pub schedule: Option<Vec<WorkingHoursSchedule>>,
    pub exceptions: Option<HashMap<chrono::NaiveDate, WorkingHoursSchedule>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct WorkingHoursSchedule(
    #[serde(with = "toml_datetime_compat")] pub chrono::NaiveTime,
    #[serde(with = "toml_datetime_compat")] pub chrono::NaiveTime,
);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Music {
    pub shuffle: Option<bool>,
    pub schedule: Option<Vec<MusicSchedule>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MusicSchedule(
    #[serde(with = "toml_datetime_compat")] pub chrono::NaiveDate,
    #[serde(with = "toml_datetime_compat")] pub chrono::NaiveDate,
    Vec<String>,
);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Advertizement {
    pub schedule: Option<HashMap<chrono::NaiveTime, Vec<String>>>,
    pub start_jingle: Option<String>,
    pub end_jingle: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TimeAnnouncement {
    pub folder: Option<String>,
}

impl ReadAndWriteTOMLFile<Playlist> for Playlist {}
