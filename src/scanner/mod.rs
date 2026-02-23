use crate::library::playlists::{Playlist, PlaylistId, PlaylistSource};
use crate::library::{gen_track_id, Track};
use crate::{
    controller::{commands::ScannerCommand, events::ScannerEvent},
    errors::ScannerError,
    library::TrackId,
};
use crossbeam_channel::{Receiver, Sender};
use gpui::RenderImage;
use image::imageops::thumbnail;
use image::{Frame, ImageReader};
use lofty::{prelude::*, probe::Probe};
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use smallvec::smallvec;
use std::collections::{HashMap, HashSet};
use std::io::Cursor;
use std::sync::Arc;
use std::{fs, path::PathBuf, time::UNIX_EPOCH};
use uuid::Uuid;
use walkdir::WalkDir;

pub struct Scanner {
    pub tx: Sender<ScannerEvent>,
    pub rx: Receiver<ScannerCommand>,
}

struct ScanResult {
    id: TrackId,
    track: Option<Track>,
    image: Option<Vec<u8>>,
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
            match self.rx.recv()? {
                ScannerCommand::GetTrackMetadata { path, track_id } => {
                    let (track, image) = self.get_track_metadata(path.clone(), track_id)?;
                    let _ = self.tx.send(ScannerEvent::Tracks(vec![track]));

                    match image {
                        Some(image) => {
                            match render_album_art(&image, false) {
                                Ok(img) => {
                                    let _ = self.tx.send(ScannerEvent::AlbumArt(img));
                                }
                                Err(e) => return Err(e),
                            }
                        }
                        None => {}
                    }
                }
                ScannerCommand::ScanFolder { path, tracks } => self.scan_folder(path, tracks)?,
            }
        }
    }

    fn get_track_metadata(
        &self,
        path: PathBuf,
        track_id: TrackId,
    ) -> Result<(Track, Option<Vec<u8>>), ScannerError> {
        let tagged_file = match Probe::open(path.clone())
            .and_then(|p| Ok(p.guess_file_type()?))
            .and_then(|p| p.read())
        {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Metadata decode failed {:?}: {:?}", path, e);

                let file_metadata = fs::metadata(path.clone())?;
                let duration = 0;
                let size = file_metadata.len();
                let modified = file_metadata
                    .modified()?
                    .duration_since(UNIX_EPOCH)?
                    .as_secs();

                let title = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Unknown")
                    .to_string();

                return Ok((Track {
                    path,
                    id: track_id,
                    title,
                    artist: "Unknown Artist".to_string(),
                    album: "Unknown Album".to_string(),
                    duration,
                    modified,
                    size,
                }, None));
            }
        };

        let file_metadata = fs::metadata(path.clone())?;

        let tag = tagged_file
            .primary_tag()
            .or_else(|| tagged_file.first_tag());

        let title;
        let artist;
        let album;
        let thumbnail;

        if let Some(tag) = tag {
            title = tag
                .get_string(ItemKey::TrackTitle)
                .unwrap_or("Untitled")
                .to_string();

            let artists: Vec<String> = tag
                .get_strings(ItemKey::TrackArtist)
                .map(|s| s.to_owned())
                .collect();

            artist = if artists.is_empty() {
                "Unknown Artist".to_string()
            } else {
                artists.join(", ")
            };

            album = tag
                .get_string(ItemKey::AlbumTitle)
                .unwrap_or("Unknown Album")
                .to_string();

            thumbnail = match tag.pictures().get(0) {
                Some(data) => Some(data.data().to_vec()),
                None => None,
            };
        } else {
            title = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled")
                .to_string();

            artist = "Unknown Artist".to_string();
            album = "Unknown Album".to_string();

            thumbnail = None;
        }

        let duration = tagged_file.properties().duration().as_secs();
        let size = file_metadata.len();
        let modified = file_metadata
            .modified()?
            .duration_since(UNIX_EPOCH)?
            .as_secs();

        Ok((Track {
            path,
            id: track_id,
            title,
            artist,
            album,
            duration,
            modified,
            size,
        }, thumbnail))
    }

    fn scan_folder(
        &mut self,
        path: PathBuf,
        existing_tracks: HashSet<TrackId>,
    ) -> Result<(), ScannerError> {
        let supported = ["mp3", "flac", "wav", "ogg", "m4a"];
        let mut track_ids = vec![];
        let mut tracks = vec![];
        let mut images = vec![];
        if path.is_dir() {
            let files: Vec<PathBuf> = WalkDir::new(path.clone())
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

            let results: Vec<ScanResult> = files
                .par_iter()
                .map(|entry| {
                    let track_id = gen_track_id(entry)?;

                    if existing_tracks.contains(&track_id) {
                        Ok(ScanResult {
                            id: track_id,
                            track: None,
                            image: None,
                        })
                    } else {
                        let (track, image) =
                            self.get_track_metadata(entry.clone(), track_id.clone())?;

                        Ok(ScanResult {
                            id: track_id,
                            track: Some(track),
                            image,
                        })
                    }
                })
                .collect::<Result<_, ScannerError>>()?;
            for result in results {
                track_ids.push(result.id);

                if let Some(track) = result.track {
                    tracks.push(track);
                }

                if let Some(img) = result.image {
                    images.push((result.id, img));
                }
            }
        }

        let name = path
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .map(|s| s.to_string())
            .unwrap_or("Unnamed Playlist".to_string());

        let playlist = Playlist {
            id: PlaylistId(Uuid::new_v4()),
            name,
            source: PlaylistSource::Folder(path),
            tracks: track_ids,
        };

        let _ = self.tx.send(ScannerEvent::Tracks(tracks.clone()));
        let _ = self.tx.send(ScannerEvent::Playlist(playlist));

        let tx = self.tx.clone();

        std::thread::spawn(move || {
            let threads = std::cmp::max(1, num_cpus::get() - 2);

            let pool = ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap();
            let thumbnails: HashMap<TrackId, Arc<RenderImage>> = pool.install(|| {
                images
                    .par_iter()
                    .filter_map(|t| {
                        let img = render_album_art(&t.1, true).ok()?;
                        Some((t.0, img))
                    })
                    .collect()
            });

            let _ = tx.send(ScannerEvent::Thumbnails(thumbnails));
        }
        );

        Ok(())
    }
}

fn render_album_art(bytes: &[u8], is_thumbnail: bool) -> Result<Arc<RenderImage>, ScannerError> {
    let mut image = ImageReader::new(Cursor::new(bytes)).with_guessed_format()?.decode()?.into_rgba8();

    for px in image.pixels_mut() {
        px.0.swap(0, 2);
    }

    let frame = if is_thumbnail {
        Frame::new(thumbnail(&image, 64, 64))
    } else {
        Frame::new(image)
    };

    Ok(Arc::new(RenderImage::new(smallvec![frame])))
}