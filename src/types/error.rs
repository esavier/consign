#[derive(Debug)]
pub enum ConsignError {
    IoError(std::io::Error),
    DirNotFound(std::io::Error),
    TomlError(String),
    LoggingError(String),
    // SerdeError(serde_json::Error),
    PermissionsError(std::io::Error),
    FileNotFound(std::io::Error),
    GeneralFailure(String),
    DeserializationError(serde_json::Error),
    SerializationError(serde_json::Error),
    ParseError(String),
}

impl From<std::io::Error> for ConsignError {
    fn from(value: std::io::Error) -> Self {
        ConsignError::IoError(value)
    }
}
