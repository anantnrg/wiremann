use crate::controller::player::{Controller, Track};
use crate::ui::image_cache::ImageCache;
use crate::ui::theme::Theme;
use ahash::AHashMap;
use gpui::prelude::FluentBuilder;
use gpui::*;
use std::path::PathBuf;
use std::sync::Arc;

// Ref: https://github.com/hummingbird-player/hummingbird/blob/master/src/ui/queue.rs
struct ItemData {
    path: PathBuf,
    title: String,
    artists: String,
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

            let title = meta.title.clone();
            let artists = meta.artists.clone().join(", ");

            let data = ItemData {
                path,
                title,
                artists,
            };

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

        let thumbnail = cx.global::<ImageCache>().get(&self.data.path);
        div()
            .h(px(64.))
            .w_full()
            .flex()
            .items_center()
            .p_3()
            .gap_4()
            .mb_2()
            .rounded_lg()
            .hover(|d| d.bg(theme.white_05))

            .when(is_current, |d| d.bg(theme.accent_15))
            .child(
                match thumbnail {
                    Some(image) => div().size_12().flex_shrink_0().child(
                        img(image.clone())
                            .object_fit(ObjectFit::Contain)
                            .size_full()
                            .rounded_md(),
                    ),
                    None => div().size_12().flex_shrink_0(),
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

impl Render for Queue {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let views = self.views.clone();
        let playlist = cx
            .global::<Controller>()
            .scanner_state
            .current_playlist
            .as_ref();

        let tracks: Arc<Vec<Track>> = Arc::new(match playlist {
            Some(p) => p.tracks.clone(),
            None => Vec::new(),
        }).clone();
        let len = tracks.len();

        uniform_list("queue", len, move |range, _, cx| {
            views.update(cx, |map, cx| {
                map.retain(|idx, _| range.contains(idx));
            });

            range
                .map(|i| {
                    let track = Arc::new(tracks[i].clone());
                    let path = track.path.clone();

                    div()
                        .id(format!("track_{}", path.to_string_lossy().to_string()))
                        .child(Queue::get_or_create_item(
                            &views,
                            i,
                            track,
                            cx,
                        ))
                        .on_click(move |_, _, cx|
                            {
                                cx.global::<Controller>().load(path.to_string_lossy().to_string())
                            }
                        )
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

