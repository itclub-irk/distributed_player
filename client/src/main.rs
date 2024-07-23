use std::path::Path;

use config::NodeConfig;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

mod config;
mod player;
mod playlist;

fn main() {
    let conf = config::NodeConfig::read_from_file("node_config.toml");
    configure_logger(&conf);

    player::Player::new(&conf).start();
}

fn configure_logger(node_config: &NodeConfig) {
    let media_folder = &node_config.media.folder;
    let node_name = node_config.node.name.as_ref();
    let log_file_path = Path::new(media_folder)
        .join("logs")
        .join(format!("log_{}.txt", node_name.unwrap()));

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} {l}: {m}{n}",
        )))
        .build(&log_file_path)
        .expect(&format!("Cannot access to log file {:?}!", &log_file_path));

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).expect("Cannot configure logging framework!");
}
