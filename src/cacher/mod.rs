use crate::controller::commands::CacherCommand;
use crate::controller::events::CacherEvent;
use crate::errors::CacherError;
use crossbeam_channel::{Receiver, Sender};

pub struct Cacher {
    pub tx: Sender<CacherEvent>,
    pub rx: Receiver<CacherCommand>,
}

impl Cacher {
    pub fn new() -> (Self, Sender<CacherCommand>, Receiver<CacherEvent>) {
        let (cmd_tx, cmd_rx) = crossbeam_channel::unbounded();
        let (event_tx, event_rx) = crossbeam_channel::unbounded();

        let cacher = Cacher {
            tx: event_tx,
            rx: cmd_rx,
        };

        (cacher, cmd_tx, event_rx)
    }

    pub fn run(&self) -> Result<(), CacherError> {
        loop {
            match self.rx.recv()? {
                _ => {}
            }
        }
    }
}