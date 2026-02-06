use crate::controller::player::{Controller, Track};
use crate::ui::theme::Theme;
use ahash::AHashMap;
use gpui::prelude::FluentBuilder;
use gpui::*;
use image::{Frame, RgbaImage};
use smallvec::SmallVec;
use std::path::PathBuf;
use std::sync::Arc;

// Ref: https://github.com/hummingbird-player/hummingbird/blob/master/src/ui/queue.rs
struct ItemData {
    path: PathBuf,
    title: String,
    artists: String,
    thumbnail_bytes: Option<Arc<Vec<u8>>>,
    thumbnail: Option<Arc<RenderImage>>,
}

struct Item {
    data: ItemData,
    idx: usize,
}

impl Item {
    pub fn new(cx: &mut App, track: Arc<Track>, idx: usize) -> Entity<Self> {
        cx.new(move |cx| {
            let path = track.path.clone();
            let meta = track.meta.clone();
            let thumbnail_bytes = meta.thumbnail.clone().map(Arc::new);

            let title = meta.title.clone();
            let artists = meta.artists.clone().join(", ");

            let data = ItemData {
                path,
                title,
                artists,
                thumbnail_bytes,
                thumbnail: None,
            };

            cx.on_release(|this: &mut Item, cx| {
                if let Some(img) = this.data.thumbnail.take() {
                    drop_image_from_app(cx, img);
                }
            })
                .detach();

            Self {
                data,
                idx,
            }
        })
    }
}

impl Render for Item {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let current = cx.global::<Controller>().player_state.current.clone();

        let is_current = Some(&self.data.path) == current.as_ref();

        if self.data.thumbnail.is_none() {
            if let Some(bytes) = &self.data.thumbnail_bytes {
                let image = image::load_from_memory(bytes)
                    .ok()
                    .and_then(|i| i.as_rgba8().map(|i| i.to_owned()))
                    .unwrap_or_else(|| {
                        let mut img = RgbaImage::new(1, 1);
                        img.put_pixel(0, 0, image::Rgba([0, 0, 0, 0]));
                        img
                    });

                self.data.thumbnail = Some(Arc::new(RenderImage::new(
                    SmallVec::from_vec(vec![Frame::new(image)]),
                )));
            }
        }

        div()
            .h(px(64.))
            .w_full()
            .flex()
            .items_center()
            .p_3()
            .gap_4()
            .rounded_lg()
            .hover(|d| d.bg(theme.white_05))
            .when(is_current, |d| d.bg(theme.accent_15))
            .child(
                match &self.data.thumbnail {
                    Some(image) => div().size_12().child(
                        img(image.clone())
                            .object_fit(ObjectFit::Contain)
                            .size_full()
                            .rounded_md(),
                    ),
                    None => div().size_12(),
                }
            )
            .child(
                div()
                    .flex_col()
                    .flex_1()
                    .child(
                        div()
                            .text_base()
                            .truncate()
                            .text_color(if is_current {
                                theme.accent
                            } else {
                                theme.text_primary
                            })
                            .child(self.data.title.clone()),
                    )
                    .child(
                        div()
                            .text_sm()
                            .truncate()
                            .text_color(theme.text_muted)
                            .child(self.data.artists.clone()),
                    ),
            )
    }
}


#[derive(Clone)]
pub struct Queue {
    views: Entity<AHashMap<usize, Entity<Item>>>,
    scroll_handle: UniformListScrollHandle,
}

impl Queue {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            Self {
                views: cx.new(|_| AHashMap::new()),
                scroll_handle: UniformListScrollHandle::new(),
            }
        })
    }

    fn get_or_create_item(
        views: &Entity<AHashMap<usize, Entity<Item>>>,
        index: usize,
        track: Arc<Track>,
        cx: &mut App,
    ) -> Entity<Item> {
        views.update(cx, |this, cx| {
            this.entry(index)
                .or_insert_with(|| Item::new(cx, track, index))
                .clone()
        })
    }
}

pub fn drop_image_from_app(cx: &mut App, image: Arc<RenderImage>) {
    for window in cx.windows() {
        let image = image.clone();

        window
            .update(cx, move |_, window, _| {
                window.drop_image(image).expect("Could not drop image");
            })
            .expect("Couldn't get window");
    }
}

impl Render for Queue {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let views = self.views.clone();
        let playlist = cx
            .global::<Controller>()
            .scanner_state
            .current_playlist
            .as_ref();

        let tracks: &[Track] = match playlist {
            Some(p) => &p.tracks,
            None => &[],
        };

        let len = tracks.len();

        uniform_list("queue", len, move |range, _, cx| {
            views.update(cx, |map, cx| {
                map.retain(|idx, _| range.contains(idx));
            });

            range
                .map(|i| {
                    let track = Arc::new(tracks[i].clone());
                    div().child(Queue::get_or_create_item(
                        &views,
                        i,
                        track,
                        cx,
                    ))
                })
                .collect()
        })
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .track_scroll(&self.scroll_handle)
    }
}

