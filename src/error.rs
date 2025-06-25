use thiserror::Error;

#[derive(Error, Debug)]
pub enum BunkuError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parsing error: {0}")]
    TomlParsing(#[from] toml::de::Error),

    #[error("JSON serialization error: {0}")]
    JsonSerialization(#[from] serde_json::Error),

    #[error("Validation error: {0}")]
    Validation(String),
}
