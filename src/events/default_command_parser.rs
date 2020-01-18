use crate::events::command::Command;
use crate::events::command_parser::CommandParser;

pub struct DefaultCommandParser {
    key_sign: char
}

impl DefaultCommandParser {
    pub fn new(key_sign: char) -> Self {
        DefaultCommandParser { key_sign }
    }
}

impl CommandParser for DefaultCommandParser {
    fn parse(&self, content: &mut String) -> Option<Command> {
        if content.starts_with(self.key_sign) {
            content.remove(0);

            return Some(Command::from(content.to_string()));
        }

        None
    }
}