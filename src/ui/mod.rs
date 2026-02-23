pub mod assets;
pub mod res_handler;
pub mod wiremann;
pub mod components;
mod theme;
mod icons;
pub mod helpers;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum UiError {}
