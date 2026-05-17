use crate::controller::Controller;
use crate::lyrics_manager::{LyricLine, LyricWord, Lyrics, SyncType};
use ahash::AHashMap;
use gpui::{
    App, AppContext, Context, Entity, FontWeight, Global, InteractiveElement, IntoElement,
    ParentElement, Render, ScrollStrategy, Styled, UniformListScrollHandle, Window, div, px, rgb,
    uniform_list,
};
use std::time::Duration;

const LINE_HEIGHT: f32 = 72.0;

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

        let playback = Duration::from_secs(state.playback.position);
        println!("{:?}", playback);
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
                    .min_h(px(LINE_HEIGHT))
                    .px_6()
                    .py_1()
                    .flex()
                    .items_center()
                    .justify_start()
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
                    .min_h(px(LINE_HEIGHT))
                    .px_6()
                    .py_1()
                    .flex()
                    .justify_start()
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .flex_row()
                            .flex_wrap()
                            .justify_start()
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
    pub scroll_handle: UniformListScrollHandle,
    pub last_active_line: usize,
}

impl LyricsView {
    pub fn new(cx: &mut App, scroll_handle: UniformListScrollHandle) -> Entity<Self> {
        cx.new(|cx| Self {
            views: cx.new(|_| AHashMap::new()),
            scroll_handle,
            last_active_line: 0,
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
}

impl Render for LyricsView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = cx.global::<Controller>().state.read(cx);

        let playback = Duration::from_millis(state.playback.position);

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
                );
        };

        let active_line = Self::active_line(&lyrics.lines, playback);

        if active_line != self.last_active_line {
            self.last_active_line = active_line;

            let handle = self.scroll_handle.clone();

            cx.defer(move |_| {
                handle.scroll_to_item(active_line, ScrollStrategy::Center);
            });
        }

        let views = self.views.clone();

        let lines = lyrics.lines.clone();
        let sync_type = lyrics.sync_type.clone();

        div().size_full().child(
            uniform_list("lyrics", lines.len(), move |range, _, cx| {
                views.update(cx, |views, cx| {
                    for idx in range.clone() {
                        if let Some(view) = views.get(&idx) {
                            cx.notify();
                        }
                    }
                });

                range
                    .map(|idx| {
                        let line = lines[idx].clone();

                        div().id(("lyrics_line", idx)).w_full().child(
                            LyricsView::get_or_create_line(
                                &views,
                                line,
                                idx,
                                sync_type.clone(),
                                cx,
                            ),
                        )
                    })
                    .collect()
            })
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .track_scroll(&self.scroll_handle),
        )
    }
}
