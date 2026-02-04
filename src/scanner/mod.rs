use crate::controller::player::{ScannerCommand, ScannerEvent, Track};
use crossbeam_channel::{Receiver, Sender};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct Playlist {
    pub name: String,
    pub path: Option<PathBuf>,
    pub tracks: Vec<Track>,
}

pub struct Scanner {
    pub cmd_rx: Receiver<ScannerEvent>,
    pub cmd_tx: Sender<ScannerCommand>,
    pub state: ScannerState,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScannerState {
    pub current_playlist: Playlist,
    pub playlists: Vec<String>,
}