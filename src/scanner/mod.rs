use crate::library::gen_track_id;
use crate::library::playlists::{Playlist, PlaylistId, PlaylistSource};
use crate::{
    controller::{commands::ScannerCommand, events::ScannerEvent},
    errors::ScannerError,
    library::TrackId,
};
use crossbeam_channel::{Receiver, Sender};
use lofty::{prelude::*, probe::Probe};
use std::collections::HashSet;
use std::{fs, path::PathBuf, time::UNIX_EPOCH};
use uuid::Uuid;
use walkdir::WalkDir;

pub struct Scanner {
    pub tx: Sender<ScannerEvent>,
    pub rx: Receiver<ScannerCommand>,
}

impl Scanner {
    pub fn new() -> (Self, Sender<ScannerCommand>, Receiver<ScannerEvent>) {
        let (cmd_tx, cmd_rx) = crossbeam_channel::unbounded();
        let (event_tx, event_rx) = crossbeam_channel::unbounded();

        let scanner = Scanner {
            tx: event_tx,
            rx: cmd_rx,
        };

        (scanner, cmd_tx, event_rx)
    }

    pub fn run(&mut self) -> Result<(), ScannerError> {
        loop {
            while let Ok(cmd) = self.rx.try_recv() {
                match cmd {
                    ScannerCommand::GetTrackMetadata { path, track_id } => {
                        self.get_track_metadata(path, track_id)?
                    }
                    ScannerCommand::ScanFolder { path, tracks } => self.scan_folder(path, tracks)?,
                }
            }
        }
    }

    fn get_track_metadata(&mut self, path: PathBuf, track_id: TrackId) -> Result<(), ScannerError> {
        let tagged_file = Probe::open(path.clone())?.guess_file_type()?.read()?;
        let file_metadata = fs::metadata(path.clone())?;

        let tag = match tagged_file.primary_tag() {
            Some(primary_tag) => primary_tag,
            None => tagged_file
                .first_tag()
                .expect("ERROR: could not find any tags!"),
        };
        let title = tag
            .get_string(ItemKey::TrackTitle)
            .unwrap_or("Untitled")
            .to_string();
        let artists: Vec<String> = tag
            .get_strings(ItemKey::TrackArtist)
            .map(|s| s.to_owned())
            .collect();
        let artist = artists.join(", ");
        let album = tag
            .get_string(ItemKey::AlbumTitle)
            .unwrap_or("Unknown Album")
            .to_string();
        let duration = tagged_file.properties().duration().as_secs();
        let size = file_metadata.len();
        let modified = file_metadata
            .modified()?
            .duration_since(UNIX_EPOCH)?
            .as_secs();

        let _ = self.tx.send(ScannerEvent::TrackMetadata {
            track_id,
            path,
            title,
            artist,
            album,
            duration,
            size,
            modified,
        });

        Ok(())
    }

    fn scan_folder(&mut self, path: PathBuf, tracks: HashSet<TrackId>) -> Result<(), ScannerError> {
        let supported = ["mp3", "flac", "wav", "ogg", "m4a"];
        let mut new_tracks = vec![];
        if path.is_dir() {
            for entry in WalkDir::new(path.clone()).into_iter().filter_map(|e| e.ok()).filter(|e| e.file_type().is_file()).filter(|e| {
                e.path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| supported.contains(&ext.to_lowercase().as_str()))
                    .unwrap_or(false)
            })
                .map(|e| e.path().to_path_buf()) {
                let track_id = gen_track_id(&PathBuf::from(entry.clone()))?;

                if tracks.contains(&track_id) {
                    new_tracks.push(track_id);
                } else {
                    self.get_track_metadata(entry.clone(), track_id)?;
                    new_tracks.push(track_id);
                }
            }
        }

        let name = path
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .map(|s| s.to_string())
            .unwrap();

        let playlist = Playlist {
            id: PlaylistId(Uuid::new_v4()),
            name,
            source: PlaylistSource::Folder(path),
            tracks: new_tracks,
        };
        
        let _ = self.tx.send(ScannerEvent::Playlist(playlist));
        
        Ok(())
    }
}
