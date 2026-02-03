use super::{metadata::Metadata, player::Track};
use rayon::prelude::*;
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct Playlist {
    pub name: String,
    pub path: Option<PathBuf>,
    pub tracks: Vec<Track>,
}

impl Playlist {
    pub fn open_path(path: PathBuf) {
        let supported = ["mp3", "flac", "wav", "ogg", "m4a"];
        let files: Vec<PathBuf> = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| {
                e.path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| supported.contains(&ext.to_lowercase().as_str()))
                    .unwrap_or(false)
            })
            .map(|e| e.path().to_path_buf())
            .collect();

        let metadatas: Vec<Metadata> = files
            .par_iter()
            .filter_map(|file| Metadata::read(file.clone()).ok())
            .collect();

        for m in metadatas {
            println!("{:#?}", m.title);
        }
    }
}
