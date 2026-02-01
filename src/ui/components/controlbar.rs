use crate::controller::player::Controller;
use crate::ui::theme::Theme;

use super::slider::{Slider, SliderState};
// use crate::ui::icons::Icons;
use gpui::*;
// use gpui_component::Icon;

#[derive(Clone)]
pub struct ControlBar {
    pub playback_slider_state: Entity<SliderState>,
    pub vol_slider_state: Entity<SliderState>,
}

impl ControlBar {
    pub fn new(
        playback_slider_state: Entity<SliderState>,
        vol_slider_state: Entity<SliderState>,
    ) -> Self {
        ControlBar {
            playback_slider_state,
            vol_slider_state,
        }
    }
}

impl Render for ControlBar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let state = cx.global::<Controller>().state.clone();

        div()
            .w_full()
            .h_32()
            .flex()
            .flex_col()
            // .bg(theme.panel)
            .border_t_1()
            // .border_color(theme.border)
            .child(
                div()
                    .w_full()
                    .h_1_2()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .px_16()
                    .items_center()
                    .justify_center()
                    .child(
                        div().flex().w_full().flex_1().child(
                            Slider::new(&self.vol_slider_state)
                                // .bg(theme.highlighted)
                                .text_color(theme.accent),
                        ),
                    )
                    .child(
                        div()
                            .flex()
                            .w_full()
                            .items_center()
                            .justify_between()
                            .child(div().flex().flex_shrink_0().child(format!(
                                "{:02}:{:02}",
                                state.position / 60,
                                state.position % 60
                            )))
                            .child(div().flex().flex_shrink_0().child(format!(
                                "{:02}:{:02}",
                                state.duration / 60,
                                state.duration % 60
                            ))),
                    ),
            )
    }
}
