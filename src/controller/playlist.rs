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
    pub fn open_path(path: PathBuf) -> Playlist {
        let supported = ["mp3", "flac", "wav", "ogg", "m4a"];
        let tracks: Vec<Track> = WalkDir::new(path.clone())
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
            .collect::<Vec<_>>()
            .par_iter()
            .filter_map(|file| {
                Metadata::read(file.clone()).ok().map(|meta| Track {
                    path: file.clone(),
                    meta,
                })
            })
            .collect();

        let name = path
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .map(|s| s.to_string())
            .unwrap();

        Playlist {
            name,
            path: Some(path),
            tracks,
        }
    }
}

impl gpui::Global for Playlist {}
