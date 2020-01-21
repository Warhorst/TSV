use std::sync::Arc;

use serenity::client::bridge::voice::ClientVoiceManager;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::events::command::Command;
use crate::events::command_parse_result::CommandParseResult;
use crate::events::command_parser::CommandParser;
use crate::events::default_command_parser::DefaultCommandParser;
use crate::events::messages::Messages;

pub struct TSVEventHandler {
    command_parser: DefaultCommandParser
}

struct VoiceManager;

impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

impl TSVEventHandler {
    pub fn new(command_parser: DefaultCommandParser) -> Self {
        TSVEventHandler { command_parser }
    }

    fn join_channel(&self, ctx: Context, msg: Message) {
        let guild = match msg.guild(&ctx.cache) {
            Some(guild) => guild,
            None => {
                println!("Error guild");
                return;
            }
        };

        let guild_id = guild.read().id;

        let channel_id = guild
            .read()
            .voice_states.get(&msg.author.id)
            .and_then(|voice_state| voice_state.channel_id);

        let target_channel = match channel_id {
            Some(channel) => channel,
            None => {
                println!("Error channel");
                return;
            }
        };

        let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
        let mut manager = manager_lock.lock();

        if manager.join(guild_id, target_channel).is_some() {
            println!("Success join")
        } else {
            println!("Error join")
        }
    }

    fn unknown_command(&self, ctx: Context, msg: Message) {
        if let Err(why) = msg.channel_id.say(&ctx.http, Messages::UnknownCommand.to_string()) {
            println!("Error sending message: {:?}", why);
        }
    }
}

impl EventHandler for TSVEventHandler {
    fn message(&self, ctx: Context, msg: Message) {
        let command_parse_result = &self.command_parser.parse(&mut msg.content.clone());

        match command_parse_result {
            CommandParseResult::Command(command, _) => {
                match command {
                    Command::JoinChannel => self.join_channel(ctx, msg),
                    Command::Unknown => self.unknown_command(ctx, msg)
                }
            }
            CommandParseResult::NoCommand => ()
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{}", Messages::BotConnected(ready.user.name).to_string());
    }
}