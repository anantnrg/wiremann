use crate::controller::player::Track;
use gpui::*;
use gpui_component::{
    v_virtual_list, VirtualListScrollHandle
    ,
};
use std::rc::Rc;

#[derive(Clone)]
pub struct Queue {
    items: Vec<Track>,
    item_sizes: Rc<Vec<Size<Pixels>>>,
    scroll_handle: VirtualListScrollHandle,
}

impl Queue {
    pub fn new() -> Self {
        let item_sizes = Rc::new(vec![size(px(320.), px(30.))]);

        Self {
            items: vec![],
            item_sizes,
            scroll_handle: VirtualListScrollHandle::new(),
        }
    }

    pub fn update_items(&mut self, items: Vec<Track>) {
        self.item_sizes = Rc::new(items.iter().map(|_| size(px(320.), px(30.))).collect());
    }
}

impl Render for Queue {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_virtual_list(
            cx.entity().clone(),
            "queue_list",
            self.item_sizes.clone(),
            |view, visible_range, _, cx| {
                visible_range
                    .map(|ix| {
                        div()
                            .h(px(30.))
                            .w_full()
                            .child(format!("Item {}", ix))
                    })
                    .collect()
            },
        )
            .track_scroll(&self.scroll_handle)
    }
}