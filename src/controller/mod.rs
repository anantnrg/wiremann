pub mod commands;
pub mod events;
pub mod state;

use crate::controller::state::AppState;
use commands::{AudioCommand, ScannerCommand, UiCommand};
use crossbeam_channel::{Receiver, Sender};
use events::{AudioEvent, ScannerEvent, UiEvent};
use gpui::Entity;

#[derive(Clone)]
pub struct Controller {
    pub state: Entity<AppState>,

    // Audio channel
    pub audio_tx: Sender<AudioCommand>,
    pub audio_rx: Receiver<AudioEvent>,

    // Scanner channel
    pub scanner_tx: Sender<ScannerCommand>,
    pub scanner_rx: Receiver<ScannerEvent>,

    pub ui_rx: Receiver<UiCommand>,
    pub ui_tx: Sender<UiEvent>,
}

impl Controller {
    pub fn new(
        state: Entity<AppState>,
        audio_tx: Sender<AudioCommand>,
        audio_rx: Receiver<AudioEvent>,
        scanner_tx: Sender<ScannerCommand>,
        scanner_rx: Receiver<ScannerEvent>,
        ui_rx: Receiver<UiCommand>,
        ui_tx: Sender<UiEvent>,
    ) -> Self {
        Controller {
            state,
            audio_tx,
            audio_rx,
            scanner_tx,
            scanner_rx,
            ui_rx,
            ui_tx,
        }
    }
}
