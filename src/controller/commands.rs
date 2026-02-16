use std::path::PathBuf;

pub enum AudioCommand {
    Load(PathBuf),
    GetPosition
}

pub enum ScannerCommand {}
