use std::sync::Arc;

use serenity::client::bridge::voice::ClientVoiceManager;
use serenity::model::channel::{Channel, Message};
use serenity::model::gateway::Ready;
use serenity::model::id::{ChannelId, GuildId, UserId};
use serenity::model::user::User;
use serenity::model::voice::VoiceState;
use serenity::prelude::*;
use serenity::voice;

use crate::events::commands::command::Command;
use crate::events::commands::command_parse_result::CommandParseResult;
use crate::events::commands::command_parser::CommandParser;
use crate::events::commands::default_command_parser::DefaultCommandParser;
use crate::events::tsv_state::TSVState;
use crate::messages::messages::Messages;
use crate::quotes::quotes::Quotes;
use crate::VoiceManager;

static mut BOT_ID: Option<UserId> = None;
static mut BOT_CHANNEL_ID: Option<ChannelId> = None;

pub struct TSVEventHandler {
    command_parser: DefaultCommandParser
}

impl EventHandler for TSVEventHandler {
    fn message(&self, ctx: Context, msg: Message) {
        let command_parse_result = &self.command_parser.parse(&mut msg.content.clone());

        match command_parse_result {
            CommandParseResult::Command(command, _) => {
                match command {
                    Command::JoinChannel => self.join_channel(ctx, msg),
                    Command::LeaveChannel => self.leave_channel(ctx, msg),
                    Command::Play => self.speak(ctx, msg),
                    Command::Unknown => self.unknown_command(ctx, msg)
                }
            }
            CommandParseResult::NoCommand => ()
        }
    }

    fn voice_state_update(&self, ctx: Context, guild: Option<GuildId>, old: Option<VoiceState>, new: VoiceState) {
        unsafe {
            let bot_id = match BOT_ID {
                Some(id) => id,
                None => {
                    println!("No user id set!");
                    return;
                }
            };

            let bot_channel_id = match BOT_CHANNEL_ID {
                Some(id) => id,
                None => {
                    println!("No channel id set!");
                    return;
                }
            };

            if new.user_id == bot_id {
                println!("TSV joined");
                return;
            }

            match old {
                Some(old) => match (old.channel_id, new.channel_id) {
                    (None, None) => println!("strange"),
                    (None, Some(_)) => self.play_quote(Quotes::UserJoinedYourChannel, ctx, bot_channel_id),
                    (Some(_), None) => self.play_quote(Quotes::UserLeftYourChannel, ctx, bot_channel_id),
                    (Some(old_id), Some(new_id)) => {
                        if new_id == bot_channel_id {
                            self.play_quote(Quotes::UserJoinedYourChannel, ctx, bot_channel_id)
                        } else if old_id == bot_channel_id {
                            self.play_quote(Quotes::UserLeftYourChannel, ctx, bot_channel_id)
                        }
                    }
                },
                None => {
                    match new.channel_id {
                        None => println!("undefined"),
                        Some(id) => {
                            if id == bot_channel_id {
                                self.play_quote(Quotes::UserJoinedYourChannel, ctx, bot_channel_id)
                            } else {
                                println!("undefined")
                            }
                        }
                    }
                }
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        unsafe {
            BOT_ID = Some(ready.user.id)
        }
        println!("{}", Messages::BotConnected(ready.user.name).to_string());
    }
}

impl TSVEventHandler {
    pub fn new(command_parser: DefaultCommandParser) -> Self {
        TSVEventHandler {
            command_parser
        }
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

        unsafe {
            BOT_CHANNEL_ID = Some(target_channel)
        }

        let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
        let mut manager = manager_lock.lock();

        if manager.join(guild_id, target_channel).is_some() {
            println!("Success join")
        } else {
            println!("Error join")
        }
    }

    fn leave_channel(&self, ctx: Context, msg: Message) {
        let guild_id = match ctx.cache.read().guild_channel(msg.channel_id) {
            Some(channel) => channel.read().guild_id,
            None => {
                println!("Guild ID");
                return;
            }
        };

        let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
        let mut manager = manager_lock.lock();
        let has_handler = manager.get(guild_id).is_some();

        if has_handler {
            manager.remove(guild_id);
            println!("Manager success")
        } else {
            println!("Manager error")
        }
    }

    fn speak(&self, ctx: Context, msg: Message) {
        self.play_quote(Quotes::UserLeftYourChannel, ctx, msg.channel_id)
    }

    fn unknown_command(&self, ctx: Context, msg: Message) {
        self.send_message(Messages::UnknownCommand, ctx, msg);
    }

    fn send_message(&self, messages: Messages, ctx: Context, msg: Message) {
        if let Err(why) = msg.channel_id.say(&ctx.http, messages.to_string()) {
            println!("Error sending message: {:?}", why);
        }
    }

    fn play_quote(&self, quote: Quotes, ctx: Context, channel_id: ChannelId) {
        let guild_id = match ctx.cache.read().guild_channel(channel_id) {
            Some(channel) => channel.read().guild_id,
            None => {
                println!("Error guild id");
                return;
            }
        };

        let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
        let mut manager = manager_lock.lock();

        if let Some(handler) = manager.get_mut(guild_id) {
            let source = match voice::ffmpeg(&quote.get_path()) {
                Ok(source) => source,
                Err(why) => {
                    println!("Source error, {:?}", why);

                    return;
                }
            };

            handler.play(source);
        } else {
            println!("Handler error")
        }
    }
}