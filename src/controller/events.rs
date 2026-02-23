use std::collections::HashMap;
use crate::controller::state::PlaybackStatus;
use crate::library::playlists::Playlist;
use crate::library::{Track, TrackId};
use std::path::PathBuf;
use std::sync::Arc;
use gpui::RenderImage;

#[derive(Clone, PartialEq, Debug)]
pub enum AudioEvent {
    TrackLoaded(PathBuf),
    Position(u64),
    PlaybackStatus(PlaybackStatus),
    Volume(f32),
    TrackEnded,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ScannerEvent {
    Tracks(Vec<Track>),
    Playlist(Playlist),
    AlbumArt(Arc<RenderImage>),
    Thumbnails(HashMap<TrackId, Arc<RenderImage>>),
}
