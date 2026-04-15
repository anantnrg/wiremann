use std::time::Duration;

use crate::{controller::{commands::SystemIntegrationCommand, events::SystemIntegrationEvent, state::PlaybackStatus}, errors::SystemIntegrationError};
use crossbeam_channel::{Receiver, Sender, select};
use raw_window_handle::RawWindowHandle;
use souvlaki::{MediaControls, MediaMetadata, MediaPlayback, MediaPosition, PlatformConfig};

pub struct SystemIntegration {
    pub tx: Sender<SystemIntegrationEvent>,
    pub rx: Receiver<SystemIntegrationCommand>,

    media_controls: Option<MediaControls>,
}

impl SystemIntegration {
    #[allow(unused_variables)]
    pub fn new(raw_window_handle: Option<RawWindowHandle>) -> (
        Self,
        Sender<SystemIntegrationCommand>,
        Receiver<SystemIntegrationEvent>,
    ) {
        let (cmd_tx, cmd_rx) = crossbeam_channel::unbounded();
        let (event_tx, event_rx) = crossbeam_channel::unbounded();

        #[cfg(not(target_os = "windows"))]
        let hwnd = None;

        #[cfg(target_os = "windows")]
        let hwnd = raw_window_handle.and_then(|handle| {
            let handle = match handle {
                RawWindowHandle::Win32(h) => h,
                _ => unreachable!()
            };
            Some(handle.hwnd)
        });

        let config = PlatformConfig {
            hwnd,
            dbus_name: "app.wiremann.wiremann",
            display_name: "Wiremann"
        };

        let media_controls = MediaControls::new(config).ok();

        (
            Self {
                tx: event_tx,
                rx: cmd_rx,
                media_controls,
            },
            cmd_tx,
            event_rx,
        )
    }

    pub fn run(&mut self) -> Result<(), SystemIntegrationError> {
        let (souvlaki_tx, souvlaki_rx) = crossbeam_channel::unbounded();

        if let Some(controls) = &mut self.media_controls {
            controls.attach(move |event| {souvlaki_tx.send(event).ok();})?;

            loop {
                select! {
                    recv(self.rx) -> msg => {
                        if let Ok(cmd) = msg {self.handle_commands(cmd)?;}
                    }
                }
            }
        }

        Ok(())
    }

    pub fn handle_commands(&mut self, cmd: SystemIntegrationCommand) -> Result<(), SystemIntegrationError> {
        if let Some(controls) = &mut self.media_controls {
            match cmd {
                SystemIntegrationCommand::SetMetadata { title, artist, album, image, duration } => {
                    controls.set_metadata(MediaMetadata {
                        title: Some(title.as_str()),
                        album: Some(album.as_str()),
                        artist: Some(artist.as_str()),
                        cover_url: None,
                        duration: Some(Duration::from_secs(duration))
                    })?;
                }
                SystemIntegrationCommand::SetPosition(pos) => {
                    controls.set_playback(MediaPlayback::Playing { progress: Some(MediaPosition(Duration::from_secs(pos))) })?;
                }
                SystemIntegrationCommand::SetPlaybackStatus(status, pos) => {
                    let status = match status {
                        PlaybackStatus::Stopped => MediaPlayback::Stopped,
                        PlaybackStatus::Paused => MediaPlayback::Paused { progress: Some(MediaPosition(Duration::from_secs(pos))) },
                        PlaybackStatus::Playing => MediaPlayback::Playing { progress: Some(MediaPosition(Duration::from_secs(pos))) },
                    };
                    controls.set_playback(status)?;
                }
            }
        }

        Ok(())
    }
}
