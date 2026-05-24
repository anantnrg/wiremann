use crate::controller::Controller;
use crate::lyrics_manager::{LyricLine, LyricWord, Lyrics, SyncType};
use crate::ui::components::bounds_observer::observe_bounds;
use ahash::AHashMap;
use gpui::prelude::FluentBuilder;
use std::cell::RefCell;

use gpui::{
    App, AppContext, Bounds, Context, Entity, FontWeight, Global, InteractiveElement, IntoElement,
    ParentElement, Pixels, Render, ScrollHandle, Styled, VirtualListScrollController, Window, div,
    px, rgb, vlist,
};

use std::rc::Rc;
use std::time::Duration;

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
    pub word_bounds: Rc<RefCell<AHashMap<(usize, usize), Bounds<Pixels>>>>,
}

impl LyricLineView {
    pub fn new(
        cx: &mut App,
        line: LyricLine,
        idx: usize,
        sync_type: SyncType,
        word_bounds: Rc<RefCell<AHashMap<(usize, usize), Bounds<Pixels>>>>,
    ) -> Entity<Self> {
        cx.new(|_| Self {
            line,
            idx,
            sync_type,
            word_bounds,
        })
    }

    fn active_word(words: &[LyricWord], playback: Duration) -> Option<usize> {
        words
            .iter()
            .enumerate()
            .rfind(|(_, word)| playback >= word.start)
            .map(|(idx, _)| idx)
    }
    fn reveal_width(&self, playback: Duration) -> Option<Pixels> {
        let words = self.line.words.as_ref()?;

        let active_idx = Self::active_word(words, playback)?;

        let active_word = &words[active_idx];

        let bounds_cache = self.word_bounds.borrow();

        let first_bounds = bounds_cache.get(&(self.idx, 0))?;

        let active_bounds = bounds_cache.get(&(self.idx, active_idx))?;

        let relative_x = active_bounds.origin.x - first_bounds.origin.x;

        let next_time = words
            .get(active_idx + 1)
            .map(|w| w.start)
            .unwrap_or(active_word.end);

        let duration = next_time.saturating_sub(active_word.start);

        let elapsed = playback.saturating_sub(active_word.start);

        let progress = if duration.is_zero() {
            1.0
        } else {
            elapsed.as_secs_f32() / duration.as_secs_f32()
        }
        .clamp(0.0, 1.0);

        Some(relative_x + active_bounds.size.width * progress)
    }
}

impl Render for LyricLineView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = cx.global::<Controller>().state.read(cx);

        let playback = state.playback.position;

        let lyrics = cx.global::<LyricsState>().0.read(cx).lyrics.clone();

        let Some(lyrics) = lyrics else {
            return div().into_any_element();
        };

        let active_line = lyrics
            .lines
            .iter()
            .enumerate()
            .rfind(|(_, line)| line.start.map(|s| playback >= s).unwrap_or(false))
            .map(|(idx, _)| idx)
            .unwrap_or(0);

        let is_active_line = self.idx == active_line;

        match self.sync_type {
            SyncType::Line => {
                let opacity = if is_active_line { 1.0 } else { 0.4 };

                div()
                    .id(("line", self.idx))
                    .w_full()
                    .min_w_0()
                    .py_2()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .max_w_full()
                            .text_center()
                            .text_size(LYRICS_TEXT_SIZE)
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0xffffff))
                            .opacity(opacity)
                            .child(self.line.text.clone()),
                    )
                    .into_any_element()
            }

            SyncType::Word => {
                let words = self.line.words.clone().unwrap_or_default();

                div()
                    .id(("line", self.idx))
                    .w_full()
                    .min_w_0()
                    .py_2()
                    .flex()
                    .justify_center()
                    .child(
                        div()
                            .w_full()
                            .min_w_0()
                            .flex()
                            .flex_row()
                            .flex_wrap()
                            .justify_center()
                            .children(words.into_iter().enumerate().map(|(word_idx, word)| {
                                let progress = {
                                    let next_start = self
                                        .line
                                        .words
                                        .as_ref()
                                        .and_then(|w| w.get(word_idx + 1))
                                        .map(|w| w.start)
                                        .unwrap_or(word.end);

                                    if playback < word.start {
                                        0.0
                                    } else if playback >= next_start {
                                        1.0
                                    } else {
                                        let duration = next_start.saturating_sub(word.start);

                                        let elapsed = playback.saturating_sub(word.start);

                                        if duration.is_zero() {
                                            1.0
                                        } else {
                                            elapsed.as_secs_f32() / duration.as_secs_f32()
                                        }
                                    }
                                }
                                .clamp(0.0, 1.0);

                                observe_bounds(
                                    format!("word_measure_{}_{}", self.idx, word_idx),
                                    div()
                                        .relative()
                                        .flex_none()
                                        .child(
                                            div()
                                                .id(format!("base_word_{}_{}", self.idx, word_idx))
                                                .text_size(LYRICS_TEXT_SIZE)
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(rgb(0xffffff))
                                                .opacity(if is_active_line { 0.35 } else { 0.2 })
                                                .child(word.text.clone()),
                                        )
                                        .child(
                                            div()
                                                .absolute()
                                                .h_full()
                                                .top_0()
                                                .left_0()
                                                .overflow_hidden()
                                                .when_some(
                                                    {
                                                        let bounds_cache =
                                                            self.word_bounds.borrow();

                                                        bounds_cache
                                                            .get(&(self.idx, word_idx))
                                                            .map(|b| b.size.width * progress)
                                                    },
                                                    |this, width| this.w(width),
                                                )
                                                .child(
                                                    div()
                                                        .h_full()
                                                        .flex()
                                                        .items_center()
                                                        .text_size(LYRICS_TEXT_SIZE)
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .text_color(rgb(0xffffff))
                                                        .child(word.text),
                                                ),
                                        ),
                                    {
                                        let bounds_cache = self.word_bounds.clone();

                                        let line_idx = self.idx;

                                        move |bounds, _, _cx| {
                                            bounds_cache
                                                .borrow_mut()
                                                .insert((line_idx, word_idx), bounds);
                                        }
                                    },
                                )
                            })),
                    )
                    .into_any_element()
            }
            SyncType::Unsynced => div()
                .id(("unsynced", self.idx))
                .w_full()
                .min_w_0()
                .py_1()
                .child(
                    div()
                        .text_size(LYRICS_TEXT_SIZE)
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0xffffff))
                        .child(self.line.text.clone()),
                )
                .into_any_element(),
        }
    }
}

#[derive(Clone)]
pub struct LyricsView {
    pub views: Entity<AHashMap<usize, Entity<LyricLineView>>>,
    pub scroll_handle: ScrollHandle,
    pub last_active_line: usize,
    pub panel_bounds: Option<Bounds<Pixels>>,
    pub measured_heights: Vec<Pixels>,
    pub list_controller: VirtualListScrollController,
    pub word_bounds: Rc<RefCell<AHashMap<(usize, usize), Bounds<Pixels>>>>,
}

impl LyricsView {
    pub fn new(cx: &mut App, scroll_handle: ScrollHandle) -> Entity<Self> {
        cx.new(|cx| Self {
            views: cx.new(|_| AHashMap::new()),
            scroll_handle,
            last_active_line: 0,
            panel_bounds: None,
            measured_heights: Vec::new(),
            list_controller: VirtualListScrollController::new(),
            word_bounds: Rc::new(RefCell::new(AHashMap::new())),
        })
    }

    fn get_or_create_line(
        views: &Entity<AHashMap<usize, Entity<LyricLineView>>>,
        line: LyricLine,
        idx: usize,
        sync_type: SyncType,
        word_bounds: Rc<RefCell<AHashMap<(usize, usize), Bounds<Pixels>>>>,
        cx: &mut App,
    ) -> Entity<LyricLineView> {
        views.update(cx, |this, cx| {
            this.entry(idx)
                .or_insert_with(|| LyricLineView::new(cx, line, idx, sync_type, word_bounds))
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
}

impl Render for LyricsView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity();

        let state = cx.global::<Controller>().state.read(cx);

        let playback = state.playback.position;

        let lyrics = cx.global::<LyricsState>().0.read(cx).lyrics.clone();

        let Some(lyrics) = lyrics else {
            return div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .text_color(rgb(0xffffff))
                        .opacity(0.5)
                        .text_xl()
                        .child("No lyrics"),
                )
                .into_any_element();
        };

        if self.measured_heights.len() != lyrics.lines.len() {
            self.measured_heights =
                vec![px(LYRICS_TEXT_SIZE.to_f64() as f32 * 2.5); lyrics.lines.len()];
        }

        let active_line = Self::active_line(&lyrics.lines, playback);

        let display_line = {
            if let Some(current) = lyrics.lines.get(active_line) {
                if let Some(end) = current.end {
                    if playback >= end {
                        (active_line + 1).min(lyrics.lines.len().saturating_sub(1))
                    } else {
                        active_line
                    }
                } else {
                    active_line
                }
            } else {
                active_line
            }
        };

        if display_line != self.last_active_line {
            self.last_active_line = display_line;

            self.list_controller.scroll_to_item(display_line);
        }

        let views = self.views.clone();

        let lines = lyrics.lines.clone();

        let sync_type = lyrics.sync_type.clone();

        let measured_heights = Rc::new(self.measured_heights.clone());
        let word_bounds = self.word_bounds.clone();
        let list_entity = entity.clone();
        let list = vlist(
            cx.entity(),
            "lyrics",
            measured_heights.clone(),
            self.scroll_handle.clone(),
            self.list_controller.clone(),
            move |_this, range, _, cx| {
                range
                    .map(|idx| {
                        let line = lines[idx].clone();

                        observe_bounds(
                            ("lyrics_line_measure", idx),
                            div().id(("lyrics_line", idx)).w_full().min_w_0().child(
                                LyricsView::get_or_create_line(
                                    &views,
                                    line,
                                    idx,
                                    sync_type.clone(),
                                    word_bounds.clone(),
                                    cx,
                                ),
                            ),
                            {
                                let entity = list_entity.clone();

                                move |bounds, _, cx| {
                                    entity.update(cx, |this, cx| {
                                        let height = bounds.size.height;

                                        if let Some(existing) = this.measured_heights.get_mut(idx) {
                                            if *existing != height {
                                                *existing = height;

                                                cx.notify();
                                            }
                                        }
                                    });
                                }
                            },
                        )
                    })
                    .collect::<Vec<_>>()
            },
        );

        let root = div()
            .w_full()
            .min_w_0()
            .h_full()
            .min_h_0()
            .flex()
            .flex_col()
            .child(list);

        observe_bounds("lyrics_panel_bounds", root, move |bounds, _, cx| {
            entity.update(cx, |this, cx| {
                this.panel_bounds = Some(bounds);

                cx.notify();
            });
        })
        .into_any_element()
    }
}
