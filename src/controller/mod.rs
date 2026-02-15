pub mod events;
pub mod commands;
pub mod state;

use crossbeam_channel::Sender;
use gpui::Entity;

pub struct Controller {
    pub state: Entity<AppState>,

    // Audio channel
    pub audio_tx: Sender<AudioCommand>,
    pub audio_rx: Receiver<AudioEvent>,

    // Scanner channel
    pub scanner_tx: Sender<ScannerCommand>,
    pub scanner_rx: Sender<ScannerEvent>
}