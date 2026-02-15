pub mod commands;
pub mod events;
pub mod state;

use crate::{controller::state::AppState, errors::AppError};
use commands::{AudioCommand, ScannerCommand, UiCommand};
use crossbeam_channel::{Receiver, Sender, select};
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

    // UI channel
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

    pub fn run(&mut self) -> Result<(), AppError> {
        loop {
            select! {
                recv(self.audio_rx)-> msg => {
                    if let Ok(e) = msg {
                        self.handle_audio_event(e)
                    }
                },
                recv(self.scanner_rx) -> msg => {
                    if let Ok(e) = msg {
                        self.handle_scanner_event(e)
                    }
                },
                recv(self.ui_rx) -> msg => {
                    if let Ok(e) = msg {
                        self.handle_ui_command(e)
                    }
                }
            }
        }
    }

    fn handle_audio_event(&mut self, event: AudioEvent) {}

    fn handle_scanner_event(&mut self, event: ScannerEvent) {}

    fn handle_ui_command(&mut self, cmd: UiCommand) {}
}
