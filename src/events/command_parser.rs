use crate::events::command::Command;

/// Returns a recognized command from a given string.
pub trait CommandParser {
    fn parse(&self, content: &mut String) -> Option<Command>;
}