use crate::events::commands::command::Command;

pub enum CommandParseResult {
    Command(Command, Vec<String>),
    NoCommand,
}