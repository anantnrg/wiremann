use ahash::AHashMap;
use gpui::RenderImage;
use std::path::PathBuf;
use std::sync::Arc;

pub struct ImageCache {
    thumbs: AHashMap<PathBuf, Arc<RenderImage>>,
}

impl Default for ImageCache {
    fn default() -> Self {
        Self {
            thumbs: AHashMap::new(),
        }
    }
}

impl ImageCache {
    pub fn get(&self, path: &PathBuf) -> Option<Arc<RenderImage>> {
        if let Some(thumbnail) = self.thumbs.get(path) {
            Some(thumbnail.clone())
        } else {
            None
        }
    }
}

impl gpui::Global for ImageCache {}