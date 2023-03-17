#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error:  {0}")]
    IO(#[from] std::io::Error),
    #[error("HTTP error:  {0}")]
    HTTP(#[from] reqwest::Error),
    #[error("Bech32 decoding error: {0}")]
    Bech32DecodingError(#[from] bech32::Error),
    #[error("Hex decoding error: {0}")]
    HexDecodingError(#[from] hex::FromHexError),
    #[error("Bit conversion error")]
    BitConversionError,
}
