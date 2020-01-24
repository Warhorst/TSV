use crate::events::commands::command::Command::{JoinChannel, LeaveChannel, Play, Unknown};

pub enum Command {
    JoinChannel,
    LeaveChannel,
    Play,
    Unknown,
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        match value {
            "join" => JoinChannel,
            "leave" => LeaveChannel,
            "play" => Play,
            _ => Unknown
        }
    }
}