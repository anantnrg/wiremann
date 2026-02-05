use std::sync::Arc;

use crate::controller::player::Controller;
use crate::ui::theme::Theme;

use gpui::*;

#[derive(Clone)]
pub struct PlayerPage;

impl PlayerPage {
    pub fn new() -> Self {
        PlayerPage {}
    }
}

impl Render for PlayerPage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            // .child(
            //     div()
            //         .id("load")
            //         .w_40()
            //         .h_8()
            //         .flex()
            //         .items_center()
            //         .justify_center()
            //         .bg(theme.bg_queue)
            //         .rounded_md()
            //         .child("Load")
            //         .on_click(|_, _, cx| {
            //             cx.global::<Controller>().load(
            //                 "E:\\music\\violence ft. doomguy\\454 - Bad and Boujee.mp3".to_string(),
            //             )
            //         }),
            // )
            .child(
                if let Some(meta) = cx.global::<Controller>().player_state.meta.clone() {
                    if let Some(thumbnail) = meta.thumbnail {
                        let format = match thumbnail.format.as_str() {
                            "png" => ImageFormat::Png,
                            "jpeg" | "jpg" => ImageFormat::Jpeg,
                            _ => ImageFormat::Bmp,
                        };
                        div().bg(theme.white_10).size_56().child(
                            img(ImageSource::Image(Arc::new(Image::from_bytes(
                                format,
                                thumbnail.image,
                            ))))
                            .object_fit(ObjectFit::Contain)
                            .size_full(),
                        )
                    } else {
                        div()
                    }
                } else {
                    div()
                },
            )
    }
}
