use super::Page;
use crate::ui::theme::Theme;

use crate::ui::icons::Icons;
use gpui::prelude::FluentBuilder;
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
            .w_auto()
            .flex()
            .border_b_1()
            .border_color(theme.white_05)
            .gap_12()
            .px_8()
            .child(
                div()
                    .w_auto()
                    .h_full()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .child("Library")
                    .text_color(if page == &Page::Library {
                        theme.text_primary
                    } else {
                        theme.text_muted
                    })
                    .when(page == &Page::Library, |this| {
                        this.child(
                            div()
                                .w_full()
                                .h_1()
                                .absolute()
                                .bottom_neg_4()
                                .bg(theme.accent),
                        )
                    }),
            )
    }
}
