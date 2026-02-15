pub mod commands;
pub mod events;
pub mod state;

use std::{path::PathBuf, time::Duration};

use crate::controller::state::AppState;
use commands::{AudioCommand, ScannerCommand};
use crossbeam_channel::{Receiver, Sender, select};
use events::{AudioEvent, ScannerEvent};
use gpui::{App, Entity, Global};

#[derive(Clone)]
pub struct Controller {
    pub state: Entity<AppState>,

    // Audio channel
    pub audio_tx: Sender<AudioCommand>,
    pub audio_rx: Receiver<AudioEvent>,

    // Scanner channel
    pub scanner_tx: Sender<ScannerCommand>,
    pub scanner_rx: Receiver<ScannerEvent>,
}

impl Controller {
    pub fn new(
        state: Entity<AppState>,
        audio_tx: Sender<AudioCommand>,
        audio_rx: Receiver<AudioEvent>,
        scanner_tx: Sender<ScannerCommand>,
        scanner_rx: Receiver<ScannerEvent>,
    ) -> Self {
        Controller {
            state,
            audio_tx,
            audio_rx,
            scanner_tx,
            scanner_rx,
        }
    }

    pub fn handle_audio_event(&mut self, cx: &mut App, event: &AudioEvent) {
        match event {
            AudioEvent::Position(pos) => {
                self.state.update(cx, |this, cx| {
                    this.playback.position = Duration::from_secs(*pos);
                    cx.notify();
                });
            }
            AudioEvent::TrackLoaded(path) => {
                self.state.update(cx, |this, cx| {
                    println!("loaded: {:#?}", path.to_str());
                    cx.notify();
                });
            }
        }
    }

    pub fn handle_scanner_event(&mut self, event: &ScannerEvent) {}

    pub fn load_audio(&self, path: PathBuf) {
        let _ = self.audio_tx.send(AudioCommand::Load(path));
    }
}

impl Global for Controller {}
