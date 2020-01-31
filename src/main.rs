use std::env;
use std::sync::Arc;

use serenity::Client;
use serenity::client::bridge::voice::ClientVoiceManager;
use serenity::prelude::{Mutex, TypeMapKey};

use crate::events::commands::default_command_parser::DefaultCommandParser;
use crate::events::tsv_event_handler::TSVEventHandler;

pub mod events;
pub mod messages;
pub mod quotes;
pub mod configuration;

struct VoiceManager;

impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

fn main() {
    let token = env::var("TSV_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::new(token, TSVEventHandler::new(DefaultCommandParser::new('?'))).expect("Err creating client");

    {
        let mut data = client.data.write();
        data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
    }

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
