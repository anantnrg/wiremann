use crossbeam_channel::{Receiver, Sender};

use crate::{
    controller::{commands::AudioCommand, events::AudioEvent},
    errors::AudioError,
};

pub struct Audio {
    pub rx: Receiver<AudioCommand>,
    pub tx: Sender<AudioEvent>,
}

impl Audio {
    pub fn new() -> (Self, Sender<AudioCommand>, Receiver<AudioEvent>) {
        let (cmd_tx, cmd_rx) = crossbeam_channel::unbounded();
        let (event_tx, event_rx) = crossbeam_channel::unbounded();
        let engine = Audio {
            rx: cmd_rx,
            tx: event_tx,
        };

        (engine, cmd_tx, event_rx)
    }

    pub fn run(&mut self) -> Result<(), AudioError> {
        Ok(())
    }
}
