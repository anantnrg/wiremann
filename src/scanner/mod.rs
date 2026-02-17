use crossbeam_channel::{Receiver, Sender};
use lofty::{prelude::*, probe::Probe};
use std::{fs, path::PathBuf, time::UNIX_EPOCH};

use crate::{
    controller::{commands::ScannerCommand, events::ScannerEvent},
    errors::ScannerError,
    library::TrackId,
};

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
}
