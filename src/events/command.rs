use crate::events::command::Command::{JoinChannel, Unknown};

pub enum Command {
    JoinChannel,
    Unknown,
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        match value {
            "join" => JoinChannel,
            _ => Unknown
        }
    }
}