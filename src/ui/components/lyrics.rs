use crate::controller::Controller;
use crate::controller::state::PlaybackStatus;
use crate::lyrics_manager::{LyricLine, Lyrics, SyncType};
use crate::ui::components::bounds_observer::observe_bounds;
use crate::ui::components::virtual_list::{VirtualListScrollController, vlist};

use ahash::{AHashMap, AHashSet};

use gpui::{
    App, AppContext, Bounds, Context, Entity, FontWeight, Global, InteractiveElement, IntoElement,
    ParentElement, Pixels, Render, ScrollHandle, Styled, Window, div, linear_color_stop, px, rgb,
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
    pub primary_line: usize,
    pub active_words: Rc<AHashSet<(usize, usize)>>,
    pub playback: Duration,
    pub word_bounds: Rc<RefCell<AHashMap<(usize, usize), Bounds<Pixels>>>>,
    pub line_bounds: Rc<RefCell<AHashMap<usize, Bounds<Pixels>>>>,
}

impl LyricLineView {
    pub fn new(
        cx: &mut App,
        line: LyricLine,
        idx: usize,
        sync_type: SyncType,
        primary_line: usize,
        active_words: Rc<AHashSet<(usize, usize)>>,
        playback: Duration,
        word_bounds: Rc<RefCell<AHashMap<(usize, usize), Bounds<Pixels>>>>,
        line_bounds: Rc<RefCell<AHashMap<usize, Bounds<Pixels>>>>,
    ) -> Entity<Self> {
        cx.new(|_| Self {
            line,
            idx,
            sync_type,
            primary_line,
            active_words,
            playback,
            word_bounds,
            line_bounds,
        })
    }

    fn line_progress(&self) -> f32 {
        let Some(words) = &self.line.words else {
            return 0.0;
        };

        if words.is_empty() {
            return 0.0;
        }

        let word_bounds = self.word_bounds.borrow();
        let line_bounds = self.line_bounds.borrow();

        let Some(line_bounds) = line_bounds.get(&self.idx) else {
            return 0.0;
        };

        let line_width = line_bounds.size.width.to_f64() as f32;

        if line_width <= 0.0 {
            return 0.0;
        }

        for (word_idx, word) in words.iter().enumerate() {
            let next_start = words
                .get(word_idx + 1)
                .map(|w| w.start)
                .or(self.line.end)
                .unwrap_or(word.end);

            if self.playback < word.start {
                break;
            }

            let Some(current_bounds) = word_bounds.get(&(self.idx, word_idx)) else {
                continue;
            };

            let progress = if self.playback >= next_start {
                1.0
            } else {
                let elapsed = self.playback.saturating_sub(word.start);

                let duration = next_start.saturating_sub(word.start);

                if duration.is_zero() {
                    1.0
                } else {
                    elapsed.as_secs_f32() / duration.as_secs_f32()
                }
            }
            .clamp(0.0, 1.0);

            let prev_width = if word_idx == 0 {
                0.0
            } else {
                word_bounds
                    .get(&(self.idx, word_idx - 1))
                    .map(|b| b.size.width.to_f64() as f32)
                    .unwrap_or(0.0)
            };

            let current_width = current_bounds.size.width.to_f64() as f32;

            let reveal = prev_width + (current_width - prev_width) * progress;

            return (reveal / line_width).clamp(0.0, 1.0);
        }

        0.0
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

            SyncType::Word => {
                let progress = self.line_progress();

                div()
                    .id(("line", self.idx))
                    .w_full()
                    .py_2()
                    .px_4()
                    .flex()
                    .justify_center()
                    .child(
                        div()
                            .relative()
                            .max_w_full()
                            .child(observe_bounds(
                                ("line_bounds", self.idx),
                                div()
                                    .text_center()
                                    .text_size(LYRICS_TEXT_SIZE)
                                    .font_weight(FontWeight::BOLD)
                                    .text_gradient_horizontal(
                                        linear_color_stop(
                                            rgb(0xffffff),
                                            (progress - 0.01).max(0.0),
                                        ),
                                        linear_color_stop(
                                            rgb(0x666666),
                                            (progress + 0.01).min(1.0),
                                        ),
                                    )
                                    .child(self.line.text.to_string()),
                                {
                                    let line_bounds = self.line_bounds.clone();

                                    let idx = self.idx;

                                    move |bounds, _, _| {
                                        line_bounds.borrow_mut().insert(idx, bounds);
                                    }
                                },
                            ))
                            .child(
                                div()
                                    .absolute()
                                    .top_0()
                                    .left_0()
                                    .opacity(0.0)
                                    .text_size(LYRICS_TEXT_SIZE)
                                    .font_weight(FontWeight::BOLD)
                                    .children(
                                        self.line
                                            .words
                                            .clone()
                                            .as_ref()
                                            .map(|words| {
                                                let mut accum = String::new();

                                                words.iter().enumerate().map(
                                                    move |(word_idx, word)| {
                                                        if !accum.is_empty() {
                                                            accum.push(' ');
                                                        }

                                                        accum.push_str(&word.text);

                                                        let measure_text = accum.clone();

                                                        observe_bounds(
                                                            format!(
                                                                "word_measure_{}_{}",
                                                                self.idx, word_idx
                                                            ),
                                                            div()
                                                                .text_size(LYRICS_TEXT_SIZE)
                                                                .font_weight(FontWeight::BOLD)
                                                                .child(measure_text),
                                                            {
                                                                let word_bounds =
                                                                    self.word_bounds.clone();

                                                                let line_idx = self.idx;

                                                                move |bounds, _, _| {
                                                                    word_bounds
                                                                        .borrow_mut()
                                                                        .insert(
                                                                            (line_idx, word_idx),
                                                                            bounds,
                                                                        );
                                                                }
                                                            },
                                                        )
                                                    },
                                                )
                                            })
                                            .into_iter()
                                            .flatten(),
                                    ),
                            ),
                    )
            }
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

    pub primary_line: usize,
    pub active_words: Rc<AHashSet<(usize, usize)>>,
    pub last_scrolled_line: usize,

    pub interpolated_playback: Duration,
    pub last_raw_playback: Duration,
    pub was_playing: bool,
    pub elapsed_since_last_update: Instant,

    pub word_bounds: Rc<RefCell<AHashMap<(usize, usize), Bounds<Pixels>>>>,

    pub line_bounds: Rc<RefCell<AHashMap<usize, Bounds<Pixels>>>>,
}

impl LyricsView {
    pub fn new(cx: &mut App, scroll_handle: ScrollHandle) -> Entity<Self> {
        cx.new(|cx| Self {
            views: cx.new(|_| AHashMap::new()),

            scroll_handle,
            list_controller: VirtualListScrollController::new(),

            measured_heights: Vec::new(),
            measured_lines: Rc::new(RefCell::new(AHashSet::new())),

            primary_line: 0,
            active_words: Rc::new(AHashSet::new()),
            last_scrolled_line: 0,

            interpolated_playback: Duration::ZERO,
            last_raw_playback: Duration::ZERO,
            was_playing: false,
            elapsed_since_last_update: Instant::now(),

            word_bounds: Rc::new(RefCell::new(AHashMap::new())),

            line_bounds: Rc::new(RefCell::new(AHashMap::new())),
        })
    }

    fn update_active_indices(&mut self, lyrics: &Lyrics, playback: Duration) {
        if lyrics.lines.is_empty() {
            self.primary_line = 0;
            self.active_words = Rc::new(AHashSet::new());
            return;
        }

        let current_start = lyrics.lines[self.primary_line]
            .start
            .unwrap_or(Duration::ZERO);

        if playback < current_start {
            self.primary_line = lyrics
                .lines
                .partition_point(|line| line.start.unwrap_or(Duration::ZERO) <= playback)
                .saturating_sub(1);
        }

        while self.primary_line + 1 < lyrics.lines.len() {
            let next_start = lyrics.lines[self.primary_line + 1]
                .start
                .unwrap_or(Duration::MAX);

            if playback >= next_start {
                self.primary_line += 1;
            } else {
                break;
            }
        }

        let mut active_words = AHashSet::new();

        let start = self.primary_line.saturating_sub(1);

        let end = (self.primary_line + 2).min(lyrics.lines.len());

        for line_idx in start..end {
            let line = &lyrics.lines[line_idx];

            if let Some(words) = &line.words {
                for (word_idx, word) in words.iter().enumerate() {
                    let next_start = words
                        .get(word_idx + 1)
                        .map(|w| w.start)
                        .or(line.end)
                        .unwrap_or(Duration::MAX);

                    if playback >= word.start && playback < next_start {
                        active_words.insert((line_idx, word_idx));
                    }
                }
            }
        }

        self.active_words = Rc::new(active_words);
    }

    fn update_playback(
        &mut self,
        window: &Window,
        raw_playback: Duration,
        playing: bool,
    ) -> Duration {
        let now = Instant::now();

        let frame_delta = now.duration_since(self.elapsed_since_last_update);

        let drift = if raw_playback > self.interpolated_playback {
            raw_playback - self.interpolated_playback
        } else {
            self.interpolated_playback - raw_playback
        };

        if drift > Duration::from_millis(400) {
            self.interpolated_playback = raw_playback;
        } else {
            if playing {
                self.interpolated_playback += frame_delta;

                if raw_playback > self.interpolated_playback {
                    self.interpolated_playback = raw_playback;
                }
            } else {
                self.interpolated_playback = raw_playback;
            }
        }

        self.last_raw_playback = raw_playback;
        self.was_playing = playing;
        self.elapsed_since_last_update = now;

        if playing {
            window.request_animation_frame();
        }

        self.interpolated_playback
    }
}

impl Render for LyricsView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity();

        let state = cx.global::<Controller>().state.read(cx);

        let playback = self.update_playback(
            window,
            state.playback.position,
            state.playback.status == PlaybackStatus::Playing,
        );

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

        self.update_active_indices(lyrics, playback);

        let primary_line = self.primary_line;

        if primary_line != self.last_scrolled_line {
            self.last_scrolled_line = primary_line;

            self.list_controller.scroll_to_item(primary_line);
        }

        let lines = lyrics.lines.clone();
        let sync_type = lyrics.sync_type.clone();

        let measured_heights = Rc::new(self.measured_heights.clone());

        let measured_lines = self.measured_lines.clone();

        let list_entity = entity.clone();

        let word_bounds = self.word_bounds.clone();

        let line_bounds = self.line_bounds.clone();

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
                            Rc::new(AHashSet::new()),
                            playback,
                            word_bounds.clone(),
                            line_bounds.clone(),
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
