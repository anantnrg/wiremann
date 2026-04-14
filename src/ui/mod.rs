pub mod animations;
pub mod assets;
pub mod components;
pub mod global_keybinds;
pub mod helpers;
pub mod res_handler;
mod theme;
pub mod wiremann;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum UiError {}
