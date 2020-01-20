use crate::events::command::Command;

pub enum CommandParseResult {
    Command(Command, Vec<String>),
    NoCommand,
}