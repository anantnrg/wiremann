use crate::{
    controller::{Controller, state::PlaybackStatus},
    ui::{
        animations::ease_in_out_expo,
        components::{
            controlbar::ControlBar,
            icons::{Icon, Icons},
            image_cache::ImageCache,
            lyrics::LyricsView,
            queue::Queue,
            scrollbar::{RightPad, floating_scrollbar},
        },
        theme::{DominantColors, Theme},
    },
};
use gpui::{
    Animation, AnimationExt, App, AppContext, Context, ElementId, Entity, FontWeight,
    InteractiveElement, IntoElement, ObjectFit, ParentElement, Render, ScrollHandle,
    StatefulInteractiveElement, Styled, StyledImage, UniformListScrollHandle, Window, div, img, px,
    rgb,
};
use gpui::{prelude::FluentBuilder, relative};

#[derive(Clone)]
pub struct PlayerPage {
    pub queue: Entity<Queue>,
    queue_scroll_handle: UniformListScrollHandle,
    pub lyrics: Entity<LyricsView>,
    lyrics_scroll_handle: UniformListScrollHandle,
    pub controlbar: Entity<ControlBar>,
    show_panel: Entity<bool>,
    current_panel: Entity<Panel>,
}

#[derive(PartialEq)]
enum Panel {
    Lyrics,
    Queue,
}

impl PlayerPage {
    pub fn new(cx: &mut App, controlbar: Entity<ControlBar>) -> Self {
        let queue_scroll_handle = UniformListScrollHandle::new();
        let show_panel = cx.new(|_| true);
        let current_panel = cx.new(|_| Panel::Queue);

        PlayerPage {
            queue: Queue::new(cx, queue_scroll_handle.clone()),
            queue_scroll_handle,
            lyrics: LyricsView::new(cx, ScrollHandle::new()),
            lyrics_scroll_handle: UniformListScrollHandle::new(),
            controlbar,
            show_panel,
            current_panel,
        }
    }
}

impl Render for PlayerPage {
    #[allow(clippy::too_many_lines)]
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = *cx.global::<Theme>();
        let dominant_colors = *cx.global::<DominantColors>();

        let controller = cx.global::<Controller>().clone();
        let state = controller.state.read(cx);
        let thumbnail = cx.global::<ImageCache>().current.clone();
        let scroll_handle = self.queue_scroll_handle.clone();
        let show_panel = self.show_panel.clone();

        let current = if let Some(id) = state.playback.current {
            state.library.tracks.get(&id)
        } else {
            None
        };

        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .child(div().h_full().w_full().absolute().bg(gpui::radial_gradient(
                0.4,
                0.4,
                1.0,
                1.0,
                gpui::gradient_color_stop(dominant_colors.color1, 0.0),
                gpui::gradient_color_stop(rgb(0x000000), 1.0),
            )))
            .child(
                div()
                    .h_full()
                    .w_full()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .px_16()
                    .pt_8()
                    .pb_2()
                    .bg(theme.player_bg)
                    .child(if let Some(track) = current {
                        div()
                            .w_auto()
                            .h_auto()
                            .flex()
                            .flex_col()
                            .items_center()
                            .justify_center()
                            .gap_y_6()
                            .flex_shrink_0()
                            .flex_1()
                            .child(if let Some(thumbnail) = thumbnail {
                                div().flex().flex_1().child(
                                    img(thumbnail)
                                        .object_fit(ObjectFit::Cover)
                                        .size_full()
                                        .rounded_xl()
                                        .border_2()
                                        .border_color(theme.border),
                                )
                            } else {
                                div().flex().flex_1().child(
                                    img("icons/placeholder.svg")
                                        .object_fit(ObjectFit::Contain)
                                        .size_full()
                                        .rounded_xl()
                                        .border_2()
                                        .border_color(theme.border),
                                )
                            })
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_y_neg_1()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        div()
                                            .text_2xl()
                                            .text_color(theme.player_title_text)
                                            .font_weight(FontWeight(500.0))
                                            .max_w_96()
                                            .truncate()
                                            .child(track.title.clone()),
                                    )
                                    .child(
                                        div()
                                            .text_base()
                                            .text_color(theme.player_artist_text)
                                            .font_weight(FontWeight(400.0))
                                            .max_w_96()
                                            .truncate()
                                            .child(track.artist.clone()),
                                    ),
                            )
                    } else {
                        div()
                    })
                    .child(
                        div()
                            .w_full()
                            .h_auto()
                            .flex()
                            .flex_shrink_0()
                            .gap_x_6()
                            .items_center()
                            .justify_center()
                            .mt_6()
                            .child(
                                div()
                                    .id("shuffle")
                                    .p_4()
                                    .rounded_md()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_color(theme.player_icons_text)
                                    .when(
                                        cx.global::<Controller>().state.read(cx).playback.shuffling,
                                        |this| {
                                            this.text_color(theme.player_icons_text_active)
                                                .bg(theme.player_icons_bg_active)
                                        },
                                    )
                                    .hover(|this| {
                                        this.bg(theme.player_icons_bg_hover)
                                            .text_color(theme.player_icons_text_hover)
                                    })
                                    .on_click({
                                        let controller = controller.clone();
                                        move |_, _, cx| controller.set_shuffle(cx)
                                    })
                                    .cursor_pointer()
                                    .child(Icon::new(Icons::Shuffle).size_4()),
                            )
                            .child(
                                div()
                                    .id("previous")
                                    .p_4()
                                    .rounded_md()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .hover(|this| {
                                        this.bg(theme.player_icons_bg_hover)
                                            .text_color(theme.player_icons_text_hover)
                                    })
                                    .on_click(|_, _, cx| cx.global::<Controller>().clone().prev(cx))
                                    .text_color(theme.player_icons_text)
                                    .cursor_pointer()
                                    .child(Icon::new(Icons::Prev).size_4()),
                            )
                            .child(
                                div()
                                    .id("play_pause")
                                    .p_5()
                                    .rounded_full()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .bg(theme.player_play_pause_bg)
                                    .hover(|this| this.bg(theme.player_play_pause_hover))
                                    .on_click(|_, _, cx| {
                                        match cx
                                            .global::<Controller>()
                                            .state
                                            .read(cx)
                                            .playback
                                            .status
                                        {
                                            PlaybackStatus::Paused | PlaybackStatus::Stopped => {
                                                cx.global::<Controller>().play();
                                            }
                                            PlaybackStatus::Playing => {
                                                cx.global::<Controller>().pause();
                                            }
                                        }
                                    })
                                    .text_color(theme.player_play_pause_text)
                                    .cursor_pointer()
                                    .child(
                                        if cx.global::<Controller>().state.read(cx).playback.status
                                            == PlaybackStatus::Playing
                                        {
                                            Icon::new(Icons::Pause).size_5()
                                        } else {
                                            Icon::new(Icons::Play).size_5()
                                        },
                                    ),
                            )
                            .child(
                                div()
                                    .id("next")
                                    .p_4()
                                    .rounded_md()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .hover(|this| {
                                        this.bg(theme.player_icons_bg_hover)
                                            .text_color(theme.player_icons_text_hover)
                                    })
                                    .on_click(|_, _, cx| cx.global::<Controller>().clone().next(cx))
                                    .cursor_pointer()
                                    .text_color(theme.player_icons_text)
                                    .child(Icon::new(Icons::Next).size_4()),
                            )
                            .child(
                                div()
                                    .id("repeat")
                                    .p_4()
                                    .rounded_md()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .cursor_pointer()
                                    .hover(|this| {
                                        this.bg(theme.player_icons_bg_hover)
                                            .text_color(theme.player_icons_text_hover)
                                    })
                                    .text_color(theme.player_icons_text)
                                    .when(
                                        cx.global::<Controller>().state.read(cx).playback.repeat,
                                        |this| {
                                            this.text_color(theme.player_icons_text_active)
                                                .bg(theme.player_icons_bg_active)
                                        },
                                    )
                                    .on_click({
                                        let controller = controller.clone();
                                        move |_, _, cx| controller.set_repeat(cx)
                                    })
                                    .child(Icon::new(Icons::Repeat).size_4()),
                            ),
                    )
                    .child(self.controlbar.clone()),
            )
            .child(if *show_panel.read(cx) {
                div()
                    .h_full()
                    .w(relative(0.46))
                    .when(*self.current_panel.read(cx) == Panel::Queue, |this| {
                        this.max_w_128()
                    })
                    .flex_shrink_0()
                    .flex()
                    .flex_col()
                    .bg(theme.player_panel_bg)
                    .child({
                        let current_panel = self.current_panel.clone();

                        let active_left = if *current_panel.read(cx) == Panel::Queue {
                            px(0.0)
                        } else {
                            px(72.0)
                        };

                        div()
                            .w_full()
                            .h_10()
                            .relative()
                            .flex()
                            .items_center()
                            .justify_start()
                            .px_4()
                            .child(
                                div()
                                    .relative()
                                    .h_full()
                                    .flex()
                                    .items_center()
                                    .gap_x_6()
                                    .child(
                                        div()
                                            .absolute()
                                            .bottom_1()
                                            .left(active_left)
                                            .w(px(48.0))
                                            .h(px(2.0))
                                            .rounded_full()
                                            .bg(theme.switcher_active),
                                    )
                                    .child({
                                        let current_panel = current_panel.clone();

                                        div()
                                            .id("panel_switcher_queue")
                                            .w(px(48.0))
                                            .flex()
                                            .justify_center()
                                            .cursor_pointer()
                                            .on_click({
                                                let current_panel = current_panel.clone();
                                                move |_, _, cx| {
                                                    current_panel.update(cx, |p, _| {
                                                        *p = Panel::Queue;
                                                    });
                                                }
                                            })
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_weight(FontWeight(500.0))
                                                    .text_color(
                                                        if *current_panel.read(cx) == Panel::Queue {
                                                            theme.player_panel_tab_text_active
                                                        } else {
                                                            theme.player_panel_tab_text
                                                        },
                                                    )
                                                    .child("Queue"),
                                            )
                                    })
                                    .child({
                                        let current_panel = current_panel.clone();

                                        div()
                                            .id("panel_switcher_lyrics")
                                            .w(px(48.0))
                                            .flex()
                                            .justify_center()
                                            .cursor_pointer()
                                            .on_click({
                                                let current_panel = current_panel.clone();
                                                move |_, _, cx| {
                                                    current_panel.update(cx, |p, _| {
                                                        *p = Panel::Lyrics;
                                                    });
                                                }
                                            })
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_weight(FontWeight(500.0))
                                                    .text_color(
                                                        if *current_panel.read(cx) == Panel::Lyrics
                                                        {
                                                            theme.player_panel_tab_text_active
                                                        } else {
                                                            theme.player_panel_tab_text
                                                        },
                                                    )
                                                    .child("Lyrics"),
                                            )
                                    }),
                            )
                    })
                    .child({
                        let current_panel = self.current_panel.clone();

                        div().w_full().h_full().px_4().flex().relative().child({
                            if *current_panel.read(cx) == Panel::Queue {
                                div()
                                    .id("queue_container")
                                    .w_full()
                                    .h_full()
                                    .child(self.queue.clone())
                                    .child(floating_scrollbar(
                                        "queue_scrollbar",
                                        scroll_handle,
                                        RightPad::Pad,
                                    ))
                            } else {
                                div()
                                    .id("lyrics_container")
                                    .w_full()
                                    .max_h_full()
                                    .min_h_0()
                                    .flex()
                                    .flex_col()
                                    .overflow_hidden()
                                    .child(self.lyrics.clone())
                            }
                        })
                    })
            } else {
                div()
            })
            .child(
                div()
                    .id("show_hide_queue")
                    .px_3()
                    .py_1()
                    .absolute()
                    .top_1()
                    .right_1()
                    .text_center()
                    .rounded_md()
                    .text_sm()
                    .font_weight(FontWeight(400.0))
                    .text_color(theme.player_panel_show_hide_text)
                    .cursor_pointer()
                    .hover(|this| {
                        this.bg(theme.player_panel_show_hide_bg_hover)
                            .text_color(theme.player_panel_show_hide_text_hover)
                    })
                    .on_click(move |_, _, cx| show_panel.update(cx, |this, _| *this = !*this))
                    .child(Icon::new(Icons::PanelRight).size_5()),
            )
    }
}
