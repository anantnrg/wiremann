use crate::controller::state::PlaybackStatus;
use crate::controller::Controller;
use crate::ui::components;
use crate::ui::components::controlbar::ControlBar;
use crate::ui::helpers::slider_to_secs;
use crate::ui::theme::Theme;
use components::{pages::player::PlayerPage, titlebar::Titlebar, Page};
use gpui::*;
use gpui_component::slider::{SliderEvent, SliderState};

pub struct Wiremann {
    pub titlebar: Entity<Titlebar>,
    pub player_page: Entity<PlayerPage>,
}

impl Wiremann {
    pub fn new(cx: &mut App) -> Self {
        let vol_slider_state = cx.new(|_| {
            SliderState::new()
                .min(0.0)
                .max(100.0)
                .default_value(100.0)
                .step(1.0)
        });

        let playback_slider_state = cx.new(|_| {
            SliderState::new()
                .min(0.0)
                .max(100.0)
                .default_value(0.0)
                .step(1.0)
        });

        cx.subscribe(
            &vol_slider_state,
            |_, _, event: &SliderEvent, cx| match event {
                SliderEvent::Change(value) => {
                    cx.global::<Controller>().volume(value.start());
                    cx.notify();
                }
            },
        )
            .detach();

        cx.subscribe(
            &playback_slider_state,
            |_, _, event: &SliderEvent, cx| match event {
                SliderEvent::Change(value) => {
                    let controller = cx.global::<Controller>();
                    if controller.player_state.state == PlaybackStatus::Playing {
                        if let Some(meta) = controller.player_state.clone().meta {
                            controller.seek(slider_to_secs(value.start(), meta.duration));
                        }
                    }

                    cx.notify();
                }
            },
        )
            .detach();

        cx.set_global(Theme::default());
        cx.set_global(Page::Player);

        let titlebar = cx.new(|cx| Titlebar::new(cx));
        let controlbar = cx.new(|_| ControlBar::new(playback_slider_state, vol_slider_state));
        let player_page = cx.new(|cx| PlayerPage::new(cx, controlbar));

        Self {
            titlebar,
            player_page,
        }
    }
}

impl Render for Wiremann {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div()
            .id("main_container")
            .size_full()
            .font_family("Space Grotesk")
            .flex()
            .flex_col()
            .justify_center()
            .items_center()
            .bg(theme.bg_main)
            .child(self.titlebar.clone())
            .child(match cx.global::<Page>() {
                Page::Player => div().w_full().h_full().child(self.player_page.clone()),
                _ => div(),
            })
    }
}
