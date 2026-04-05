#![warn(clippy::pedantic)]
// #![windows_subsystem = "windows"]
pub mod app;
pub mod audio;
mod cacher;
pub mod controller;
pub mod errors;
pub mod library;
mod queue;
pub mod scanner;
mod scanner_v2;
pub mod ui;
mod worker_config;

use errors::AppError;

fn main() -> Result<(), AppError> {
    app::run()
}
