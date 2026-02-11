use crate::controller::metadata::Metadata;
use crate::controller::player::Track;
use crate::scanner::Playlist;
use ahash::AHashMap;
use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use uuid::Uuid;

pub struct CacheManager {
    cache_dir: PathBuf,
    pub playlist_indexes: CachedPlaylistIndexes,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct CachedPlaylistIndexes {
    pub playlists: Vec<CachedPlaylistIndex>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CachedPlaylistIndex {
    pub id: String,
    pub name: String,
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

#[derive(Clone, Encode, Decode)]
pub struct ThumbnailsCached {
    pub thumbnails: AHashMap<String, Vec<u8>>,
}

impl From<&Playlist> for PlaylistCache {
    fn from(value: &Playlist) -> Self {
        PlaylistCache {
            id: value.id.to_string(),
            name: value.name,
            path: value
                .path
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default(),
            tracks: value.tracks.into_iter().map(TrackCache::from).collect(),
        }
    }
}

impl From<PlaylistCache> for Playlist {
    fn from(value: PlaylistCache) -> Self {
        Playlist {
            id: Uuid::parse_str(&value.id).expect("invalid uuid in cache"),
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

impl CacheManager {
    pub fn init() -> Self {
        let cache_dir = dirs::audio_dir().unwrap().join("wiremann").join("cache");

        let playlist_indexes: CachedPlaylistIndexes =
            match File::open(cache_dir.join("playlists.ron")) {
                Ok(mut file) => {
                    let mut playlists = String::new();
                    file.read_to_string(&mut playlists)
                        .expect("couldnt read to string");
                    ron::from_str(&playlists).unwrap_or_default()
                }
                Err(_) => CachedPlaylistIndexes::default(),
            };

        CacheManager {
            cache_dir,
            playlist_indexes,
        }
    }

    pub fn write_playlist(&mut self, playlist: Playlist, thumbnails: Vec<(PathBuf, Vec<u8>)>) {
        let base = match dirs::audio_dir() {
            Some(dir) => dir,
            None => return,
        };

        let cache_dir = base
            .join("wiremann")
            .join("cache")
            .join(playlist.id.to_string());

        fs::create_dir_all(&cache_dir).expect("couldnt create cache dir");

        let playlist: PlaylistCache = playlist.into();

        let mut thumbnails_cached = ThumbnailsCached { thumbnails: AHashMap::new() };

        for (path, image) in thumbnails {
            thumbnails_cached
                .thumbnails
                .insert(
                    path.to_string_lossy().to_string(),
                    image,
                );
        }

        let playlist_encoded = bitcode::encode(&playlist);
        let thumbnails_encoded = bitcode::encode(&thumbnails_cached);

        let tracks_tmp = cache_dir.join("tracks.tmp");
        let tracks_final = cache_dir.join("tracks.bin");

        let thumbnails_tmp = cache_dir.join("thumbnails.tmp");
        let thumbnails_final = cache_dir.join("thumbnails.bin");

        fs::write(&tracks_tmp, &playlist_encoded).expect("write failed");
        fs::rename(&tracks_tmp, &tracks_final).expect("rename failed");

        fs::write(&thumbnails_tmp, &thumbnails_encoded).expect("write failed");
        fs::rename(&thumbnails_final, &tracks_final).expect("rename failed");
    }
}
