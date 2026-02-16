use std::{fs::File, path::PathBuf};

use crossbeam_channel::{Receiver, Sender, tick};

use crate::{
    controller::{commands::AudioCommand, events::AudioEvent},
    errors::AudioError,
};
use rodio::{OutputStream, OutputStreamBuilder, Sink, decoder::DecoderBuilder};

pub struct Audio {
    sink: Sink,
    stream_handle: OutputStream,

    pub rx: Receiver<AudioCommand>,
    pub tx: Sender<AudioEvent>,
}

impl Audio {
    pub fn new() -> (Self, Sender<AudioCommand>, Receiver<AudioEvent>) {
        let (cmd_tx, cmd_rx) = crossbeam_channel::unbounded();
        let (event_tx, event_rx) = crossbeam_channel::unbounded();
        let stream_handle = OutputStreamBuilder::open_default_stream().unwrap();
        let sink = Sink::connect_new(&stream_handle.mixer());

        let engine = Audio {
            stream_handle,
            sink,
            rx: cmd_rx,
            tx: event_tx,
        };

        (engine, cmd_tx, event_rx)
    }

    pub fn run(&mut self) -> Result<(), AudioError> {
        loop {
            while let Ok(cmd) = self.rx.try_recv() {
                match cmd {
                    AudioCommand::Load(path) => self.load_path(PathBuf::from(path))?,
                    AudioCommand::GetPosition => self.emit_position()?,
                }
            }
        }
    }

    fn load_path(&mut self, path: PathBuf) -> Result<(), AudioError> {
        self.sink.stop();
        self.sink = Sink::connect_new(self.stream_handle.mixer());

        let file = File::open(path.clone()).unwrap();
        let len = file.metadata().unwrap().len();
        let source = DecoderBuilder::new()
            .with_data(file)
            .with_byte_len(len)
            .with_seekable(true)
            .build()
            .unwrap();

        self.sink.append(source);

        let _ = self.tx.send(AudioEvent::TrackLoaded(path));

        Ok(())
    }

    fn emit_position(&self) -> Result<(), AudioError> {
        let _ = self.tx.send(AudioEvent::Position(self.sink.get_pos().as_secs()));
        Ok(())
    }
}
