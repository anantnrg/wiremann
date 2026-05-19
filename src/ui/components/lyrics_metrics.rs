use cosmic_text::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping, SwashCache};

use gpui::Pixels;

use std::collections::HashMap;

pub struct LyricsMetrics {
    pub font_system: FontSystem,
    pub swash_cache: SwashCache,
    pub cache: HashMap<CacheKey, Pixels>,
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct CacheKey {
    pub text: String,
    pub width: u32,
    pub font_size: u32,
}

impl LyricsMetrics {
    pub fn new() -> Self {
        Self {
            font_system: FontSystem::new(),
            swash_cache: SwashCache::new(),
            cache: HashMap::new(),
        }
    }
}

impl LyricsMetrics {
    pub fn measure_height(&mut self, text: &str, width: f32, font_size: f32) -> Pixels {
        let key = CacheKey {
            text: text.to_string(),
            width: width as u32,
            font_size: font_size as u32,
        };

        if let Some(cached) = self.cache.get(&key) {
            return *cached;
        }

        let metrics = Metrics::new(font_size, font_size * 1.25);

        let mut buffer = Buffer::new(&mut self.font_system, metrics);

        buffer.set_size(Some(width), None);

        buffer.set_text(
            text,
            &Attrs::new().family(Family::SansSerif),
            Shaping::Advanced,
            None,
        );

        buffer.shape_until_scroll(&mut self.font_system, false);

        let lines = buffer.layout_runs().count();

        let height = lines as f32 * metrics.line_height;

        let px_height = gpui::px(height);

        self.cache.insert(key, px_height);

        px_height
    }
}
