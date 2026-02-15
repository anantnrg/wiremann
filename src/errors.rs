use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("anyhow Error occurred: `{0}`")]
    AnyHowError(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum AudioError {
    #[error("Failed to load audio file: `{0}`")]
    LoadFile(String)
}

#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("Failed to load folder: `{0}`")]
    LoadFolder(String)
}