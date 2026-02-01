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
            .w_full()
            .h_full()
            .flex()
            .flex_1()
            .items_center()
            .bg(theme.panel)
            .py_2()
            .gap_3()
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
                    .text_color(if page == &Page::Home {
                        theme.accent
                    } else {
                        theme.text
                    })
                    .hover(|this| this.text_color(theme.accent))
                    .on_click(|_, _, cx| *cx.global_mut::<Page>() = Page::Home)
                    .child(Icon::new(Icons::Music).size_6())
                    .child(if page == &Page::Home {
                        div()
                            .h_full()
                            .w_2()
                            .bg(theme.accent)
                            .absolute()
                            .rounded_l_md()
                            .right_neg_4()
                    } else {
                        div()
                    }),
            )
            .child(
                div()
                    .id("playlists")
                    .size_12()
                    .rounded_md()
                    .flex()
                    .flex_shrink_0()
                    .items_center()
                    .justify_center()
                    .text_color(if page == &Page::Playlists {
                        theme.accent
                    } else {
                        theme.text
                    })
                    .hover(|this| this.text_color(theme.accent))
                    .on_click(|_, _, cx| *cx.global_mut::<Page>() = Page::Playlists)
                    .child(Icon::new(Icons::MusicList).size_6())
                    .child(if page == &Page::Playlists {
                        div()
                            .h_full()
                            .w_2()
                            .bg(theme.accent)
                            .absolute()
                            .rounded_l_md()
                            .right_neg_4()
                    } else {
                        div()
                    }),
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
                    .text_color(if page == &Page::Settings {
                        theme.accent
                    } else {
                        theme.text
                    })
                    .hover(|this| this.text_color(theme.accent))
                    .on_click(|_, _, cx| *cx.global_mut::<Page>() = Page::Settings)
                    .child(Icon::new(Icons::Settings).size_6())
                    .child(if page == &Page::Settings {
                        div()
                            .h_full()
                            .w_2()
                            .bg(theme.accent)
                            .absolute()
                            .rounded_l_md()
                            .right_neg_4()
                    } else {
                        div()
                    }),
            )
    }
}
