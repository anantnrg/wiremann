use gpui::*;

use crate::controller::Controller;

pub struct Wiremann;

impl Wiremann {
    pub fn new(cx: &mut App) -> Self {
        cx.global::<Controller>()
            .load_audio("E:\\music\\violence ft. doomguy\\468 - GIVE ME A REASON.mp3".into());
        Wiremann {}
    }
}

impl Render for Wiremann {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full()
    }
}
