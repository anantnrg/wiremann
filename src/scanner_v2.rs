use crossbeam_channel::{Receiver, Sender};
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicU64;
use std::thread;
use std::thread::JoinHandle;
use walkdir::WalkDir;

static TOTAL_TIME: AtomicU64 = AtomicU64::new(0);
static COUNT: AtomicU64 = AtomicU64::new(0);

macro_rules! time_block {
    ($name:expr, $block:expr) => {{
        let start = std::time::Instant::now();
        let result = $block;
        println!("[{}] took {:?}", $name, start.elapsed());
        result
    }};
}

pub struct ScannerV2 {
    pub tx: Sender<ScannerV2Event>,
    pub rx: Receiver<ScannerV2Command>,
}

pub enum ScannerV2Command {
    ScanFolder(PathBuf),
}

pub enum ScannerV2Event {}

impl ScannerV2 {
    #[must_use]
    pub fn new() -> (Self, Sender<ScannerV2Command>, Receiver<ScannerV2Event>) {
        let (cmd_tx, cmd_rx) = crossbeam_channel::unbounded();
        let (event_tx, event_rx) = crossbeam_channel::unbounded();

        let scanner = ScannerV2 {
            tx: event_tx,
            rx: cmd_rx,
        };

        (scanner, cmd_tx, event_rx)
    }

    pub fn start(mut self) -> JoinHandle<()> {
        thread::spawn(move || {
            println!("thread spawned");
            if let Ok(cmd) = self.rx.recv() {
                match cmd {
                    ScannerV2Command::ScanFolder(path) => {
                        self.scan_folder(path);
                    }
                }
            }
        })
    }

    pub fn scan_folder(&self, root: PathBuf) {
        let start = std::time::Instant::now();
        let mut count = 0;

        println!("starting scan...");

        let (tx, rx): (Sender<Vec<PathBuf>>, Receiver<Vec<PathBuf>>) = crossbeam_channel::bounded(64);

        for _ in 0..5 {
            let rx = rx.clone();

            thread::spawn(move || {
                for chunk in rx.iter() {
                    for path in chunk {
                        let _ = crate::scanner::metadata::read_metadata(&path);
                    }
                }
            });
        }

        let mut chunk = Vec::with_capacity(32);

        WalkDir::new(root)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.path().is_file() && Self::is_audio_file(e.path()))
            .map(|e| e.into_path())
            .for_each(|p| {
                count += 1;
                chunk.push(p);
                if chunk.len() >= 32 { tx.send(std::mem::take(&mut chunk)).unwrap() }
            });

        println!("TOTAL: {} files in {:?}", count, start.elapsed());
    }

    fn is_audio_file(path: &Path) -> bool {
        path.extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| matches!(e, "mp3" | "flac" | "wav" | "ogg" | "m4a"))
    }
}

