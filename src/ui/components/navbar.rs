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
        let page = *cx.global::<Page>();

        let active_highlight_offset = match page {
            Page::Library => 0.0,
            Page::Player => 96.0,
            Page::Playlists => 192.0,
        };

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
                    .id("active_highlight")
                    .absolute()
                    .top_0()
                    .left(px(active_highlight_offset))
                    .h_full()
                    .w_24()
                    .rounded_full()
                    .bg(theme.switcher_active),
            )
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
                    .font_weight(FontWeight::MEDIUM)
                    .hover(|this| this.text_color(theme.switcher_text_hover))
                    .on_click(|_, _, cx| *cx.global_mut::<Page>() = Page::Library)
                    .when(page == Page::Library, |this| {
                        this.text_color(theme.switcher_text_active)
                    })
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
                    .font_weight(FontWeight::MEDIUM)
                    .hover(|this| this.text_color(theme.switcher_text_hover))
                    .on_click(|_, _, cx| *cx.global_mut::<Page>() = Page::Player)
                    .when(page == Page::Player, |this| {
                        this.text_color(theme.switcher_text_active)
                    })
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
                    .font_weight(FontWeight::MEDIUM)
                    .on_click(|_, _, cx| *cx.global_mut::<Page>() = Page::Playlists)
                    .hover(|this| this.text_color(theme.switcher_text_hover))
                    .when(page == Page::Playlists, |this| {
                        this.text_color(theme.switcher_text_active)
                    })
                    .child("Playlists"),
            )
    }
}
