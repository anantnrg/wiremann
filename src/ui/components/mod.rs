pub mod controlbar;
pub mod navbar;
pub mod pages;
pub mod slider;
pub mod titlebar;
mod queue;
mod image_cache;

#[derive(Clone, Copy, PartialEq)]
pub enum Page {
    Library,
    Player,
    Playlists,
    Settings,
}

impl gpui::Global for Page {}
