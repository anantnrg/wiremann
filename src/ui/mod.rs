pub mod assets;
pub mod wiremann;
pub mod res_handler;
pub mod commander;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum UiError {}

