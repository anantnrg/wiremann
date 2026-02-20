pub mod titlebar;
pub mod navbar;
pub mod pages;
pub mod controlbar;
pub mod slider;
pub mod scrollbar;
pub mod queue;
pub mod image_cache;

#[derive(Clone, Copy, PartialEq)]
pub enum Page {
    Library,
    Player,
    Playlists,
    Settings,
}

impl gpui::Global for Page {}