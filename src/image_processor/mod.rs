use std::sync::Arc;

use fast_image_resize as fr;
use garb::bytes::rgba_to_bgra_inplace;
use gpui::{ImageId, RenderImage};
use image::{DynamicImage, EncodableLayout, Frame, RgbaImage, imageops};
use smallvec::smallvec;

use crate::{cacher::ImageKind, errors::ScannerError};
pub struct ImageProcessor {}

fn render_album_art(
    bytes: &[u8],
    kind: ImageKind,
    resizer: &mut fr::Resizer,
) -> Result<Arc<RenderImage>, ScannerError> {
    let raw_img = image::load_from_memory(bytes)?;

    let image = match kind {
        ImageKind::AlbumArt => {
            let mut rgba = raw_img.into_rgba8();
            rgba_to_bgra_inplace(rgba.as_mut())?;
            rgba
        }
        ImageKind::ThumbnailSmall | ImageKind::ThumbnailLarge => {
            let (new_w, new_h) = match kind {
                ImageKind::ThumbnailSmall => (128, 128),
                ImageKind::ThumbnailLarge => (256, 256),
                _ => unreachable!(),
            };

            let mut dst = fr::images::Image::new(new_w, new_h, fr::PixelType::U8x4);

            resizer.resize(
                &raw_img,
                &mut dst,
                &fr::ResizeOptions::new()
                    .resize_alg(fr::ResizeAlg::Convolution(fr::FilterType::Bilinear)),
            )?;

            let mut buf = dst.into_vec();
            rgba_to_bgra_inplace(&mut buf)?;

            RgbaImage::from_raw(new_w, new_h, buf).unwrap()
        }
        _ => unreachable!(),
    };

    let frame = Frame::new(image);

    Ok(Arc::new(RenderImage::new(smallvec![frame])))
}

fn render_playlist_thumbnail(
    mut images: Vec<DynamicImage>,
) -> (Option<Arc<RenderImage>>, Option<ImageId>) {
    let mut canvas = DynamicImage::new_rgba8(256, 256);

    match images.len() {
        1 => {
            let img = images
                .remove(0)
                .resize_exact(256, 256, imageops::FilterType::Lanczos3);

            imageops::overlay(&mut canvas, &img, 0, 0);
        }

        2 => {
            for (i, img) in images.into_iter().enumerate() {
                let resized = img.resize_exact(128, 256, imageops::FilterType::Lanczos3);
                imageops::overlay(&mut canvas, &resized, (i * 128) as i64, 0);
            }
        }

        3 => {
            let a = images
                .remove(0)
                .resize_exact(128, 128, imageops::FilterType::Lanczos3);
            let b = images
                .remove(0)
                .resize_exact(128, 128, imageops::FilterType::Lanczos3);
            let c = images
                .remove(0)
                .resize_exact(256, 128, imageops::FilterType::Lanczos3);

            imageops::overlay(&mut canvas, &a, 0, 0);
            imageops::overlay(&mut canvas, &b, 128, 0);
            imageops::overlay(&mut canvas, &c, 0, 128);
        }

        _ => {
            for (i, img) in images.into_iter().take(4).enumerate() {
                let resized = img.resize_exact(128, 128, imageops::FilterType::Lanczos3);

                let x = (i % 2) * 128;
                let y = (i / 2) * 128;

                imageops::overlay(&mut canvas, &resized, x as i64, y as i64);
            }
        }
    }

    let mut image = canvas.to_rgba8();

    let hash = if let Ok(hash) = ImageId::generate(image.as_bytes()) {
        Some(hash)
    } else {
        None
    };

    rgba_to_bgra_inplace(image.as_mut()).ok();

    let frame = Frame::new(image);

    let render_image = Arc::new(RenderImage::new(smallvec![frame]));

    (Some(render_image), hash)
}
