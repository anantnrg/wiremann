use crate::controller::Controller;
use crate::lyrics_manager::{LyricLine, LyricWord, Lyrics, SyncType};
use crate::ui::components::bounds_observer::observe_bounds;
use crate::ui::components::lyrics_metrics::LyricsMetrics;
use crate::ui::components::virtual_list::vlist;
use ahash::AHashMap;
use gpui::{
    App, AppContext, Bounds, Context, Entity, FontWeight, Global, InteractiveElement, IntoElement,
    ParentElement, Pixels, Point, Render, ScrollHandle, StatefulInteractiveElement, Styled, Window,
    div, px, rgb,
};

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

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

    fn active_word(words: &[LyricWord], playback: Duration) -> Option<usize> {
        words
            .iter()
            .enumerate()
            .rfind(|(_, word)| playback >= word.start)
            .map(|(idx, _)| idx)
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
                    .px_6()
                    .py_2()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_3xl()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0xffffff))
                            .opacity(opacity)
                            .child(self.line.text.clone()),
                    )
                    .into_any_element()
            }

            SyncType::Word => {
                let active_word = self
                    .line
                    .words
                    .as_ref()
                    .and_then(|words| Self::active_word(words, playback));

                div()
                    .id(("line", self.idx))
                    .w_full()
                    .min_w_0()
                    .px_6()
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
                            .children(
                                self.line
                                    .words
                                    .clone()
                                    .unwrap_or_default()
                                    .into_iter()
                                    .enumerate()
                                    .map(|(word_idx, word)| {
                                        let opacity = if is_active_line {
                                            match active_word {
                                                Some(active) if word_idx < active => 0.85,

                                                Some(active) if word_idx == active => 1.0,

                                                _ => 0.55,
                                            }
                                        } else {
                                            0.4
                                        };

                                        div()
                                            .id(format!("word_{}_{}", self.idx, word_idx))
                                            .text_3xl()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0xffffff))
                                            .opacity(opacity)
                                            .child(word.text)
                                    }),
                            ),
                    )
                    .into_any_element()
            }

            SyncType::Unsynced => div()
                .id(("unsynced", self.idx))
                .w_full()
                .min_w_0()
                .px_6()
                .py_1()
                .child(
                    div()
                        .text_3xl()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0xffffff))
                        .opacity(0.4)
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
    pub cumulative_offsets: Vec<Pixels>,
    pub metrics: Rc<RefCell<LyricsMetrics>>,
}

impl LyricsView {
    pub fn new(cx: &mut App, scroll_handle: ScrollHandle) -> Entity<Self> {
        cx.new(|cx| Self {
            views: cx.new(|_| AHashMap::new()),

            scroll_handle,

            last_active_line: 0,

            panel_bounds: None,

            measured_heights: Vec::new(),

            cumulative_offsets: Vec::new(),

            metrics: Rc::new(RefCell::new(LyricsMetrics::new())),
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

    fn recompute_heights(&mut self, lyrics: &Lyrics) {
        let Some(bounds) = self.panel_bounds else {
            return;
        };

        let width = bounds.size.width.to_f64() as f32;

        self.measured_heights = lyrics
            .lines
            .iter()
            .map(|line| {
                let text_height =
                    self.metrics
                        .borrow_mut()
                        .measure_height(&line.text, width - 48.0, 30.0);

                text_height + px(32.0)
            })
            .collect();

        self.cumulative_offsets.clear();

        let mut y = px(0.0);

        for height in &self.measured_heights {
            self.cumulative_offsets.push(y);

            y += *height;
        }
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

        let active_line = Self::active_line(&lyrics.lines, playback);

        if active_line != self.last_active_line {
            self.last_active_line = active_line;

            if let Some(offset) = self.cumulative_offsets.get(active_line) {
                let handle = self.scroll_handle.clone();

                let y = *offset;

                cx.defer(move |_| {
                    handle.set_offset(Point { x: px(0.0), y });
                });
            }
        }

        let views = self.views.clone();

        let lines = lyrics.lines.clone();

        let sync_type = lyrics.sync_type.clone();

        let measured_heights = Rc::new(self.measured_heights.clone());

        let root =
            div()
                .w_full()
                .min_w_0()
                .h_full()
                .min_h_0()
                .flex()
                .flex_col()
                .child(vlist(
                    cx.entity(),
                    "lyrics",
                    measured_heights.clone(),
                    self.scroll_handle.clone(),
                    move |_this, range, _, cx| {
                        range
                            .map(|idx| {
                                let line = lines[idx].clone();

                                let measured_height =
                                    measured_heights.get(idx).copied().unwrap_or(px(40.0));

                                div()
                                    .id(("lyrics_line", idx))
                                    .w_full()
                                    .min_w_0()
                                    .h(measured_height)
                                    .child(div().text_xs().text_color(rgb(0x888888)).px_6().child(
                                        format!("measured: {:.1}", measured_height.to_f64()),
                                    ))
                                    .child(LyricsView::get_or_create_line(
                                        &views,
                                        line,
                                        idx,
                                        sync_type.clone(),
                                        cx,
                                    ))
                            })
                            .collect::<Vec<_>>()
                    },
                ));

        observe_bounds("lyrics_panel_bounds", root, move |bounds, _, cx| {
            entity.update(cx, |this, cx| {
                let changed = this
                    .panel_bounds
                    .map(|b| b.size.width != bounds.size.width)
                    .unwrap_or(true);

                this.panel_bounds = Some(bounds);

                if changed {
                    let lyrics = cx.global::<LyricsState>().0.read(cx).lyrics.clone();

                    if let Some(lyrics) = lyrics {
                        this.recompute_heights(&lyrics);
                    }
                }

                cx.notify();
            });
        })
        .into_any_element()
    }
}
