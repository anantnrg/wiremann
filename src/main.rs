#![warn(clippy::pedantic)]
#![allow(
    clippy::unreadable_literal,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::type_complexity,
    clippy::too_many_lines,
    clippy::new_without_default
)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod app;
pub mod audio;
mod cacher;
pub mod controller;
pub mod errors;
pub mod image_processor;
pub mod lyrics_manager;
pub mod scanner;
pub mod system_integration;
pub mod ui;
mod worker_config;

use errors::AppError;

fn main() -> Result<(), AppError> {
    if cfg!(debug_assertions) {
        eprintln!("WARNING: running in debug mode — performance will be garbage");
    }

    app::run()
}
