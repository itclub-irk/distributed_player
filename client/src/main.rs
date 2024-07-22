mod config;
mod player;
mod playlist;

fn main() {
    let conf = config::NodeConfig::read_from_file("node_config.toml");
    player::Player::new(&conf).start();
}
