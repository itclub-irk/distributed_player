use crate::{
    config::NodeConfig,
    playlist::{self, Playlist},
};
use glob::glob;
use rand::prelude::*;
use std::{
    cell::RefCell,
    ops::Deref,
    path::{Path, PathBuf},
    thread,
};

use std::sync::{mpsc::channel, Arc, Mutex};
use vlc::MediaPlayerAudioEx;

#[derive(Debug)]
pub enum PlayerState {
    Stopped,
    // MusicPlaying(Vec<PathBuf>, usize),
    MusicPlaying,
    Advertizement(Vec<PathBuf>, usize),
    TimeAnnouncement(u8),
}

pub struct Player<'a> {
    vlc_instance: vlc::Instance,
    media_player: vlc::MediaPlayer,
    node_config: &'a NodeConfig,
    status: PlayerState,
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
            random_generator: thread_rng(),
        }
    }

    pub fn play(&mut self) {
        loop {

            
        }
    }

    fn dispatch(&mut self) {

    }

    pub fn play1(&mut self) {
        let player_state = Arc::new(Mutex::new(PlayerState::Stopped));

        loop {
            let player_state_for_callback = Arc::clone(&player_state);
            let pl = self.read_playlist();
            let media_files = self.load_media_files_list_from_dirs(&pl.folders.unwrap());

            let mut next_file = &media_files[0];
            if pl.shuffle.unwrap_or(false) {
                next_file = media_files.choose(&mut self.random_generator).unwrap();
            }

            let media = vlc::Media::new_path(&self.vlc_instance, next_file).unwrap();
            self.media_player.set_media(&media);
            self.media_player.set_volume(100).unwrap();
            self.media_player.play().unwrap();

            {
                let mut s = player_state.lock().unwrap();
                *s = PlayerState::MusicPlaying;
            }

            // let (tx, _rx) = channel::<()>();

            // let em = media.event_manager();
            // let _ = em.attach(vlc::EventType::MediaStateChanged, move |e, _| match e {
            //     vlc::Event::MediaStateChanged(s) => {
            //         if s == vlc::State::Ended || s == vlc::State::Error {
            //             let mut s = player_state_for_callback.lock().unwrap();
            //             *s = PlayerState::Stopped;
            //             tx.send(()).unwrap();
            //         }
            //     }
            //     _ => (),
            // });

            loop {
                // {
                //     let s = player_state.lock().unwrap();
                //     if let PlayerState::Stopped = s.deref() {
                //         println!("Track ended, next track");
                //         break;
                //     }
                // }
                thread::sleep(std::time::Duration::from_secs(1));
                if !self.media_player.is_playing() {
                    println!("Track ended, switch to next track");
                    break;
                }
                println!("ping");
            }
        }
    }

    fn read_playlist(&self) -> Playlist {
        let mut playlist = playlist::Playlist::read_from_config(self.node_config);

        // wait for playlist
        while playlist.is_none() {
            playlist = playlist::Playlist::read_from_config(self.node_config);
            thread::sleep(std::time::Duration::from_secs(1));
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
                thread::sleep(std::time::Duration::from_secs(1));
            } else {
                break;
            }
        }

        media_list
    }
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
