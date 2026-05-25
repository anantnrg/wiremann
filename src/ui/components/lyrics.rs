use crate::controller::Controller;
use crate::controller::state::PlaybackStatus;
use crate::lyrics_manager::{LyricLine, Lyrics, SyncType};
use crate::ui::components::bounds_observer::observe_bounds;
use crate::ui::components::virtual_list::{VirtualListScrollController, vlist};
use ahash::{AHashMap, AHashSet};
use gpui::prelude::FluentBuilder;
use gpui::{
    App, AppContext, Bounds, Context, Entity, FontWeight, Global, InteractiveElement, IntoElement,
    ParentElement, Pixels, Render, ScrollHandle, Styled, Window, div, gradient_color_stop,
    linear_gradient, px, rgb, rgba,
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
}

impl Render for LyricLineView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = cx.global::<Controller>().state.read(cx);
        let playback = state.playback.position;
        let lyrics_state = cx.global::<LyricsState>().0.read(cx);
        let Some(lyrics) = lyrics_state.lyrics.as_ref() else {
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
        let distance = self.idx.abs_diff(active_line) as i32;

        let inactive_opacity = match distance {
            0 => 0.35,
            1 => 0.32,
            2 => 0.26,
            3 => 0.18,
            _ => 0.10,
        };

        let active_opacity = match distance {
            0 => 1.0,
            1 => 0.75,
            2 => 0.45,
            _ => 0.2,
        };

        match self.sync_type {
            SyncType::Line => div()
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
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xffffff))
                        .opacity(if is_active_line { 1.0 } else { 0.4 })
                        .child(self.line.text.to_string()),
                )
                .into_any_element(),
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
                            .children(words.iter().enumerate().map(|(word_idx, word)| {
                                let next_start = self
                                    .line
                                    .words
                                    .as_ref()
                                    .and_then(|w| w.get(word_idx + 1))
                                    .map(|w| w.start)
                                    .unwrap_or(word.end);

                                let progress = if playback < word.start {
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
                                .clamp(0.0, 1.0);

                                let text = word.text.to_string();

                                let element = div()
                                    .relative()
                                    .flex_none()
                                    .child(
                                        div()
                                            .id(format!("base_word_{}_{}", self.idx, word_idx))
                                            .text_size(LYRICS_TEXT_SIZE)
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0xffffff))
                                            .opacity(inactive_opacity)
                                            .child(text.clone()),
                                    )
                                    .child(
                                        div()
                                            .absolute()
                                            .top_0()
                                            .left_0()
                                            .h_full()
                                            .overflow_hidden()
                                            .when_some(
                                                {
                                                    let bounds_cache = self.word_bounds.borrow();
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
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(0xffffff))
                                                    .opacity(active_opacity)
                                                    .child(text),
                                            ),
                                    );

                                let bounds_cache = self.word_bounds.clone();
                                let line_idx = self.idx;

                                if bounds_cache.borrow().contains_key(&(line_idx, word_idx)) {
                                    element
                                } else {
                                    div().child(observe_bounds(
                                        format!("word_measure_{}_{}", line_idx, word_idx),
                                        element,
                                        move |bounds, _, _| {
                                            bounds_cache
                                                .borrow_mut()
                                                .insert((line_idx, word_idx), bounds);
                                        },
                                    ))
                                }
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
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xffffff))
                        .child(self.line.text.to_string()),
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
    pub measured_lines: Rc<RefCell<AHashSet<usize>>>,
    pub list_controller: VirtualListScrollController,
    pub word_bounds: Rc<RefCell<AHashMap<(usize, usize), Bounds<Pixels>>>>,
    pub last_playback: Duration,
    pub elapsed_since_last_update: Instant,
}

impl LyricsView {
    pub fn new(cx: &mut App, scroll_handle: ScrollHandle) -> Entity<Self> {
        cx.new(|cx| Self {
            views: cx.new(|_| AHashMap::new()),
            scroll_handle,
            last_active_line: 0,
            panel_bounds: None,
            measured_heights: Vec::new(),
            measured_lines: Rc::new(RefCell::new(AHashSet::new())),
            list_controller: VirtualListScrollController::new(),
            word_bounds: Rc::new(RefCell::new(AHashMap::new())),
            last_playback: Duration::from_millis(0),
            elapsed_since_last_update: Instant::now(),
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

        let display_line = if let Some(current) = lyrics.lines.get(active_line) {
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
        };

        if display_line != self.last_active_line {
            self.last_active_line = display_line;
            self.list_controller.scroll_to_item(display_line);
        }

        let views = self.views.clone();
        let lines = lyrics.lines.clone();
        let sync_type = lyrics.sync_type.clone();
        let measured_heights = Rc::new(self.measured_heights.clone());
        let measured_lines = self.measured_lines.clone();
        let word_bounds = self.word_bounds.clone();
        let list_entity = entity.clone();

        let list = vlist(
            cx.entity(),
            "lyrics",
            measured_heights,
            self.scroll_handle.clone(),
            self.list_controller.clone(),
            move |_this, range, _, cx| {
                range
                    .map(|idx| {
                        let line = lines[idx].clone();

                        let content = div()
                            .font_family("Space Grotesk")
                            .id(("lyrics_line", idx))
                            .w_full()
                            .min_w_0()
                            .child(LyricsView::get_or_create_line(
                                &views,
                                line,
                                idx,
                                sync_type.clone(),
                                word_bounds.clone(),
                                cx,
                            ));

                        if measured_lines.borrow().contains(&idx) {
                            content.into_any_element()
                        } else {
                            observe_bounds(("lyrics_line_measure", idx), content, {
                                let entity = list_entity.clone();
                                let measured_lines = measured_lines.clone();

                                move |bounds, _, cx| {
                                    entity.update(cx, |this, _| {
                                        let height = bounds.size.height;

                                        if let Some(existing) = this.measured_heights.get_mut(idx) {
                                            if (*existing - height).abs() > px(1.0) {
                                                *existing = height;
                                            }
                                        }

                                        measured_lines.borrow_mut().insert(idx);
                                    });
                                }
                            })
                            .into_any_element()
                        }
                    })
                    .collect::<Vec<_>>()
            },
        );

        let root =
            div()
                .font_family("Space Grotesk")
                .relative()
                .w_full()
                .min_w_0()
                .h_full()
                .min_h_0()
                .child(list)
                .child(div().absolute().top_0().left_0().right_0().h(px(180.0)).bg(
                    linear_gradient(
                        180.,
                        gradient_color_stop(rgb(0x000000), 0.0),
                        gradient_color_stop(rgba(0x00000000), 1.0),
                    ),
                ))
                .child(
                    div()
                        .absolute()
                        .bottom_0()
                        .left_0()
                        .right_0()
                        .h(px(180.0))
                        .bg(linear_gradient(
                            0.,
                            gradient_color_stop(rgb(0x000000), 0.0),
                            gradient_color_stop(rgba(0x00000000), 1.0),
                        )),
                );

        observe_bounds("lyrics_panel_bounds", root, move |bounds, _, cx| {
            entity.update(cx, |this, _| {
                this.panel_bounds = Some(bounds);
            });
        })
        .into_any_element()
    }
}
