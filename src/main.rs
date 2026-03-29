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
pub mod ui;
mod worker_config;
mod scanner_v2;

use crate::scanner_v2::{ScannerV2, ScannerV2Command};
use errors::AppError;

fn main() -> Result<(), AppError> {
    let (scanner, cmd_tx, _event_rx) = ScannerV2::new();

    let handle = scanner.start();

    let path = "E:\\music\\$UMH4RD$H1T".into();

    cmd_tx.send(ScannerV2Command::ScanFolder(path)).unwrap();

    handle.join().unwrap();

    Ok(())
    // app::run()
}
