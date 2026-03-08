use crate::{
    controller::Controller,
    ui::{
        components::image_cache::ImageCache

        ,
        theme::Theme,
    },
};
use gpui::{div, App, AppContext, Context, Entity, ImageFormat, IntoElement, ParentElement, Render, Styled, UniformListScrollHandle, Window};

#[derive(Clone)]
pub struct LibraryPage {
    scroll_handle: UniformListScrollHandle,
    show_playlists: Entity<bool>,
}

impl LibraryPage {
    pub fn new(cx: &mut App) -> Self {
        let scroll_handle = UniformListScrollHandle::new();
        let show_playlists = cx.new(|_| true);

        LibraryPage {
            scroll_handle,
            show_playlists,
        }
    }
}

impl Render for LibraryPage {
    #[allow(clippy::too_many_lines)]
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let controller = cx.global::<Controller>().clone();
        let state = controller.state.read(cx);
        let thumbnail = cx.global::<ImageCache>().current.clone();
        // let scanner_state = cx.global::<Controller>().scanner_state.clone();
        let scroll_handle = self.scroll_handle.clone();
        let show_playlists = self.show_playlists.clone();

        let current = if let Some(id) = state.playback.current {
            state.library.tracks.get(&id)
        } else {
            None
        };

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
                    .pt_8()
                    .pb_2()
            )
    }
}

#[must_use]
pub fn get_img_format(format: &str) -> ImageFormat {
    match format {
        "png" => ImageFormat::Png,
        "jpeg" | "jpg" => ImageFormat::Jpeg,
        _ => ImageFormat::Bmp,
    }
}
