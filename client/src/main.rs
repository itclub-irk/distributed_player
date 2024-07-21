use player::Player;
use std::path::Path;
use std::sync::mpsc::channel;
use std::{str::FromStr, thread};
use vlc;
use vlc::{Instance, Media, MediaLibrary, MediaPlayer, MediaPlayerAudioEx};

mod config;
mod controller;
mod player;
mod playlist;

fn main() {
    let conf = config::NodeConfig::read_from_file("node_config.toml");
    player::Player::new(&conf).play();

    // let vlc_service = vlc::Instance::new().unwrap();
    // // let media = vlc::Media::new_location(&vlc_service, &conf.media.folder).unwrap();

    // let media = vlc::Media::new_path(&vlc_service, &files.get(0).unwrap()).unwrap();
    // let vlc_player = vlc::MediaPlayer::new(&vlc_service).unwrap();
    // vlc_player.set_media(&media);
    // vlc_player.set_volume(100).unwrap();
    // vlc_player.play().unwrap();

    // let (tx, rx) = channel::<()>();

    // let em = media.event_manager();
    // let _ = em.attach(vlc::EventType::MediaStateChanged, move |e, _| match e {
    //     vlc::Event::MediaStateChanged(s) => {
    //         println!("State : {:?}", s);
    //         if s == vlc::State::Ended || s == vlc::State::Error {
    //             tx.send(()).unwrap();
    //         }
    //     }
    //     _ => (),
    // });

    // loop {
    //     thread::sleep(std::time::Duration::from_secs(1));
    //     println!("ping");
    // }
    // loop {

    // }

    // thread::sleep(::std::time::Duration::from_secs(100));
    // // MainWindow::new().unwrap().run().unwrap();
    // // Create an instance
    // let instance = Instance::new().unwrap();
    // // Create a media from a file
    // let md = Media::new_path(&instance, "media/halo.mp3").unwrap();
    // // Create a media player
    // let mdp = MediaPlayer::new(&instance).unwrap();
    // mdp.set_media(&md);
    // mdp.set_volume(100).unwrap();
    // // Start playing
    // mdp.play().unwrap();

    // // Wait for 10 seconds
    // thread::sleep(::std::time::Duration::from_secs(10));
    // fade_volume(&mdp);
}

fn load_config() {}

fn create_equalizer_from_config() {}

fn fade_volume(mdp: &MediaPlayer) {
    for vol in (0..100).step_by(10).rev() {
        mdp.set_volume(vol).unwrap();
        thread::sleep(::std::time::Duration::from_millis(500));
    }
}
