use crate::controller::{
    metadata::Metadata,
    player::{AudioCommand, AudioEvent, PlayerState},
};
use crossbeam_channel::{Receiver, Sender, select, tick};
use rodio::{OutputStream, OutputStreamBuilder, Sink, decoder::DecoderBuilder};
use std::{fs::File, path::PathBuf, time::Duration};

pub struct AudioEngine {
    sink: Sink,
    stream_handle: OutputStream,
    player_state: PlayerState,
    audio_rx: Receiver<AudioCommand>,
    event_tx: Sender<AudioEvent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum PlaybackState {
    #[default]
    Stopped,
    Playing,
    Paused,
}

impl AudioEngine {
    pub fn run(audio_rx: Receiver<AudioCommand>, event_tx: Sender<AudioEvent>) {
        let stream_handle = OutputStreamBuilder::open_default_stream().unwrap();
        let sink = Sink::connect_new(&stream_handle.mixer());

        let mut engine = AudioEngine {
            sink,
            stream_handle,
            player_state: PlayerState::default(),
            audio_rx,
            event_tx,
        };

        engine.event_loop();
    }

    fn event_loop(&mut self) {
        let ticker = tick(Duration::from_millis(500));

        loop {
            select! {
                recv(self.audio_rx) -> msg => {
                    let cmd = match msg {
                        Ok(c) => c,
                        Err(_) => break,
                    };

                    match cmd {
                        AudioCommand::Load(path) => self.load(PathBuf::from(path)),
                        AudioCommand::Play => self.play(),
                        AudioCommand::Pause => self.pause(),
                        AudioCommand::Stop => self.stop(),
                        AudioCommand::Volume(vol) => self.set_volume(vol),
                        AudioCommand::Seek(pos) => self.seek(pos),
                        AudioCommand::Meta(meta) => self.meta(meta)
                    }
                }

                recv(ticker) -> _ => {
                    self.emit_position();
                }
            }
        }
    }

    fn load(&mut self, path: PathBuf) {
        self.sink.stop();
        self.sink = Sink::connect_new(self.stream_handle.mixer());
        self.player_state.current = Some(path.clone());

        let file = File::open(path.clone()).unwrap();
        let len = file.metadata().unwrap().len();
        let source = DecoderBuilder::new()
            .with_data(file)
            .with_byte_len(len)
            .with_seekable(true)
            .build()
            .unwrap();

        self.sink.set_volume(self.player_state.volume);
        self.sink.append(source);

        let _ = self.event_tx.send(AudioEvent::TrackLoaded(path));

        self.player_state.state = PlaybackState::Playing;

        let _ = self
            .event_tx
            .send(AudioEvent::StateChanged(self.player_state.clone()));
    }

    fn meta(&mut self, meta: Metadata) {
        self.player_state.meta = Some(meta);
        self.send_player_state();
    }

    fn play(&mut self) {
        if self.player_state.state != PlaybackState::Playing {
            self.sink.play();
            self.player_state.state = PlaybackState::Playing;
            let _ = self
                .event_tx
                .send(AudioEvent::StateChanged(self.player_state.clone()));
        }
    }

    fn pause(&mut self) {
        if self.player_state.state == PlaybackState::Playing {
            self.sink.pause();
            self.player_state.state = PlaybackState::Paused;
            let _ = self
                .event_tx
                .send(AudioEvent::StateChanged(self.player_state.clone()));
        }
    }

    fn stop(&mut self) {
        self.sink.stop();
        self.player_state.state = PlaybackState::Stopped;
        let _ = self
            .event_tx
            .send(AudioEvent::StateChanged(self.player_state.clone()));
    }

    fn set_volume(&mut self, volume: f32) {
        self.player_state.volume = volume.clamp(0.0, 1.0);
        self.sink.set_volume(self.player_state.volume);
        let _ = self
            .event_tx
            .send(AudioEvent::StateChanged(self.player_state.clone()));
    }

    fn send_player_state(&mut self) {
        let _ = self
            .event_tx
            .send(AudioEvent::StateChanged(self.player_state.clone()));
    }

    fn emit_position(&mut self) {
        if self.player_state.state == PlaybackState::Playing {
            self.player_state.position = self.sink.get_pos().as_secs();
            self.send_player_state();
        }
    }

    fn seek(&mut self, pos: u64) {
        self.sink.try_seek(Duration::from_secs(pos)).unwrap();
    }
}
