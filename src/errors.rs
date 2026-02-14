use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("anyhow Error occurred: `{0}`")]
    AnyHowError(#[from] anyhow::Error),
}