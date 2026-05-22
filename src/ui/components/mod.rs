pub mod bounds_observer;
pub mod controlbar;
mod element_ext;
pub mod icons;
pub mod image_cache;
pub mod lyrics;
pub mod navbar;
pub mod pages;
pub mod queue;
pub mod scrollbar;
pub mod slider;
pub mod titlebar;
pub mod toasts;

#[derive(Clone, Copy, PartialEq)]
pub enum Page {
    Library,
    Player,
    Playlists,
}

impl gpui::Global for Page {}
