use crate::controller::Controller;
use crate::controller::state::PlaybackStatus;
use crate::lyrics_manager::{LyricLine, Lyrics, SyncType};
use crate::ui::components::virtual_list::{VirtualListScrollController, vlist};

use ahash::AHashMap;

use gpui::{
    App, AppContext, Context, Entity, FontWeight, Global, InteractiveElement, IntoElement,
    ParentElement, Pixels, Render, ScrollHandle, Styled, Window, div, px, rgb,
};

use std::time::{Duration, Instant};

const LYRICS_TEXT_SIZE: Pixels = px(38.0);

#[derive(Debug, PartialEq)]
pub struct LyricsStateInner {
    pub status: LyricsStatus,
    pub lyrics: Option<Lyrics>,
}

#[derive(Debug, PartialEq)]
pub enum LyricsStatus {
    Fetching,
    Available,
    Unavailable,
}

pub struct LyricsState(pub Entity<LyricsStateInner>);

impl Global for LyricsState {}

impl LyricsStateInner {
    pub fn new() -> Self {
        Self {
            status: LyricsStatus::Unavailable,
            lyrics: None,
        }
    }
}
pub struct LyricLineView {
    pub line: LyricLine,
    pub idx: usize,
}

impl LyricLineView {
    pub fn new(cx: &mut App, line: LyricLine, idx: usize) -> Entity<Self> {
        cx.new(|_| Self { line, idx })
    }
}

impl Render for LyricLineView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .id(("line", self.idx))
            .w_full()
            .py_2()
            .flex()
            .justify_center()
            .child(
                div()
                    .text_center()
                    .text_size(LYRICS_TEXT_SIZE)
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xffffff))
                    .child(self.line.text.clone().to_string()),
            )
    }
}

#[derive(Clone)]
pub struct LyricsView {
    pub views: Entity<AHashMap<usize, Entity<LyricLineView>>>,
    pub scroll_handle: ScrollHandle,
    pub list_controller: VirtualListScrollController,

    pub last_active_line: usize,

    pub last_playback: Duration,
    pub elapsed_since_last_update: Instant,
}

impl LyricsView {
    pub fn new(cx: &mut App, scroll_handle: ScrollHandle) -> Entity<Self> {
        cx.new(|cx| Self {
            views: cx.new(|_| AHashMap::new()),
            scroll_handle,
            list_controller: VirtualListScrollController::new(),

            last_active_line: 0,

            last_playback: Duration::ZERO,
            elapsed_since_last_update: Instant::now(),
        })
    }

    fn get_or_create_line(
        views: &Entity<AHashMap<usize, Entity<LyricLineView>>>,
        line: LyricLine,
        idx: usize,
        cx: &mut App,
    ) -> Entity<LyricLineView> {
        views.update(cx, |this, cx| {
            this.entry(idx)
                .or_insert_with(|| LyricLineView::new(cx, line, idx))
                .clone()
        })
    }

    fn active_line(lines: &[LyricLine], playback: Duration) -> usize {
        lines
            .iter()
            .enumerate()
            .rfind(|(_, line)| line.start.map(|s| playback >= s).unwrap_or(false))
            .map(|(idx, _)| idx)
            .unwrap_or(0)
    }

    fn interpolated_playback(&self, playing: bool) -> Duration {
        if playing {
            self.last_playback + self.elapsed_since_last_update.elapsed()
        } else {
            self.last_playback
        }
    }
}

impl Render for LyricsView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = cx.global::<Controller>().state.read(cx);

        let playback = self.interpolated_playback(state.playback.status == PlaybackStatus::Playing);

        let lyrics_state = cx.global::<LyricsState>().0.read(cx);

        let Some(lyrics) = lyrics_state.lyrics.as_ref() else {
            return div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .child(div().text_color(rgb(0xffffff)).child("No lyrics"))
                .into_any_element();
        };

        let active_line = Self::active_line(&lyrics.lines, playback);

        if active_line != self.last_active_line {
            self.last_active_line = active_line;

            self.list_controller.scroll_to_item(active_line);
        }

        let views = self.views.clone();
        let lines = lyrics.lines.clone();

        let estimated_heights = vec![px(110.0); lines.len()];

        vlist(
            cx.entity(),
            "lyrics",
            estimated_heights.into(),
            self.scroll_handle.clone(),
            self.list_controller.clone(),
            move |_this, range, _, cx| {
                range
                    .map(|idx| {
                        let line = lines[idx].clone();

                        div()
                            .w_full()
                            .child(LyricsView::get_or_create_line(&views, line, idx, cx))
                    })
                    .collect::<Vec<_>>()
            },
        )
        .into_any_element()
    }
}
