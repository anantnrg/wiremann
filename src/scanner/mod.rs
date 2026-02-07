use crate::controller::metadata::Metadata;
use crate::controller::player::{ScannerCommand, ScannerEvent, Track};
use crate::utils::decode_thumbnail;
use crossbeam_channel::{select, Receiver, Sender};
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
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
    cancel_thumbs: Option<Arc<AtomicBool>>,
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
            cancel_thumbs: None,
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

        if let Some(flag) = &self.cancel_thumbs {
            flag.store(true, Ordering::Relaxed);
        }

        let _ = self.scanner_event_tx.send(ScannerEvent::ClearImageCache);

        let cancel = Arc::new(AtomicBool::new(false));
        self.cancel_thumbs = Some(cancel.clone());

        let path = PathBuf::from(path);
        let tracks = self.scan(path.clone());

        let name = path
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .map(|s| s.to_string())
            .unwrap();

        let playlist = Playlist {
            name,
            path: Some(path),
            tracks: tracks.clone(),
        };

        self.state.current_playlist = Some(playlist);

        let _ = self.scanner_event_tx.send(ScannerEvent::State(self.state.clone()));

        let tx = self.scanner_event_tx.clone();

        std::thread::spawn(move || {
            let threads = std::cmp::max(1, num_cpus::get() / 2);

            let pool = ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap();
            pool.install(|| {
                tracks.par_iter().for_each(|track| {
                    if cancel.load(Ordering::Relaxed) {
                        return;
                    }

                    if let Some(bytes) = track.meta.thumbnail.clone() {
                        if let Ok(image) = decode_thumbnail(bytes.into_boxed_slice(), true) {
                            let _ = tx.send(ScannerEvent::Thumbnail {
                                path: track.path.clone(),
                                image,
                            });
                        }
                    }
                });
            });
        });
    }

    fn scan(&mut self, path: PathBuf) -> Vec<Track> {
        let supported = ["mp3", "flac", "wav", "ogg", "m4a"];
        let threads = std::cmp::max(1, num_cpus::get() / 2);

        let pool = ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap();
        pool.install(|| {
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
        })
    }
}