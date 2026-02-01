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
            .w_auto()
            .flex()
            .border_b_1()
            .border_color(theme.white_05)
            .gap_12()
    }
}
