pub mod engine;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use blake3::Hasher;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct TrackId(pub [u8; 16]);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Track {
    pub id: TrackId,
    pub path: PathBuf,

    pub title: String,
    pub artist: String,
    pub album: String,

    pub duration: u32,
    pub size: u64,
    pub modified: u64
}


fn gen_track_id(path: &Path, size: u64, modified: u64) -> TrackId {
    let mut hasher = Hasher::new();

    hasher.update(path.to_string_lossy().as_bytes());
    hasher.update(&size.to_le_bytes());
    hasher.update(&modified.to_le_bytes());

    TrackId::from(hasher.finalize())
}