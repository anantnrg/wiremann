use crate::controller::Controller;
use crate::controller::state::PlaybackStatus;
use crate::library::TrackId;
use crate::lyrics_manager::{LyricLine, Lyrics, SyncType};
use crate::ui::components::bounds_observer::observe_bounds;
use crate::ui::components::virtual_list::{VirtualListScrollController, vlist};
use ahash::{AHashMap, AHashSet};
use gpui::prelude::FluentBuilder;
use gpui::{
    App, AppContext, Bounds, Context, Entity, FontWeight, Global, InteractiveElement, IntoElement,
    ParentElement, Pixels, Render, ScrollHandle, Styled, Window, div, px, rgb,
};

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

const LYRICS_TEXT_SIZE: Pixels = px(38.0);

#[derive(Debug, PartialEq)]
pub struct LyricsStateInner {
    pub status: LyricsStatus,
    pub lyrics: Option<Lyrics>,
    pub track_id: Option<TrackId>,
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
            track_id: None,
        }
    }
}

pub struct LyricLineView {
    pub line: LyricLine,
    pub idx: usize,
    pub sync_type: SyncType,
    pub primary_line: usize,
    pub word_bounds: Rc<RefCell<AHashMap<(usize, usize), Bounds<Pixels>>>>,
    pub playback: Duration,
}

impl LyricLineView {
    pub fn new(
        cx: &mut App,
        line: LyricLine,
        idx: usize,
        sync_type: SyncType,
        primary_line: usize,
        word_bounds: Rc<RefCell<AHashMap<(usize, usize), Bounds<Pixels>>>>,
        playback: Duration,
    ) -> Entity<Self> {
        cx.new(|_| Self {
            line,
            idx,
            sync_type,
            primary_line,
            word_bounds,
            playback,
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
                        .text_color(if self.idx == self.primary_line {
                            rgb(0xffffff)
                        } else {
                            rgb(0x666666)
                        })
                        .child(self.line.text.to_string()),
                ),

            SyncType::Word => div()
                .id(("line", self.idx))
                .w_full()
                .py_2()
                .px_4()
                .flex()
                .justify_center()
                .child(
                    div()
                        .w_full()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .justify_center()
                        .children(
                            self.line
                                .words
                                .as_ref()
                                .map(|words| words.iter())
                                .into_iter()
                                .flatten()
                                .enumerate()
                                .map(|(word_idx, word)| {
                                    let next_start = word.end;

                                    let progress = if self.playback < word.start {
                                        0.0
                                    } else if self.playback >= next_start {
                                        1.0
                                    } else {
                                        let elapsed = self.playback - word.start;
                                        let total = next_start - word.start;

                                        (elapsed.as_secs_f64() / total.as_secs_f64()) as f32
                                    }
                                    .clamp(0.0, 1.0);

                                    observe_bounds(
                                        format!("word_measure_{}_{}", self.idx, word_idx),
                                        div()
                                            .relative()
                                            .flex_none()
                                            .child(
                                                div()
                                                    .text_size(LYRICS_TEXT_SIZE)
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(0x666666))
                                                    .child(word.text.to_string()),
                                            )
                                            .child(
                                                div()
                                                    .absolute()
                                                    .top_0()
                                                    .left_0()
                                                    .h_full()
                                                    .overflow_hidden()
                                                    .when_some(
                                                        self.word_bounds
                                                            .borrow()
                                                            .get(&(self.idx, word_idx))
                                                            .map(|b| b.size.width * progress),
                                                        |this, width| this.w(width),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(LYRICS_TEXT_SIZE)
                                                            .font_weight(FontWeight::BOLD)
                                                            .text_color(rgb(0xffffff))
                                                            .child(word.text.to_string()),
                                                    ),
                                            ),
                                        {
                                            let bounds_cache = self.word_bounds.clone();

                                            let line_idx = self.idx;

                                            move |bounds, _, _cx| {
                                                bounds_cache
                                                    .borrow_mut()
                                                    .entry((line_idx, word_idx))
                                                    .or_insert(bounds);
                                            }
                                        },
                                    )
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
    pub word_bounds: Rc<RefCell<AHashMap<(usize, usize), Bounds<Pixels>>>>,

    pub primary_line: usize,
    pub last_scrolled_line: usize,
    pub last_track_id: Option<TrackId>,
}

impl LyricsView {
    pub fn new(cx: &mut App, scroll_handle: ScrollHandle) -> Entity<Self> {
        cx.new(|cx| Self {
            views: cx.new(|_| AHashMap::new()),

            scroll_handle,
            list_controller: VirtualListScrollController::new(),

            measured_heights: Vec::new(),
            measured_lines: Rc::new(RefCell::new(AHashSet::new())),
            word_bounds: Rc::new(RefCell::new(AHashMap::new())),

            primary_line: 0,
            last_scrolled_line: 0,
            last_track_id: None,
        })
    }

    fn update_primary_line(&mut self, lyrics: &Lyrics, playback: Duration) {
        if lyrics.lines.is_empty() {
            self.primary_line = 0;
            return;
        }

        self.primary_line = lyrics
            .lines
            .partition_point(|line| line.start.unwrap_or(Duration::ZERO) <= playback)
            .saturating_sub(1);
    }
}

impl Render for LyricsView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity();

        let state = cx.global::<Controller>().state.read(cx);

        let playback = state.playback.position;

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

        if state.playback.status == PlaybackStatus::Playing && lyrics.sync_type == SyncType::Word {
            window.request_animation_frame();
        }

        if self.last_track_id != lyrics_state.track_id {
            self.last_track_id = lyrics_state.track_id;
            self.primary_line = 0;
            self.last_scrolled_line = 0;
            self.measured_heights.clear();
            self.measured_lines.borrow_mut().clear();
            self.word_bounds.borrow_mut().clear();
        }

        if self.measured_heights.len() != lyrics.lines.len() {
            self.measured_heights = vec![px(110.0); lyrics.lines.len()];
        }

        self.update_primary_line(lyrics, playback);

        let primary_line = self.primary_line;
        let word_bounds = self.word_bounds.clone();

        if primary_line != self.last_scrolled_line {
            self.last_scrolled_line = primary_line;

            self.list_controller.scroll_to_item(primary_line);
        }

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

                        let content = div().w_full().child(LyricLineView::new(
                            cx,
                            line,
                            idx,
                            sync_type.clone(),
                            primary_line,
                            word_bounds.clone(),
                            playback,
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
