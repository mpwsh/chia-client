#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error:  {0}")]
    IO(#[from] std::io::Error),
    #[error("HTTP error:  {0}")]
    HTTP(#[from] reqwest::Error),
}
