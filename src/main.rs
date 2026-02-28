#![warn(clippy::pedantic)]

pub mod app;
pub mod audio;
pub mod controller;
pub mod errors;
pub mod library;
pub mod scanner;
pub mod ui;
mod queue;
mod cacher;

use errors::AppError;

fn main() -> Result<(), AppError> {
    app::run()
}
