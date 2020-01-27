use crate::events::commands::command::Command::{JoinChannel, LeaveChannel, Unknown};

pub enum Command {
    JoinChannel,
    LeaveChannel,
    Unknown,
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        match value {
            "join" => JoinChannel,
            "leave" => LeaveChannel,
            _ => Unknown
        }
    }
}