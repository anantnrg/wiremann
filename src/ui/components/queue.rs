use crate::controller::player::{Controller, Track};
use crate::ui::components::image_cache::get_or_create_image;
use crate::ui::theme::Theme;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{v_virtual_list, VirtualListScrollHandle};
use std::rc::Rc;
use std::sync::Arc;

#[derive(Clone)]
pub struct Queue {
    items: Vec<Arc<Track>>,
    item_sizes: Rc<Vec<Size<Pixels>>>,
    scroll_handle: VirtualListScrollHandle,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            items: vec![],
            item_sizes: Rc::new(vec![]),
            scroll_handle: VirtualListScrollHandle::new(),
        }
    }

    pub fn update_items(&mut self, items: Vec<Track>) {
        self.item_sizes = Rc::new(items.iter().map(|_| size(px(320.), px(64.))).collect());
        self.items = items.into_iter().map(Arc::new).collect();
    }
}

impl Render for Queue {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_virtual_list(
            cx.entity().clone(),
            "queue_list",
            self.item_sizes.clone(),
            move |view, visible_range, _, cx| {
                let images: Vec<Option<Arc<Image>>> = visible_range
                    .clone()
                    .map(|ix| {
                        let track = &view.items[ix];
                        get_or_create_image(cx, track)
                    })
                    .collect();

                let theme = cx.global::<Theme>();
                let current = cx.global::<Controller>().player_state.current.clone();

                visible_range
                    .zip(images.into_iter())
                    .map(|(ix, image)| {
                        let track = &view.items[ix];
                        let meta = &track.meta;
                        div()
                            .id(ix)
                            .h(px(64.))
                            .w_full()
                            .flex()
                            .items_center()
                            .justify_start()
                            .p_3()
                            .gap_4()
                            .rounded_lg()
                            .mb_2()
                            .hover(|this| this.bg(theme.white_05))
                            .when(Some(&track.path) == current.as_ref(), |this| {
                                this.bg(theme.accent_15)
                            })
                            .child({
                                match image {
                                    Some(image) => {
                                        div().size_12().child(
                                            img(image)
                                                .object_fit(ObjectFit::Contain)
                                                .size_full()
                                                .rounded_md(),
                                        )
                                    }
                                    None => div().size_12(),
                                }
                            })
                            .child(
                                div()
                                    .w_auto()
                                    .h_12()
                                    .flex()
                                    .flex_col()
                                    .flex_1()
                                    .gap_y_2()
                                    .child(div().text_base().truncate().text_color(
                                        if Some(&track.path) == current.as_ref() {
                                            theme.accent
                                        } else {
                                            theme.text_primary
                                        }
                                        ,
                                    ).child(meta.title.clone()))
                                    .child(div().text_sm().truncate().text_color(theme.text_muted).child(meta.artists.join(", "))),
                            )
                    })
                    .collect()
            },
        )
            .track_scroll(&self.scroll_handle)
    }
}
