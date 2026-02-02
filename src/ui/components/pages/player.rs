use crate::ui::theme::Theme;

use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::StyledExt;

#[derive(Clone)]
pub struct PlayerPage;

impl PlayerPage {
    pub fn new() -> Self {
        Player {}
    }
}

impl Render for PlayerPage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
    }
}
