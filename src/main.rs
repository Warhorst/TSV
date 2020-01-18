use std::env;

use serenity::Client;

use crate::events::default_command_parser::DefaultCommandParser;
use crate::events::tsv_event_handler::TSVEventHandler;

pub mod events;

fn main() {
    let args: Vec<String> = env::args().collect();
    let token = args.get(1).unwrap();

    let mut client = Client::new(token, TSVEventHandler::new(DefaultCommandParser::new('!'))).expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
