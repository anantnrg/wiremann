use crossbeam_channel::{Receiver, Sender};

use crate::{
    controller::{commands::ScannerCommand, events::ScannerEvent},
    errors::ScannerError,
};

pub struct Scanner {
    pub tx: Sender<ScannerEvent>,
    pub rx: Receiver<ScannerCommand>,
}

impl Scanner {
    pub fn new() -> (Self, Sender<ScannerCommand>, Receiver<ScannerEvent>) {
        let (cmd_tx, cmd_rx) = crossbeam_channel::unbounded();
        let (event_tx, event_rx) = crossbeam_channel::unbounded();

        let scanner = Scanner {
            tx: event_tx,
            rx: cmd_rx,
        };

        (scanner, cmd_tx, event_rx)
    }

    pub fn run(&mut self) -> Result<(), ScannerError> {
        loop {}
    }
}
