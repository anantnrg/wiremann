pub mod commands;
pub mod events;
pub mod state;
use crate::library::TrackId;
use crate::{controller::state::AppState, errors::ControllerError, library::gen_track_id};
use commands::{AudioCommand, ScannerCommand};
use crossbeam_channel::{Receiver, Sender};
use events::{AudioEvent, ScannerEvent};
use gpui::{App, Entity, Global};
use std::collections::HashSet;
use std::{path::PathBuf, sync::Arc};

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

    pub fn handle_audio_event(
        &mut self,
        cx: &mut App,
        event: &AudioEvent,
    ) -> Result<(), ControllerError> {
        match event {
            AudioEvent::Position(pos) => {
                self.state.update(cx, |this, cx| {
                    this.playback.position = *pos;
                    cx.notify();
                });
            }
            AudioEvent::TrackLoaded(path) => {
                let track_id = gen_track_id(path)?;
                if !self.state.read(cx).library.tracks.contains_key(&track_id) {
                    let _ = self.scanner_tx.send(ScannerCommand::GetTrackMetadata {
                        path: path.clone(),
                        track_id: track_id.clone(),
                    });
                }

                self.state.update(cx, |this, cx| {
                    this.playback.current = Some(track_id);
                    cx.notify();
                });
            }
        }
        Ok(())
    }

    pub fn handle_scanner_event(
        &mut self,
        cx: &mut App,
        event: &ScannerEvent,
    ) -> Result<(), ControllerError> {
        match event {
            ScannerEvent::Tracks(tracks) => {
                self.state.update(cx, |this, cx| {
                    this.library.tracks.reserve(tracks.len());
                    for track in tracks {
                        this.library
                            .tracks
                            .insert(track.id.clone(), Arc::new(track.clone()));
                    }
                    cx.notify();
                });
            }
            ScannerEvent::Playlist(playlist) => self.state.update(cx, |this, cx| {
                this.library
                    .playlists
                    .insert(playlist.id.clone(), playlist.clone());
                this.playback.current_playlist = Some(playlist.id.clone());
                this.queue.tracks = playlist.tracks.clone();
                cx.notify();
            }),
        }
        Ok(())
    }

    pub fn load_audio(&self, path: PathBuf) {
        let _ = self.audio_tx.send(AudioCommand::Load(path));
    }

    pub fn get_pos(&self) {
        let _ = self.audio_tx.send(AudioCommand::GetPosition);
    }

    pub fn scan_folder(&self, tracks: HashSet<TrackId>, path: PathBuf) {
        let _ = self
            .scanner_tx
            .send(ScannerCommand::ScanFolder { path, tracks });
    }

    pub fn play(&self) {
        let _ = self.audio_tx.send(AudioCommand::Play);
    }

    pub fn pause(&self) {
        let _ = self.audio_tx.send(AudioCommand::Pause);
    }

    pub fn stop(&self) {
        let _ = self.audio_tx.send(AudioCommand::Stop);
    }

    pub fn set_repeat(&self, cx: &mut App) {
        self.state.update(cx, |this, cx| {
            this.playback.repeat = !this.playback.repeat;
        })
    }

    pub fn set_mute(&self, cx: &mut App) {
        self.state.update(cx, |this, cx| {
            this.playback.mute = !this.playback.mute;
        })
    }

    pub fn set_volume(&self, vol: f32) {
        let _ = self.audio_tx.send(AudioCommand::SetVolume(vol));
    }

    pub fn set_shuffle(&self, cx: &mut App) {}

    pub fn next(&self) {}
    pub fn prev(&self) {}
}

impl Global for Controller {}
