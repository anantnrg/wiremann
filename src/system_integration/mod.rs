use crate::controller::{commands::SystemIntegrationCommand, events::SystemIntegrationEvent};
use crossbeam_channel::{Receiver, Sender};
use souvlaki::MediaControls;

pub struct SystemIntegration {
    pub tx: Sender<SystemIntegrationEvent>,
    pub rx: Receiver<SystemIntegrationCommand>,

    media_controls: Option<MediaControls>,
}

impl SystemIntegration {
    pub fn new() -> (
        Self,
        Sender<SystemIntegrationCommand>,
        Receiver<SystemIntegrationEvent>,
    ) {
        let (cmd_tx, cmd_rx) = crossbeam_channel::unbounded();
        let (event_tx, event_rx) = crossbeam_channel::unbounded();

        (
            Self {
                tx: event_tx,
                rx: cmd_rx,
                media_controls: None,
            },
            cmd_tx,
            event_rx,
        )
    }
}
