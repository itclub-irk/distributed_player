# Distributed player: client

Client for distributed player.

## Node settings

Node settings defined in `node_config.toml` file.

```toml
[media]
folder = "media" # path to media directory (without trailing slash) local or absolute, for windows-based hosts slashes must be escaped: "D:\\media"

[node]
name = "pc101" # node name, if not defined - host name will be used
```

## Playlist settings

General playlist settings defined inside `media` folder, in `cfg/playlist.toml` fiile. Playlist file structure is presented below:

```toml
[working_hours]
# working hours intervals for music playing
schedule = [
  [08:00:00, 20:00:00], # monday
  [08:00:00, 20:00:00], # tuesday
  [08:00:00, 20:00:00], # wednesday
  [08:00:00, 20:00:00], # thursday
  [08:00:00, 20:00:00], # friday
  [08:00:00, 20:00:00], # saturday
  [10:00:00, 18:00:00], # sunday
]
exceptions = {2024-07-21 = [10:00:00, 20:00:00], 2024-07-14 = [10:00:00, 17:00:00]} # if necessary, certain days working hours can be redefined here

[music]
shuffle = true # shuffle music tracks using uniform distribution
# Different music folders can be played at different days. Each record contains start date, end date, music folders list (inside media folder). Intervals from different records must not intersect. Intervals with year 1970 - are annual (valid for every year). Each path in this file must use unix style slashes.
schedule = [
  [1970-01-01, 1970-12-31, [
    "music",
    "special_music",
  ]]
]

[advertizement]
schedule = {"00:10:00" = ["ad_1",], "00:20:00" = ["ad_1", "ad_2",], "00:40:00" = ["ad_1", "ad_2", "ad_3",]} # advertizement schedule, key - minutes count for current hour, value - list of adverts folders
start_jingle="jingle/open.mp3" # each advert block begins with this jingle
end_jingle="jingle/close.mp3" # each advert block ends with this jingle

[time_announcement]
folder = "time_announcement" # folder with time announcement files. File name must match dd_00.mp3 pattern: 09_00.mp3, 23_00.mp3
```

Each individual setting from `cfg/playlist.toml` file can be redefined for current node in file `cfg/playlist_{node_name}.toms`, for example, for node `pc101` file name will be `cfg/playlist_pc101.toml`

## How to build

### Linux-based host

Install rust toolchain using [rustup](https://rustup.rs/);

```sh
$ sudo apt install libvlc libvlc-dev
$ git clone ...
$ cd ...
$ cargo run
```

### Windows-based host

Not compiled yet

## How to contribute

Fork, make feature branch, change, make pull request.
