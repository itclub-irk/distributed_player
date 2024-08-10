use crate::{
    config::NodeConfig,
    playlist::{self, Playlist},
};
use chrono::{prelude::*, Duration};
use glob::glob;
use log;
use rand::prelude::*;
use std::{
    path::{Path, PathBuf},
    thread,
};
use vlc::{Media, MediaPlayerAudioEx};

#[derive(Debug)]
pub enum PlayerState {
    Stopped,
    MusicPlaying(Vec<PathBuf>),
    Advertizement(Vec<PathBuf>),
    TimeAnnouncement(PathBuf),
}

pub struct Player<'a> {
    vlc_instance: vlc::Instance,
    media_player: vlc::MediaPlayer,
    node_config: &'a NodeConfig,
    status: PlayerState,
    playlist: Option<Playlist>,
    next_track_index: usize,
    random_generator: ThreadRng,
}

impl Player<'_> {
    pub fn new<'a>(node_config: &'a NodeConfig) -> Player {
        let vlc_instance = vlc::Instance::new().expect("Cannot instaniate libvlc!");
        let media_player =
            vlc::MediaPlayer::new(&vlc_instance).expect("Cannot instaniate vlc media player");
        Player {
            vlc_instance: vlc_instance,
            media_player,
            status: PlayerState::Stopped,
            node_config: node_config,
            next_track_index: 0,
            playlist: None,
            random_generator: thread_rng(),
        }
    }

    pub fn start(&mut self) {
        loop {
            self.dispatch();

            if let PlayerState::MusicPlaying(music_files) = &self.status {
                let next_track_path = &music_files[self.next_track_index];
                log::info!("start {:?}", next_track_path);
                self.play_media_non_blocking(next_track_path.as_path());

                self.next_track_index += 1;
                if self.next_track_index == music_files.len() {
                    self.next_track_index = 0;
                }
            } else if let PlayerState::Advertizement(advertizement_files) = &self.status {
                log::info!("start adv block");

                let mut start_jingle_file_path: Option<String> = None;
                let mut end_jingle_file_path: Option<String> = None;

                if let Some(pl) = self.playlist.as_ref() {
                    (start_jingle_file_path, end_jingle_file_path) =
                        pl.get_advertizement_jingles_file_path();
                }

                if let Some(p) = start_jingle_file_path {
                    self.play_media_blocking(Path::new(&p));
                }
                for advert in advertizement_files.iter() {
                    log::info!("start adv {:?}", advert);
                    self.play_media_blocking(&advert);
                }
                if let Some(p) = end_jingle_file_path {
                    self.play_media_blocking(Path::new(&p));
                }
                log::info!("end adv block");
                self.status = PlayerState::Stopped;
            } else if let PlayerState::TimeAnnouncement(announcement_file_path) = &self.status {
                self.play_media_blocking(&announcement_file_path);
                self.status = PlayerState::Stopped;
            }
        }
    }

    /// Changes player status using playlist rules
    fn dispatch(&mut self) {
        match &self.status {
            PlayerState::Stopped => {
                let pl = self.read_playlist();

                let current_datetime = Local::now().naive_local();

                if !pl.is_working_time(current_datetime) {
                    wait_seconds(1);
                    return;
                }

                let music_folders = pl.get_music_folders_for_date(current_datetime.date());
                if music_folders.len() == 0 {
                    wait_seconds(1);
                    return;
                }

                let music_files = self.load_media_files_list_from_dirs(&music_folders);
                let total_music_files = music_files.len();

                self.status = PlayerState::MusicPlaying(music_files);

                if let Some(music) = pl.music.as_ref() {
                    if let Some(shuffle) = music.shuffle.as_ref() {
                        if *shuffle {
                            self.next_track_index =
                                self.random_generator.gen_range(0..total_music_files);
                        }
                    }
                }
                self.playlist = Some(pl);
            }

            PlayerState::MusicPlaying(_) => {
                let mut prev_dt: NaiveDateTime = Local::now().naive_local() - Duration::seconds(1);
                loop {
                    wait_seconds(1);

                    // track is over
                    if !self.media_player.is_playing() {
                        self.status = PlayerState::Stopped;
                        return;
                    };

                    let dt = Local::now().naive_local();
                    if let Some(pl) = self.playlist.as_ref() {
                        // working time is over
                        if !pl.is_working_time(dt) {
                            self.fade_out();
                            self.status = PlayerState::Stopped;
                            return;
                        }

                        // it's advertizement
                        let advertizement_folders = pl.get_advertizement_folders_for_datetime(dt);
                        if advertizement_folders.len() > 0 {
                            let advertizement_files =
                                self.load_media_files_list_from_dirs(&advertizement_folders);
                            self.fade_out();
                            self.status = PlayerState::Advertizement(advertizement_files);
                            return;
                        }

                        // it's time announcement
                        if prev_dt.hour() != dt.hour() {
                            if let Some(announcement_file) =
                                pl.get_announcement_file_path_for_time(dt)
                            {
                                self.fade_out();
                                self.status = PlayerState::TimeAnnouncement(announcement_file);
                                return;
                            }
                        }
                        prev_dt = dt;
                    }
                }
            }
            _ => wait_seconds(1),
        }
    }

    fn play_media_blocking(&self, path: &Path) {
        self.play_media_non_blocking(path);
        loop {
            wait_seconds(1);
            if !self.media_player.is_playing() {
                break;
            }
        }
    }

    fn play_media_non_blocking(&self, path: &Path) {
        let media_folder = self.node_config.media.folder.as_str();
        let media: Media;

        if !path.starts_with(media_folder) {
            media = vlc::Media::new_path(
                &self.vlc_instance,
                &Path::join(Path::new(media_folder), path),
            )
            .unwrap();
        } else {
            media = vlc::Media::new_path(&self.vlc_instance, path).unwrap();
        }

        self.media_player.set_media(&media);
        self.media_player.set_volume(100).unwrap();
        self.media_player.play().unwrap();
    }

    fn fade_out(&self) {
        if !self.media_player.is_playing() {
            return;
        }
        for vol in (0..100).step_by(10).rev() {
            self.media_player.set_volume(vol).unwrap();
            thread::sleep(::std::time::Duration::from_millis(500));
        }
        self.media_player.pause();
    }

    fn read_playlist(&self) -> Playlist {
        let mut playlist = playlist::Playlist::read_from_config(self.node_config);

        // wait for playlist
        while playlist.is_none() {
            playlist = playlist::Playlist::read_from_config(self.node_config);
            wait_seconds(1);
        }
        return playlist.unwrap();
    }

    pub fn load_media_files_list_from_dirs(&self, dirs: &Vec<String>) -> Vec<PathBuf> {
        let media_folder = &self.node_config.media.folder;
        let mut media_list: Vec<PathBuf> = Vec::new();

        // wait for media files
        loop {
            for dir in dirs {
                media_list.extend(load_media_files_list_from_dir(
                    Path::new(media_folder).join(dir).as_path(),
                ))
            }
            if media_list.len() == 0 {
                wait_seconds(1);
            } else {
                break;
            }
        }

        media_list
    }
}

fn wait_seconds(seconds: u64) {
    thread::sleep(std::time::Duration::from_secs(seconds));
}

fn get_supported_file_wildcards(path: &Path) -> Vec<String> {
    let supported_file_ext = ["mp3", "ogg", "wav", "wma", "flac", "m4a"];
    let mut supported_file_wildcards: Vec<String> = Vec::new();
    for ext in supported_file_ext.iter() {
        let path_as_str = path.to_str().unwrap();
        supported_file_wildcards.push(format!("{path_as_str}/**/*.{ext}"));
    }
    supported_file_wildcards
}

fn load_media_files_list_from_dir(path: &Path) -> Vec<PathBuf> {
    let mut media_list: Vec<PathBuf> = Vec::new();

    let patterns = get_supported_file_wildcards(path);

    let it = patterns.iter().map(|p| glob(&p).unwrap()).flatten();
    for entry in it {
        match entry {
            Ok(p) => {
                media_list.push(p);
            }
            Err(_) => continue,
        }
    }

    media_list
}
