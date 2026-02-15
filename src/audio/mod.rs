pub mod engine;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(
    Clone, Copy, Hash, Eq, PartialEq,
    Serialize, Deserialize, Debug
)]
pub struct TrackId(pub [u8; 16]);

#[derive(Serialize, Deserialize, Clone)]
pub struct TrackHeader {
    pub id: TrackId,
    pub path: PathBuf,

    pub title: String,
    pub artist: String,
    pub album: String,

    pub duration: u32,
}