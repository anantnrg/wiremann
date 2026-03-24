use crate::library::playlists::PlaylistId;
use crate::library::TrackId;
use gpui::RenderImage;
use std::sync::Arc;

#[must_use]
#[allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss
)]
pub fn slider_to_secs(slider: f32, duration_secs: u64) -> u64 {
    ((slider.clamp(0.0, 100.0) / 100.0) * duration_secs as f32) as u64
}

#[must_use]
#[allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss
)]
pub fn secs_to_slider(pos: u64, duration: u64) -> f32 {
    if duration == 0 {
        0.0
    } else {
        (pos as f32 / duration as f32) * 100.0
    }
}

pub fn drop_image_from_app(cx: &mut gpui::App, image: Arc<RenderImage>) {
    cx.defer(move |cx| {
        for window in cx.windows() {
            let image = image.clone();

            window
                .update(cx, move |_, window, _| {
                    let _ = window.drop_image(image);
                })
                .ok();
        }
    });
}

pub fn fingerprint_tracks(ids: impl IntoIterator<Item=TrackId>) -> u128 {
    let mut acc = 0u128;

    for id in ids {
        acc ^= u128::from_le_bytes(id.0);
    }

    acc
}

pub fn fingerprint_playlists(ids: impl IntoIterator<Item=PlaylistId>) -> u128 {
    let mut acc = 0u128;

    for id in ids {
        acc ^= u128::from_le_bytes(*id.0.as_bytes());
    }

    acc
}
