use crate::ui::theme::Theme;

use crate::controller::player::Controller;
use crate::ui::components::queue::Queue;
use gpui::*;

#[derive(Clone)]
pub struct PlayerPage {
    pub queue: Entity<Queue>,
}

impl PlayerPage {
    pub fn new(cx: &mut App) -> Self {
        PlayerPage {
            queue: Queue::new(cx),
        }
    }
}

impl Render for PlayerPage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let player_state = cx.global::<Controller>().player_state.clone();
        let thumbnail = player_state.thumbnail;
        let scanner_state = cx.global::<Controller>().scanner_state.clone();

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
                    .py_12()
                    .child(if let Some(meta) = player_state.meta {
                        div()
                            .w_auto()
                            .h_auto()
                            .flex()
                            .flex_col()
                            .items_center()
                            .justify_center()
                            .gap_y_8()
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
                                    .gap_y_2()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        div()
                                            .text_2xl()
                                            .text_color(theme.text_primary)
                                            .font_weight(FontWeight(500.0))
                                            .child(meta.title.clone()),
                                    )
                                    .child(
                                        div()
                                            .text_base()
                                            .text_color(theme.text_muted)
                                            .font_weight(FontWeight(400.0))
                                            .child(meta.artists.join(", ").clone()),
                                    ),
                            )
                    } else {
                        div()
                    }),
            )
            .child(div().w(px(1.0)).h_full().bg(theme.white_05))
            .child(
                div()
                    .h_full()
                    .w_80()
                    .flex_shrink_0()
                    .flex()
                    .flex_col()
                    .p_4()
                    .bg(theme.bg_queue)
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .items_center()
                            .justify_between()
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
                    ), // .child(self.queue.clone()),
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
