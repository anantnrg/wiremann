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
    thumbnail: Option<Arc<RenderImage>>,
}

struct Item {
    data: ItemData,
    current: Option<PathBuf>,
    idx: usize,
}

impl Item {
    pub fn new(cx: &mut App, track: Arc<Track>, idx: usize) -> Entity<Self> {
        cx.new(move |cx| {
            let path = track.path.clone();
            let meta = track.meta.clone();
            let thumbnail = meta.thumbnail.as_ref().map(|thumbnail| {
                Arc::new(RenderImage::new(SmallVec::from_vec(vec![Frame::new(image::load_from_memory(&thumbnail).unwrap()
                    .as_rgba8()
                    .map(|image| image.to_owned())
                    .unwrap_or_else(|| {
                        let mut image = RgbaImage::new(1, 1);
                        image.put_pixel(0, 0, image::Rgba([0, 0, 0, 0]));
                        image
                    }))])))
            });
            let title = meta.title.clone();
            let artists = meta.artists.clone().join(", ");

            let data = ItemData {
                path,
                title,
                artists,
                thumbnail,
            };

            cx.on_release(|this: &mut Item, cx| {
                if let Some(img) = this.data.thumbnail.take() {
                    drop_image_from_app(cx, img);
                }
            })
                .detach();

            let current = cx.global::<Controller>().player_state.current.clone();

            Self {
                data,
                current,
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
                    Some(img) => div().size_12().child(
                        img(img.clone())
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
                            .child(self.data.artists),
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

// impl Render for Queue {
//     fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
//         v_virtual_list(
//             cx.entity().clone(),
//             "queue_list",
//             self.item_sizes.clone(),
//             move |view, visible_range, _, cx| {
//                 let images: Vec<Option<Arc<Image>>> = visible_range
//                     .clone()
//                     .map(|ix| {
//                         let track = &view.items[ix];
//                         get_or_create_image(cx, track)
//                     })
//                     .collect();
//
//                 let theme = cx.global::<Theme>();
//                 let current = cx.global::<Controller>().player_state.current.clone();
//
//                 visible_range
//                     .zip(images.into_iter())
//                     .map(|(ix, image)| {
//                         let track = &view.items[ix];
//                         let meta = &track.meta;
//                         div()
//                             .id(ix)
//                             .h(px(64.))
//                             .w_full()
//                             .flex()
//                             .items_center()
//                             .justify_start()
//                             .p_3()
//                             .gap_4()
//                             .rounded_lg()
//                             .mb_2()
//                             .hover(|this| this.bg(theme.white_05))
//                             .when(Some(&track.path) == current.as_ref(), |this| {
//                                 this.bg(theme.accent_15)
//                             })
//                             .child({
//                                 match image {
//                                     Some(image) => {
//                                         div().size_12().child(
//                                             img(image)
//                                                 .object_fit(ObjectFit::Contain)
//                                                 .size_full()
//                                                 .rounded_md(),
//                                         )
//                                     }
//                                     None => div().size_12(),
//                                 }
//                             })
//                             .child(
//                                 div()
//                                     .w_auto()
//                                     .h_12()
//                                     .flex()
//                                     .flex_col()
//                                     .flex_1()
//                                     .gap_y_2()
//                                     .child(div().text_base().truncate().text_color(
//                                         if Some(&track.path) == current.as_ref() {
//                                             theme.accent
//                                         } else {
//                                             theme.text_primary
//                                         }
//                                         ,
//                                     ).child(meta.title.clone()))
//                                     .child(div().text_sm().truncate().text_color(theme.text_muted).child(meta.artists.join(", "))),
//                             )
//                     })
//                     .collect()
//             },
//         )
//             .track_scroll(&self.scroll_handle)
//     }
// }
