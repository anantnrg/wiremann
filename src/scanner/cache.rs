use crate::controller::metadata::Metadata;
use bitcode::{Decode, Encode};
use std::path::PathBuf;
use uuid::Uuid;
use crate::controller::player::Track;
use crate::scanner::Playlist;

pub struct CacheManager {
    cache_dir: PathBuf,
}

#[derive(Clone, Encode, Decode)]
pub struct PlaylistCache {
    pub id: String,
    pub name: String,
    pub path: String,
    pub tracks: Vec<TrackCache>,
}

#[derive(Clone, Encode, Decode)]
pub struct TrackCache {
    pub path: String,
    pub meta: MetadataCache,
}

#[derive(Clone, Encode, Decode)]
pub struct MetadataCache {
    pub title: String,
    pub artists: Vec<String>,
    pub album: String,
    pub genre: String,
    pub duration: u64,
    pub writer: String,
    pub producer: String,
    pub publisher: String,
    pub label: String,
}

impl From<Playlist> for PlaylistCache {
    fn from(value: Playlist) -> Self {
        PlaylistCache {
            id: value.id.to_string(),
            name: value.name,
            path: value.path.unwrap_or(PathBuf::new()).to_string_lossy().to_string(),
            tracks: value.tracks.into_iter().map(TrackCache::from).collect(),
        }
    }
}

impl From<PlaylistCache> for Playlist {
    fn from(value: PlaylistCache) -> Self {
        Playlist {
            id: Uuid::from(value.id),
            name: value.name,
            path: Some(PathBuf::from(value.path)),
            tracks: value.tracks.into_iter().map(Track::from).collect(),
        }
    }
}

impl From<Track> for TrackCache {
    fn from(value: Track) -> Self {
        TrackCache {
            path: value.path.to_string_lossy().to_string(),
            meta: value.meta.into(),
        }
    }
}

impl From<TrackCache> for Track {
    fn from(value: TrackCache) -> Self {
        Track {
            path: PathBuf::from(value.path),
            meta: value.meta.into(),
        }
    }
}


impl From<Metadata> for MetadataCache {
    fn from(value: Metadata) -> Self {
        MetadataCache {
            title: value.title,
            artists: value.artists,
            album: value.album,
            genre: value.genre,
            duration: value.duration,
            writer: value.writer,
            producer: value.producer,
            publisher: value.publisher,
            label: value.label,
        }
    }
}

impl From<MetadataCache> for Metadata {
    fn from(value: MetadataCache) -> Self {
        Metadata {
            title: value.title,
            artists: value.artists,
            album: value.album,
            genre: value.genre,
            duration: value.duration,
            writer: value.writer,
            producer: value.producer,
            publisher: value.publisher,
            label: value.label,
            thumbnail: None,
        }
    }
}