use std::path::PathBuf;

use crate::library::TrackId;

pub enum AudioCommand {
    Load(PathBuf),
    GetPosition,
}

pub enum ScannerCommand {
    GetTrackMetadata { path: PathBuf, track_id: TrackId },
}
