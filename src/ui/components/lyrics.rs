use crate::controller::Controller;
use crate::controller::state::PlaybackStatus;
use crate::lyrics_manager::{LyricLine, Lyrics, SyncType};
use crate::ui::components::bounds_observer::observe_bounds;
use crate::ui::components::virtual_list::{VirtualListScrollController, vlist};

use ahash::{AHashMap, AHashSet};

use gpui::{
    App, AppContext, Bounds, Context, Entity, FontWeight, Global, InteractiveElement, IntoElement,
    ParentElement, Pixels, Render, ScrollHandle, Styled, Window, div, px, rgb,
};

use std::cell::RefCell;
use std::rc::Rc;
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
    pub sync_type: SyncType,
}

impl LyricLineView {
    pub fn new(cx: &mut App, line: LyricLine, idx: usize, sync_type: SyncType) -> Entity<Self> {
        cx.new(|_| Self {
            line,
            idx,
            sync_type,
        })
    }
}

impl Render for LyricLineView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        match self.sync_type {
            SyncType::Line | SyncType::Unsynced => div()
                .id(("line", self.idx))
                .w_full()
                .py_2()
                .flex()
                .justify_center()
                .child(
                    div()
                        .max_w_full()
                        .text_center()
                        .text_size(LYRICS_TEXT_SIZE)
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xffffff))
                        .child(self.line.text.to_string()),
                ),

            SyncType::Word => div()
                .id(("line", self.idx))
                .w_full()
                .py_2()
                .flex()
                .justify_center()
                .child(
                    div()
                        .max_w(px(900.0))
                        .w_full()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .justify_center()
                        .gap_x_1()
                        .text_size(LYRICS_TEXT_SIZE)
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xffffff))
                        .children(
                            self.line
                                .words
                                .as_ref()
                                .map(|words| words.iter())
                                .into_iter()
                                .flatten()
                                .enumerate()
                                .map(|(word_idx, word)| {
                                    div()
                                        .id(format!("word_{}_{}", self.idx, word_idx))
                                        .child(word.text.to_string())
                                }),
                        ),
                ),
        }
    }
}

#[derive(Clone)]
pub struct LyricsView {
    pub views: Entity<AHashMap<usize, Entity<LyricLineView>>>,

    pub scroll_handle: ScrollHandle,
    pub list_controller: VirtualListScrollController,

    pub measured_heights: Vec<Pixels>,
    pub measured_lines: Rc<RefCell<AHashSet<usize>>>,

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

            measured_heights: Vec::new(),
            measured_lines: Rc::new(RefCell::new(AHashSet::new())),

            last_active_line: 0,

            last_playback: Duration::ZERO,
            elapsed_since_last_update: Instant::now(),
        })
    }

    fn get_or_create_line(
        views: &Entity<AHashMap<usize, Entity<LyricLineView>>>,
        line: LyricLine,
        idx: usize,
        sync_type: SyncType,
        cx: &mut App,
    ) -> Entity<LyricLineView> {
        views.update(cx, |this, cx| {
            this.entry(idx)
                .or_insert_with(|| LyricLineView::new(cx, line, idx, sync_type))
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
        let entity = cx.entity();

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

        if self.measured_heights.len() != lyrics.lines.len() {
            self.measured_heights = vec![px(110.0); lyrics.lines.len()];
        }

        let active_line = Self::active_line(&lyrics.lines, playback);

        if active_line != self.last_active_line {
            self.last_active_line = active_line;

            self.list_controller.scroll_to_item(active_line);
        }

        let views = self.views.clone();
        let lines = lyrics.lines.clone();
        let sync_type = lyrics.sync_type.clone();

        let measured_heights = Rc::new(self.measured_heights.clone());

        let measured_lines = self.measured_lines.clone();

        let list_entity = entity.clone();

        vlist(
            cx.entity(),
            "lyrics",
            measured_heights,
            self.scroll_handle.clone(),
            self.list_controller.clone(),
            move |_this, range, _, cx| {
                range
                    .map(|idx| {
                        let line = lines[idx].clone();

                        let content = div().w_full().child(LyricsView::get_or_create_line(
                            &views,
                            line,
                            idx,
                            sync_type.clone(),
                            cx,
                        ));

                        observe_bounds(("lyrics_line_measure", idx), content, {
                            let entity = list_entity.clone();

                            let measured_lines = measured_lines.clone();

                            move |bounds, _, cx| {
                                entity.update(cx, |this, _| {
                                    let height = bounds.size.height;

                                    if let Some(existing) = this.measured_heights.get_mut(idx) {
                                        *existing = height;
                                    }

                                    measured_lines.borrow_mut().insert(idx);
                                });
                            }
                        })
                        .into_any_element()
                    })
                    .collect::<Vec<_>>()
            },
        )
        .into_any_element()
    }
}
