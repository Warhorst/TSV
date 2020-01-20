use crate::events::command_parse_result::CommandParseResult;

/// Returns a recognized command from a given string.
pub trait CommandParser {
    fn parse(&self, content: &mut String) -> CommandParseResult;
}