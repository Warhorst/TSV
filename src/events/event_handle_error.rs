use core::fmt;
use std::fmt::Formatter;

/// An error that can occur while handling an event.
#[derive(Debug)]
pub struct EventHandleError {
    message: String,
    cause: Option<serenity::Error>,
}

impl EventHandleError {
    pub fn new(message: String) -> Self {
        EventHandleError {
            message,
            cause: None,
        }
    }

    pub fn new_with_cause(message: String, cause: serenity::Error) -> Self {
        EventHandleError {
            message,
            cause: Some(cause),
        }
    }
}

impl std::error::Error for EventHandleError {}

impl std::fmt::Display for EventHandleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.cause {
            None => writeln!(f, "{}", &self.message),
            Some(e) => writeln!(f, "{}, caused by: {}", &self.message, e)
        }
    }
}