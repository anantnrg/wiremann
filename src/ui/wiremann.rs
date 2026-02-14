use gpui::*;

pub struct Wiremann;

impl Wiremann {
    pub fn new(cx: &mut App) -> Self {
        Wiremann {}
    }
}

impl Render for Wiremann {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full()
    }
}
