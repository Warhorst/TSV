use crate::events::command::Command;
use crate::events::command_parse_result::CommandParseResult;
use crate::events::command_parse_result::CommandParseResult::NoCommand;
use crate::events::command_parser::CommandParser;

pub struct DefaultCommandParser {
    key_sign: char
}

impl DefaultCommandParser {
    const SEPARATOR: &'static str = " ";

    pub fn new(key_sign: char) -> Self {
        DefaultCommandParser { key_sign }
    }
}

impl CommandParser for DefaultCommandParser {
    fn parse(&self, content: &mut String) -> CommandParseResult {
        if content.starts_with(self.key_sign) {
            content.remove(0);

            let parts = content.split(DefaultCommandParser::SEPARATOR);
            let mut args: Vec<String> = Vec::new();

            for p in parts {
                args.push(String::from(p));
            }

            let command = Command::from(args.remove(0));

            return CommandParseResult::Command(command, args);
        }

        NoCommand
    }
}