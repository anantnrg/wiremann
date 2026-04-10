use super::Page;
use crate::ui::theme::Theme;

use gpui::prelude::FluentBuilder;
use gpui::{
    Context, FontWeight, InteractiveElement, IntoElement, ParentElement, Render,
    StatefulInteractiveElement, Styled, Window, div, px,
};

#[derive(Clone)]
pub struct NavBar;

impl NavBar {
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self {
        NavBar {}
    }
}

impl Render for NavBar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let page = cx.global::<Page>();

        div()
            .flex()
            .w_auto()
            .h_full()
            .rounded_full()
            .items_center()
            .justify_center()
            .bg(theme.switcher_bg)
            .border_1()
            .border_color(theme.border)
            .child(
                div()
                    .id("library")
                    .h_full()
                    .w_24()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_sm()
                    .text_color(theme.switcher_text)
                    .hover(|this| this.text_color(theme.switcher_text_hover))
                    .on_click(|_, _, cx| *cx.global_mut::<Page>() = Page::Library)
                    .child("Library"),
            )
            .child(
                div()
                    .id("player")
                    .h_full()
                    .w_24()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_sm()
                    .text_color(theme.switcher_text)
                    .hover(|this| this.text_color(theme.switcher_text_hover))
                    .on_click(|_, _, cx| *cx.global_mut::<Page>() = Page::Player)
                    .child("Player"),
            )
            .child(
                div()
                    .id("playlists")
                    .h_full()
                    .w_24()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_sm()
                    .text_color(theme.switcher_text)
                    .on_click(|_, _, cx| *cx.global_mut::<Page>() = Page::Playlists)
                    .hover(|this| this.text_color(theme.switcher_text_hover))
                    .child("Playlists"),
            )
    }
}
