#![warn(clippy::pedantic)]

pub mod app;
pub mod errors;
pub mod ui;
pub mod controller;

use errors::Error;

fn main() -> Result<(), Error> {
    app::run()
}
