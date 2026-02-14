#![warn(clippy::pedantic)]

pub mod app;
pub mod errors;
pub mod ui;
pub mod controller;

use errors::AppError;

fn main() -> Result<(), AppError> {
    app::run()
}
