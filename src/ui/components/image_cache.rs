use crate::controller::player::Track;
use crate::ui::components::pages::player::get_img_format;
use ahash::AHasher;
use gpui::*;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Default)]
pub struct ImageCache {
    map: HashMap<u64, Arc<Image>>,
}

impl ImageCache {
    pub fn new() -> Self {
        Self {
            map: HashMap::default(),
        }
    }
}

fn image_key(path: &PathBuf) -> u64 {
    let mut h = AHasher::default();
    path.hash(&mut h);
    h.finish()
}

pub fn get_or_create_image(
    cx: &mut App,
    track: &Track,
) -> Option<Arc<Image>> {
    let thumbnail = track.meta.thumbnail.as_ref()?;

    let key = image_key(&track.path);
    let cache = cx.global_mut::<ImageCache>();

    if let Some(img) = cache.map.get(&key) {
        return Some(img.clone());
    }

    let image = Arc::new(Image::from_bytes(
        get_img_format(thumbnail.format.clone()),
        thumbnail.image.clone(),
    ));

    cache.map.insert(key, image.clone());
    Some(image)
}

impl Global for ImageCache {}
