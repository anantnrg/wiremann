use crate::library::Track;
use crate::library::playlists::Playlist;
use std::path::PathBuf;

#[derive(Clone, PartialEq, Debug)]
pub enum AudioEvent {
    TrackLoaded(PathBuf),
    Position(u64),
}

#[derive(Clone, PartialEq, Debug)]
pub enum ScannerEvent {
    Tracks(Vec<Track>),
    Playlist(Playlist),
}
