pub mod assets;
pub mod res_handler;
pub mod wiremann;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum UiError {}
