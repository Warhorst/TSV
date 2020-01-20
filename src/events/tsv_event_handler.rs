use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::events::command::Command;
use crate::events::command_parser::CommandParser;
use crate::events::default_command_parser::DefaultCommandParser;
use crate::events::messages::Messages;

pub struct TSVEventHandler {
    command_parser: DefaultCommandParser
}

impl TSVEventHandler {
    pub fn new(command_parser: DefaultCommandParser) -> Self {
        TSVEventHandler { command_parser }
    }

    fn unknown_command(&self, ctx: Context, msg: Message) {
        if let Err(why) = msg.channel_id.say(&ctx.http, Messages::UnknownCommand.to_string()) {
            println!("Error sending message: {:?}", why);
        }
    }
}

impl EventHandler for TSVEventHandler {
    fn message(&self, ctx: Context, msg: Message) {
        let command_opt = &self.command_parser.parse(&mut msg.content.clone());

        match command_opt {
            Some(command) => {
                match command {
                    Command::Unknown => self.unknown_command(ctx, msg)
                }
            }
            None => ()
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{}", Messages::BotConnected(ready.user.name).to_string());
    }
}