use super::Page;
use crate::ui::theme::Theme;

use crate::ui::icons::Icons;
use gpui::*;
use gpui_component::Icon;

#[derive(Clone)]
pub struct NavBar;

impl NavBar {
    pub fn new() -> Self {
        NavBar {}
    }
}

impl Render for NavBar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let page = cx.global::<Page>();

        div()
            .h_full()
            .w_16()
            .flex()
            .flex_col()
            .flex_shrink_0()
            .items_center()
            .bg(theme.panel)
            .py_2()
            .gap_2()
            .border_r_1()
            .border_color(theme.border)
            .child(
                div()
                    .id("home")
                    .size_12()
                    .rounded_md()
                    .flex()
                    .flex_shrink_0()
                    .items_center()
                    .justify_center()
                    .bg(if page == &Page::Home {
                        theme.accent
                    } else {
                        theme.bg
                    })
                    .hover(|this| {
                        if page != &Page::Home {
                            this.bg(theme.highlighted)
                        } else {
                            this.bg(theme.accent)
                        }
                    })
                    .on_click(|_, _, cx| *cx.global_mut::<Page>() = Page::Home)
                    .child(Icon::new(Icons::Music).size_5().text_color(theme.text)),
            )
            .child(
                div()
                    .id("playlist")
                    .size_12()
                    .rounded_md()
                    .flex()
                    .flex_shrink_0()
                    .items_center()
                    .justify_center()
                    .bg(if page == &Page::Playlists {
                        theme.accent
                    } else {
                        theme.bg
                    })
                    .hover(|this| {
                        if page != &Page::Playlists {
                            this.bg(theme.highlighted)
                        } else {
                            this.bg(theme.accent)
                        }
                    })
                    .on_click(|_, _, cx| *cx.global_mut::<Page>() = Page::Playlists)
                    .child(Icon::new(Icons::MusicList).size_5().text_color(theme.text)),
            )
            .child(
                div()
                    .w_full()
                    .h_full()
                    .border_b_1()
                    .border_color(theme.border),
            )
            .child(
                div()
                    .id("settings")
                    .size_12()
                    .rounded_md()
                    .flex()
                    .flex_shrink_0()
                    .items_center()
                    .justify_center()
                    .bg(if page == &Page::Settings {
                        theme.accent
                    } else {
                        theme.bg
                    })
                    .hover(|this| {
                        if page != &Page::Settings {
                            this.bg(theme.highlighted)
                        } else {
                            this.bg(theme.accent)
                        }
                    })
                    .on_click(|_, _, cx| *cx.global_mut::<Page>() = Page::Settings)
                    .child(Icon::new(Icons::Settings).size_5().text_color(theme.text)),
            )
    }
}
