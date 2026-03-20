pub mod playlists;

use crate::errors::ScannerError;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::{Path, PathBuf};

const AUDIO_HASH_SEED: u64 = 0x3141_5926_5358_9793;
const IMAGE_HASH_SEED: u64 = 0x2718_2818_2845_9045;
const CHUNK_SIZE: u64 = 65536;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct TrackId(pub [u8; 16]);

#[derive(Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct ImageId(pub [u8; 16]);

#[derive(Clone, Debug, PartialEq)]
pub struct Track {
    pub id: TrackId,
    pub sources: Vec<TrackSource>,

    pub title: String,
    pub artist: String,
    pub album: String,

    pub duration: u64,

    pub image_id: Option<ImageId>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TrackSource {
    pub path: PathBuf,
    pub size: u64,
    pub modified: u64,
}


impl TrackId {
    #[allow(clippy::missing_errors_doc)]
    pub fn generate(path: &Path) -> Result<Self, ScannerError> {
        let file = File::open(path)?;

        let length = file.metadata()?.len();

        if length > CHUNK_SIZE * 3 {
            let length = length as f64;
            let first_chunk = (length * 0.25) as u64;
            let second_chunk = (length * 0.5) as u64;
            let third_chunk = (length * 0.75) as u64;
        }
    }
}
