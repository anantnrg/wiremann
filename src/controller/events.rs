use std::path::PathBuf;

#[derive(Clone, PartialEq, Debug)]
pub enum AudioEvent {
    TrackLoaded(PathBuf),
    Position(u64),
}

#[derive(Clone, PartialEq, Debug)]
pub enum ScannerEvent {
    TrackMetadata {
        title: String,
        artist: String,
        album: String,
        duration: u64,
        size: u64,
        modified: u64,
    },
}
