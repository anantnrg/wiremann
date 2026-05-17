use crate::controller::Controller;
use crate::lyrics_manager::Lyrics;
use crate::lyrics_manager::{LyricLine, LyricWord, SyncType};
use crate::ui::theme::Theme;
use gpui::{
    Animation, AnimationExt as _, App, AppContext, Context, ElementId, Entity, FontWeight, Global,
    InteractiveElement, IntoElement, ParentElement, Render, ScrollStrategy, Styled,
    UniformListScrollHandle, Window, div, px, rgb, uniform_list,
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
        LyricsStateInner {
            status: LyricsStatus::Unavailable,
            lyrics: None,
        }
    }
}

#[derive(Clone)]
pub struct LyricsView {
    pub scroll_handle: UniformListScrollHandle,
    pub last_active_line: usize,
}

impl LyricsView {
    pub fn new(cx: &mut App, scroll_handle: UniformListScrollHandle) -> Entity<Self> {
        cx.new(|_| Self {
            scroll_handle,
            last_active_line: 0,
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

    fn active_word(words: &[LyricWord], playback: Duration) -> Option<usize> {
        words
            .iter()
            .enumerate()
            .rfind(|(_, word)| playback >= word.start)
            .map(|(idx, _)| idx)
    }
}

impl Render for LyricsView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = *cx.global::<Theme>();

        let state = cx.global::<Controller>().state.read(cx).clone();

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
                );
        };

        let lines = lyrics.lines.clone();

        let active_line = Self::active_line(&lines, Duration::from_millis(playback));

        if active_line != self.last_active_line {
            self.last_active_line = active_line;

            let handle = self.scroll_handle.clone();

            cx.defer(move |_| {
                handle.scroll_to_item(active_line, ScrollStrategy::Center);
            });
        }

        let line_count = lines.len();

        div().size_full().child(
            uniform_list("lyrics", line_count, move |range, _, cx| {
                range
                    .map(|idx| {
                        let line = lines[idx].clone();

                        let is_active_line = idx == active_line;

                        let line_opacity = if is_active_line { 0.7 } else { 0.4 };

                        match lyrics.sync_type {
                            SyncType::Line => div()
                                .id(("line", idx))
                                .w_full()
                                .h(px(LINE_HEIGHT))
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(
                                    div()
                                        .text_3xl()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(rgb(0xffffff))
                                        .opacity(line_opacity)
                                        .child(line.text),
                                ),

                            SyncType::Word => {
                                let active_word = line.words.as_ref().and_then(|words| {
                                    Self::active_word(words, Duration::from_millis(playback))
                                });

                                div()
                                    .id(("line", idx))
                                    .w_full()
                                    .h(px(LINE_HEIGHT))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .flex_wrap()
                                            .justify_center()
                                            .children(
                                                line.words
                                                    .unwrap_or_default()
                                                    .into_iter()
                                                    .enumerate()
                                                    .map(|(word_idx, word)| {
                                                        let target_opacity = if is_active_line {
                                                            if active_word
                                                                .map(|a| word_idx <= a)
                                                                .unwrap_or(false)
                                                            {
                                                                1.0
                                                            } else {
                                                                0.7
                                                            }
                                                        } else {
                                                            0.4
                                                        };

                                                        div()
                                                        .id(format!("word_{}_{}", idx, word_idx))
                                                        .text_3xl()
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .child(word.text)
                                                        .with_animation(
                                                            ElementId::NamedInteger(
                                                                "lyric_word".into(),
                                                                ((idx as u64) << 32)
                                                                    | word_idx as u64,
                                                            ),
                                                            Animation::new(Duration::from_millis(
                                                                180,
                                                            ))
                                                            .with_easing(gpui::ease_out_quint()),
                                                            move |this, delta| {
                                                                let opacity = 0.4
                                                                    + ((target_opacity - 0.4)
                                                                        * delta);

                                                                this.text_color(
                                                                    rgb(0xffffff))
                                                                    .opacity(opacity)

                                                            },
                                                        )
                                                    }),
                                            ),
                                    )
                            }
                            SyncType::Unsynced => div().id("unsynced"),
                        }
                    })
                    .collect()
            })
            .track_scroll(&self.scroll_handle)
            .flex()
            .flex_col()
            .w_full()
            .h_full(),
        )
    }
}
