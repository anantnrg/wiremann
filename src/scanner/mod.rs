use crate::controller::metadata::Metadata;
use crate::controller::player::{ScannerCommand, ScannerEvent, Track};
use crossbeam_channel::{select, Receiver, Sender};
use rayon::prelude::*;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq)]
pub struct Playlist {
    pub name: String,
    pub path: Option<PathBuf>,
    pub tracks: Vec<Track>,
}

pub struct Scanner {
    pub scanner_cmd_rx: Receiver<ScannerCommand>,
    pub scanner_event_tx: Sender<ScannerEvent>,
    pub state: ScannerState,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct ScannerState {
    pub current_playlist: Option<Playlist>,
    pub playlists: Option<Vec<String>>,
}

impl Scanner {
    pub fn run(scanner_cmd_rx: Receiver<ScannerCommand>, scanner_event_tx: Sender<ScannerEvent>) {
        let state = ScannerState::default();
        let mut scanner = Scanner {
            scanner_cmd_rx,
            scanner_event_tx,
            state,
        };

        scanner.event_loop();
    }

    fn event_loop(&mut self) {
        loop {
            select! {
                recv(self.scanner_cmd_rx) -> msg => {
                    let cmd = match msg {
                        Ok(c) => c,
                        Err(_) => break,
                    };

                    match cmd {
                        ScannerCommand::Load(path) => self.load(&path),
                    }
            }}
        }
    }

    fn load(&mut self, path: &String) {
        // TODO: Check if playlist has already been cached

        let tracks = self.scan(PathBuf::from(path));
    }

    fn scan(&mut self, path: PathBuf) -> Vec<Track> {
        let supported = ["mp3", "flac", "wav", "ogg", "m4a"];
        WalkDir::new(path)
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
            .collect()
    }
}