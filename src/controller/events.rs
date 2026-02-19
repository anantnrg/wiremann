use crate::controller::state::PlaybackStatus;
use crate::library::playlists::Playlist;
use crate::library::Track;
use std::path::PathBuf;

#[derive(Clone, PartialEq, Debug)]
pub enum AudioEvent {
    TrackLoaded(PathBuf),
    Position(u64),
    PlaybackStatus(PlaybackStatus),
    Volume(f32),
    TrackEnded,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ScannerEvent {
    Tracks(Vec<Track>),
    Playlist(Playlist),
}
