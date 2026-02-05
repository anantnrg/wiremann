use super::metadata::Metadata;
use crate::audio::engine::PlaybackState;
use crate::scanner::Playlist;
use crossbeam_channel::{Receiver, Sender};
use gpui::*;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Controller {
    pub audio_cmd_tx: Sender<AudioCommand>,
    pub audio_events_rx: Receiver<AudioEvent>,
    pub scanner_cmd_tx: Sender<ScannerCommand>,
    pub scanner_events_rx: Receiver<ScannerEvent>,
    pub state: PlayerState,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerState {
    pub current: Option<PathBuf>,
    pub state: PlaybackState,
    pub position: u64,
    pub volume: f32,
    pub duration: u64,
    pub meta: Option<Metadata>,
}

pub enum AudioCommand {
    Load(String),
    Play,
    Pause,
    Volume(f32),
    Seek(u64),
    Stop,
    Meta(Metadata),
}

#[derive(Debug, Clone, PartialEq)]
pub enum AudioEvent {
    StateChanged(PlayerState),
    TrackLoaded(PathBuf),
    TrackEnded,
}

pub enum ScannerCommand {
    Load(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScannerEvent {
    Playlist(Playlist)
}

impl Controller {
    pub fn new(
        audio_cmd_tx: Sender<AudioCommand>,
        audio_events_rx: Receiver<AudioEvent>,
        scanner_cmd_tx: Sender<ScannerCommand>,
        scanner_events_rx: Receiver<ScannerEvent>,
        state: PlayerState,
    ) -> Controller {
        Controller {
            audio_cmd_tx,
            audio_events_rx,
            scanner_cmd_tx,
            scanner_events_rx,
            state,
        }
    }

    pub fn play(&self) {
        let _ = self.audio_cmd_tx.send(AudioCommand::Play);
    }

    pub fn pause(&self) {
        let _ = self.audio_cmd_tx.send(AudioCommand::Pause);
    }

    pub fn load(&self, path: String) {
        let _ = self.audio_cmd_tx.send(AudioCommand::Load(path));
    }

    pub fn volume(&self, volume: f32) {
        let _ = self.audio_cmd_tx.send(AudioCommand::Volume(volume / 100.0));
    }

    pub fn seek(&self, secs: u64) {
        let _ = self.audio_cmd_tx.send(AudioCommand::Seek(secs));
    }

    pub fn set_meta_in_engine(&self, meta: Metadata) {
        let _ = self.audio_cmd_tx.send(AudioCommand::Meta(meta));
    }
}

impl gpui::Global for Controller {}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            current: None,
            state: PlaybackState::Stopped,
            position: 0,
            volume: 1.0,
            duration: 0,
            meta: None,
        }
    }
}

#[derive(Clone, Copy)]
pub struct ResHandler {}

impl ResHandler {
    pub fn handle(&mut self, cx: &mut Context<Self>, event: AudioEvent) {
        cx.emit(event);
        cx.notify();
    }
}

pub enum PlayerStateEvent {
    Position(u64),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Track {
    pub path: PathBuf,
    pub meta: Metadata,
}

impl EventEmitter<AudioEvent> for ResHandler {}
impl EventEmitter<PlayerStateEvent> for PlayerState {}
