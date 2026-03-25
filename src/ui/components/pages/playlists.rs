use crate::controller::state::LibraryState;
use crate::controller::Controller;
use crate::library::playlists::PlaylistId;
use crate::library::TrackId;
use crate::ui::components::scrollbar::{floating_scrollbar, RightPad};
use crate::ui::components::virtual_list::vlist;
use crate::ui::helpers::{fingerprint_playlists, fingerprint_tracks};
use crate::ui::theme::Theme;
use gpui::prelude::FluentBuilder;
use gpui::{div, px, uniform_list, App, AppContext, Context, Entity, FontWeight, InteractiveElement, IntoElement, ParentElement, Pixels, Render, ScrollHandle, StatefulInteractiveElement, Styled, UniformListScrollHandle, Window};
use std::rc::Rc;

const THUMBNAIL_MARGIN: usize = 16;

#[derive(Clone)]
pub struct PlaylistsPage {
    sidebar_scroll_handle: UniformListScrollHandle,
    main_scroll_handle: ScrollHandle,

    rows: Rc<Vec<PlaylistsRows>>,
    heights: Rc<Vec<Pixels>>,

    selected_playlist: Entity<Option<PlaylistId>>,
    last_fp: u128,
}

#[derive(Clone)]
enum PlaylistsRows {
    Header,
    TrackTableHeader,
    TrackRow(usize, TrackId),
}

impl PlaylistsPage {
    pub fn new(cx: &mut App) -> Self {
        let sidebar_scroll_handle = UniformListScrollHandle::new();
        let main_scroll_handle = ScrollHandle::new();

        PlaylistsPage {
            sidebar_scroll_handle,
            main_scroll_handle,
            rows: Rc::new(Vec::new()),
            heights: Rc::new(Vec::new()),
            selected_playlist: cx.new(|_| None),
            last_fp: 0,
        }
    }
}

impl Render for PlaylistsPage {
    #[allow(clippy::too_many_lines)]
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>().clone();

        let controller = cx.global::<Controller>().clone();
        let state = controller.state.read(cx);
        let sidebar_scroll_handle = self.sidebar_scroll_handle.clone();
        let main_scroll_handle = self.main_scroll_handle.clone();

        let tracks_fp = fingerprint_tracks(state.library.tracks.keys().cloned());
        let playlists_fp = fingerprint_playlists(state.library.playlists.keys().cloned());

        let combined_fp = tracks_fp ^ playlists_fp;

        let rows = self.rows.clone();
        let heights = self.heights.clone();

        let playlists: Vec<_> = state.library.playlists.values().cloned().collect();
        let selected = self.selected_playlist.clone();
        let len = playlists.len();

        div()
            .size_full()
            .bg(theme.bg_main)
            .text_color(theme.text_primary)
            .flex()
            .child(
                div().w_1_3().h_full().flex().flex_col().gap_3()
                    .bg(theme.bg_queue)
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .items_center()
                            .justify_start()
                            .p_4()
                            .child(
                                div()
                                    .text_base()
                                    .text_color(theme.text_primary)
                                    .font_weight(FontWeight(500.0))
                                    .child("Playlists"),
                            ),
                    )
                    .child(
                        uniform_list("playlist_sidebar", len, move |range, _, cx| {
                            range.map(|i| {
                                let playlist = &playlists[i];

                                div()
                                    .id(format!("playlist_sidebar_{}", playlist.id.0))
                                    .px_4()
                                    .py_3()
                                    .cursor_pointer()
                                    .rounded_md()
                                    .hover(|d| d.bg(theme.accent_10))
                                    .when(
                                        Some(playlist.id) == *self.selected_playlist.read(cx),
                                        |d| d.bg(theme.accent_15),
                                    )
                                    .on_click({
                                        let id = playlist.id;
                                        move |_, _, cx| {
                                            selected.update(cx, |this, cx| {
                                                *this = Some(id);
                                                cx.notify();
                                            });
                                        }
                                    })
                                    .child(playlist.name.clone())
                            }).collect::<Vec<_>>()
                        })
                            .track_scroll(&sidebar_scroll_handle)
                    )
                    .child(floating_scrollbar(
                        "queue_scrollbar",
                        self.sidebar_scroll_handle.clone(),
                        RightPad::Pad,
                    ))
            )
            .child(
                div().w_full().h_full().flex().flex_grow().child(vlist(
                    cx.entity(),
                    "playlists_main",
                    heights.clone(),
                    main_scroll_handle,
                    move |_this, range, _, cx| {
                        let len = rows.len();

                        let start = range.start.saturating_sub(THUMBNAIL_MARGIN);
                        let end = (range.end + THUMBNAIL_MARGIN).min(len);

                        let thumb_track_ids: Vec<TrackId> = (start..end)
                            .filter_map(|idx| match &rows[idx] {
                                PlaylistsRows::TrackRow(_, id) => Some(*id),
                                _ => None,
                            })
                            .collect();

                        controller.request_track_thumbnails(&thumb_track_ids, cx);

                        range
                            .map(|idx| match &rows[idx] {
                                PlaylistsRows::Header => Self::render_header(heights[idx], cx),

                                PlaylistsRows::TrackTableHeader => {
                                    Self::render_track_table_header(heights[idx], cx)
                                }

                                PlaylistsRows::TrackRow(i, id) => {
                                    Self::render_track(*i, id, heights[idx], cx)
                                }
                            })
                            .collect::<Vec<_>>()
                    },
                ))
                    .child(floating_scrollbar(
                        "queue_scrollbar",
                        self.main_scroll_handle.clone(),
                        RightPad::Pad,
                    ))
            )
    }
}

fn build_rows(library: &LibraryState) -> (Vec<PlaylistsRows>, Vec<Pixels>) {
    let mut rows = Vec::new();
    let mut heights = Vec::new();

    rows.push(PlaylistsRows::Header);
    heights.push(px(120.0));

    if !library.tracks.is_empty() {
        let mut sorted_tracks: Vec<_> = library.tracks.values().collect();

        sorted_tracks.sort_by(|a, b| a.title.cmp(&b.title));

        rows.push(PlaylistsRows::TrackTableHeader);
        heights.push(px(40.0));

        for (i, track) in sorted_tracks.iter().enumerate() {
            rows.push(PlaylistsRows::TrackRow(i + 1, track.id));
            heights.push(px(60.0));
        }
    }

    (rows, heights)
}
