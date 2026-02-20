pub mod assets;
pub mod res_handler;
pub mod wiremann;
mod components;
mod theme;
mod icons;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum UiError {}
