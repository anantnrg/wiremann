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
        let controller = cx.global::<Controller>();
        let state = controller.state.read(cx);

        let title = state
            .playback
            .current
            .and_then(|id| state.library.tracks.get(&id))
            .map(|t| t.title.clone())
            .unwrap_or("Not loaded.".to_string());

        let position = state.playback.position.clone();

        div().flex().flex_col().size_full().child(title).child(position.as_secs().to_string())
    }
}
