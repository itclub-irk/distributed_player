use crate::config::NodeConfig;
use chrono::prelude::*;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs, path::Path};

#[derive(Deserialize, Debug, Clone)]
pub struct Playlist {
    pub working_hours: Option<WorkingHours>,
    pub music: Option<Music>,
    pub advertizement: Option<Advertizement>,
    pub time_announcement: Option<TimeAnnouncement>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct WorkingHours {
    pub schedule: Option<Vec<WorkingHoursSchedule>>,
    pub exceptions: Option<HashMap<chrono::NaiveDate, WorkingHoursSchedule>>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct WorkingHoursSchedule(
    #[serde(with = "toml_datetime_compat")] pub chrono::NaiveTime,
    #[serde(with = "toml_datetime_compat")] pub chrono::NaiveTime,
);

#[derive(Deserialize, Debug, Clone)]
pub struct Music {
    pub shuffle: Option<bool>,
    pub schedule: Option<Vec<MusicSchedule>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MusicSchedule(
    #[serde(with = "toml_datetime_compat")] pub chrono::NaiveDate,
    #[serde(with = "toml_datetime_compat")] pub chrono::NaiveDate,
    Vec<String>,
);

#[derive(Deserialize, Debug, Clone)]
pub struct Advertizement {
    pub schedule: Option<HashMap<chrono::NaiveTime, Vec<String>>>,
    pub start_jingle: Option<String>,
    pub end_jingle: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TimeAnnouncement {
    pub folder: Option<String>,
}

fn read_playlist_from_file(file_path: &Path) -> Option<Playlist> {
    let config_file_content = fs::read_to_string(file_path);

    if let Ok(c) = config_file_content {
        let parsed = toml::from_str::<Playlist>(&c);
        if let Ok(p) = parsed {
            return Some(p);
        };
        return None;
    }
    None
}

fn merge_playlists(first: Option<Playlist>, second: Option<Playlist>) -> Option<Playlist> {
    if first.is_some() && second.is_none() {
        return first;
    } else if first.is_none() && second.is_some() {
        return second;
    } else if first.is_some() && second.is_some() {
        let first = first.unwrap();
        let second = second.unwrap();

        let mut merged_working_hours: Option<WorkingHours> = None;
        if first.working_hours.is_some() && second.working_hours.is_none() {
            merged_working_hours = first.working_hours;
        } else if first.working_hours.is_none() && second.working_hours.is_some() {
            merged_working_hours = second.working_hours;
        } else if first.working_hours.is_some() && second.working_hours.is_some() {
            let f = first.working_hours.unwrap();
            let s = second.working_hours.unwrap();
            merged_working_hours = Some(WorkingHours {
                schedule: s.schedule.or(f.schedule),
                exceptions: s.exceptions.or(f.exceptions),
            })
        }

        let mut merged_music: Option<Music> = None;
        if first.music.is_some() && second.music.is_none() {
            merged_music = first.music;
        } else if first.music.is_none() && second.music.is_some() {
            merged_music = second.music;
        } else if first.music.is_some() && second.music.is_some() {
            let f = first.music.unwrap();
            let s = second.music.unwrap();
            merged_music = Some(Music {
                schedule: s.schedule.or(f.schedule),
                shuffle: s.shuffle.or(f.shuffle),
            })
        }

        let mut merged_advertizement: Option<Advertizement> = None;
        if first.advertizement.is_some() && second.advertizement.is_none() {
            merged_advertizement = first.advertizement;
        } else if first.advertizement.is_none() && second.advertizement.is_some() {
            merged_advertizement = second.advertizement;
        } else if first.advertizement.is_some() && second.advertizement.is_some() {
            let f = first.advertizement.unwrap();
            let s = second.advertizement.unwrap();
            merged_advertizement = Some(Advertizement {
                schedule: s.schedule.or(f.schedule),
                start_jingle: s.start_jingle.or(f.start_jingle),
                end_jingle: s.end_jingle.or(f.end_jingle),
            })
        }

        return Some(Playlist {
            working_hours: merged_working_hours,
            music: merged_music,
            advertizement: merged_advertizement,
            time_announcement: second.time_announcement.or(first.time_announcement),
        });
    }
    None
}

fn get_month_day(dt: NaiveDate) -> (u32, u32) {
    (dt.month(), dt.day())
}

impl Playlist {
    /// Returns playlist (if it exists and parseable) for current node
    pub fn read_from_config(node_config: &NodeConfig) -> Option<Playlist> {
        let media_folder = node_config.media.folder.as_str();
        let node_name = &node_config.node.name;

        let cfg_folder = Path::new(media_folder).join("cfg");
        let default_playlist_file_path = cfg_folder.clone().join("playlist.toml");
        let default_playlist = read_playlist_from_file(&default_playlist_file_path);

        if node_name.is_some() {
            let node_playlist_file_path = cfg_folder
                .clone()
                .join(format!("playlist_{}.toml", node_name.as_ref().unwrap()));
            let node_playlist = read_playlist_from_file(&node_playlist_file_path);
            return merge_playlists(default_playlist, node_playlist);
        }

        default_playlist
    }

    /// Returns music folders list for given date
    pub fn get_music_folders_for_date(&self, dt: NaiveDate) -> Vec<String> {
        if let Some(music) = self.music.as_ref() {
            if let Some(schedule) = music.schedule.as_ref() {
                for shedule_entry in schedule.iter() {
                    let mut ignore_year = false;
                    if shedule_entry.0.year() == 1970 {
                        ignore_year = true;
                    };

                    if ignore_year {
                        let (start_month, start_day) = get_month_day(shedule_entry.0);
                        let (end_month, end_day) = get_month_day(shedule_entry.1);
                        let (current_month, current_day) = get_month_day(dt);
                        if current_month < start_month
                            || current_month > end_month
                            || current_day < start_day
                            || current_day > end_day
                        {
                            continue;
                        }
                    } else {
                        if dt < shedule_entry.0 || dt > shedule_entry.1 {
                            continue;
                        }
                    }
                    return shedule_entry.2.clone();
                }
            }
        }

        vec![]
    }

    /// Returns true if given datetime is in working time interval
    pub fn is_working_time(&self, dt: NaiveDateTime) -> bool {
        let date_from_dt = dt.date();
        let weekday = date_from_dt.weekday();
        let time_from_dt = dt.time();

        if let Some(working_hours) = self.working_hours.as_ref() {
            if let Some(schedule) = working_hours.schedule.as_ref() {
                let day_index = weekday.num_days_from_monday() as usize;
                if let Some(mut schedule_for_current_day) = schedule.get(day_index) {
                    if let Some(exceptions) = working_hours.exceptions.as_ref() {
                        if let Some(s) = exceptions.get(&date_from_dt) {
                            schedule_for_current_day = s;
                        };
                    };
                    return time_from_dt >= schedule_for_current_day.0
                        && time_from_dt <= schedule_for_current_day.1;
                };
            };
        };
        false
    }

    /// Returns advertizement media folders for given datetime
    pub fn get_advertizement_folders_for_datetime(&self, dt: NaiveDateTime) -> Vec<String> {
        if let Some(advertizement) = self.advertizement.as_ref() {
            if let Some(schedule) = advertizement.schedule.as_ref() {
                let key = NaiveTime::from_hms_opt(0, dt.minute(), 0).unwrap();

                if let Some(dirs) = schedule.get(&key) {
                    return dirs.clone();
                }
            }
        }
        vec![]
    }

    /// Returns time announcement file path for given datetime
    pub fn get_announcement_file_path_for_time(&self, dt: NaiveDateTime) -> Option<PathBuf> {
        let minutes = dt.time().minute();
        let hours = dt.time().hour();

        if minutes > 0 {
            return None;
        };

        if let Some(time_announcement) = &self.time_announcement {
            if let Some(folder) = time_announcement.folder.as_ref() {
                return Some(Path::join(
                    Path::new(folder),
                    format!("{:0>2}_00.mp3", hours),
                ));
            }
        }
        None
    }

    pub fn get_advertizement_jingles_file_path(&self) -> (Option<String>, Option<String>) {
        if let Some(advertizement) = self.advertizement.as_ref() {
            return (
                advertizement.start_jingle.clone(),
                advertizement.end_jingle.clone(),
            );
        };
        (None, None)
    }
}
