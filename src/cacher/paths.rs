use crate::cacher::ImageKind;
use crate::controller::state::ImageId;
use std::path::{Path, PathBuf};

pub struct CachePaths;

impl CachePaths {
    pub fn image_cache_path(cache_dir: &Path, id: ImageId, kind: ImageKind) -> PathBuf {
        let hex = hex::encode(id.0);
        let folder = &hex[0..2];
        let name = Self::image_filename(&hex, kind);

        cache_dir.join("images").join(folder).join(name)
    }

    pub fn image_filename(hex: &str, kind: ImageKind) -> String {
        match kind {
            ImageKind::ThumbnailSmall => format!("{hex}_tmbhs.rgba.zstd"),
            ImageKind::ThumbnailLarge => format!("{hex}_tmbhl.rgba.zstd"),
            ImageKind::AlbumArt => format!("{hex}_art.rgba.zstd"),
            ImageKind::Playlist => format!("{hex}_playlist.rgba.zstd"),
        }
    }

    pub fn temp_file_path(final_path: &Path) -> PathBuf {
        final_path.with_extension("tmp")
    }
}
