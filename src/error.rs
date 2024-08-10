#[derive(Debug)]
pub enum ConsignError {
    IoError(std::io::Error),
    TomlError(String),
    LoggingError(String),
    // SerdeError(serde_json::Error),
    PermissionsError(std::io::Error),
    FileNotFound(std::io::Error),
    GeneralFailure(String),
}

impl From<std::io::Error> for ConsignError {
    fn from(value: std::io::Error) -> Self {
        ConsignError::IoError(value)
    }
}
