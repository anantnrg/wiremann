use std::path::PathBuf;

use crate::library::TrackId;

#[derive(Clone, PartialEq, Debug)]
pub enum AudioEvent {
    TrackLoaded(PathBuf),
    Position(u64),
}

#[derive(Clone, PartialEq, Debug)]
pub enum ScannerEvent {
    TrackMetadata {
        path: PathBuf,
        track_id: TrackId,
        title: String,
        artist: String,
        album: String,
        duration: u64,
        size: u64,
        modified: u64,
    },
}
