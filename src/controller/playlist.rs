use std::path::PathBuf;

use super::player::Track;

pub struct Playlist {
    pub name: String,
    pub path: Option<PathBuf>,
    pub tracks: Vec<Track>,
}

impl Playlist {
    pub fn open_path(path: PathBuf) {}
}
