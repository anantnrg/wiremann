pub mod assets;
pub mod wiremann;
pub mod res_handler;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum UiError {}
