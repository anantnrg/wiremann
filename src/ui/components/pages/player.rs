use crate::ui::theme::Theme;

use crate::controller::player::Controller;
use crate::ui::components::queue::Queue;
use crate::ui::components::scrollbar::{floating_scrollbar, RightPad};
use gpui::*;

#[derive(Clone)]
pub struct PlayerPage {
    pub queue: Entity<Queue>,
    queue_scroll_handle: UniformListScrollHandle,
}

impl PlayerPage {
    pub fn new(cx: &mut App) -> Self {
        let queue_scroll_handle = UniformListScrollHandle::new();
        PlayerPage {
            queue: Queue::new(cx, queue_scroll_handle.clone()),
            queue_scroll_handle,
        }
    }
}

impl Render for PlayerPage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let player_state = cx.global::<Controller>().player_state.clone();
        let thumbnail = player_state.thumbnail;
        let scanner_state = cx.global::<Controller>().scanner_state.clone();
        let scroll_handle = self.queue_scroll_handle.clone();

        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .h_full()
                    .w_full()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .px_16()
                    .py_8()
                    .child(if let Some(meta) = player_state.meta {
                        div()
                            .w_auto()
                            .h_auto()
                            .flex()
                            .flex_col()
                            .items_center()
                            .justify_center()
                            .gap_y_6()
                            .flex_shrink_0()
                            .child(if let Some(thumbnail) = thumbnail {
                                div().size_96().child(
                                    img(thumbnail)
                                        .object_fit(ObjectFit::Contain)
                                        .size_full()
                                        .rounded_xl(),
                                )
                            } else {
                                div().size_full()
                            })
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_y_1()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        div()
                                            .text_2xl()
                                            .text_color(theme.text_primary)
                                            .font_weight(FontWeight(500.0))
                                            .max_w_96()
                                            .truncate()
                                            .child(meta.title.clone()),
                                    )
                                    .child(
                                        div()
                                            .text_base()
                                            .text_color(theme.text_muted)
                                            .font_weight(FontWeight(400.0))
                                            .max_w_96()
                                            .truncate()
                                            .child(meta.artists.join(", ").clone()),
                                    ),
                            )
                    } else {
                        div()
                    })
                    .child(div().w_full().h_auto().flex().flex_shrink_0().gap_x_6().items_center().justify_center()),
            )
            .child(div().w(px(1.0)).h_full().bg(theme.white_05))
            .child(
                div()
                    .h_full()
                    .w_80()
                    .flex_shrink_0()
                    .flex()
                    .flex_col()
                    .bg(theme.bg_queue)
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .items_center()
                            .justify_between()
                            .p_4()
                            .child(
                                div()
                                    .text_base()
                                    .text_color(theme.text_primary)
                                    .font_weight(FontWeight(500.0))
                                    .child("Queue"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight(400.0))
                                    .text_color(theme.text_muted)
                                    .child("Hide"),
                            ),
                    )
                    .child(div()
                        .id("queue_container")
                        .w_full()
                        .h_full()
                        .p_4()
                        .flex()
                        .relative()
                        .child(self.queue.clone())
                        .child(floating_scrollbar(
                            "queue_scrollbar",
                            scroll_handle,
                            RightPad::None,
                        )))
                ,
            )
    }
}

pub fn get_img_format(format: String) -> ImageFormat {
    match format.as_str() {
        "png" => ImageFormat::Png,
        "jpeg" | "jpg" => ImageFormat::Jpeg,
        _ => ImageFormat::Bmp,
    }
}
