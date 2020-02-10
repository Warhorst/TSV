use std::fmt::Formatter;
use core::fmt;
use std::io;

/// Encapsulates all possible errors that can occur during a configuration read execution.
#[derive(Debug)]
pub enum ConfigurationReadError {
    IOError(io::Error),
    JSONParseError(serde_json::error::Error),
}

impl std::error::Error for ConfigurationReadError {}

impl std::fmt::Display for ConfigurationReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ConfigurationReadError::IOError(e) => e.fmt(f),
            ConfigurationReadError::JSONParseError(e) => e.fmt(f)
        }
    }
}

impl From<io::Error> for ConfigurationReadError {
    fn from(error: io::Error) -> Self {
        ConfigurationReadError::IOError(error)
    }
}

impl From<serde_json::error::Error> for ConfigurationReadError {
    fn from(error: serde_json::error::Error) -> Self {
        ConfigurationReadError::JSONParseError(error)
    }
}