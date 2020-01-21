use std::env;
use std::process::Command;
use std::sync::Arc;

use serenity::Client;
use serenity::client::bridge::voice::ClientVoiceManager;
use serenity::prelude::{Mutex, TypeMapKey};

use crate::events::default_command_parser::DefaultCommandParser;
use crate::events::tsv_event_handler::TSVEventHandler;

pub mod events;

struct VoiceManager;

impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let token = args.get(1).unwrap();

    let mut client = Client::new(token, TSVEventHandler::new(DefaultCommandParser::new('?'))).expect("Err creating client");

    {
        let mut data = client.data.write();
        data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
    }

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
