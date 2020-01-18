use crate::events::command::Command::Unknown;

pub enum Command {
    Unknown
}

impl From<String> for Command {
    fn from(value: String) -> Self {
        match value {
            _ => Unknown
        }
    }
}