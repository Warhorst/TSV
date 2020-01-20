use std::ops::Add;

pub enum Messages {
    BotConnected(String),
    UnknownCommand,
}

impl ToString for Messages {
    fn to_string(&self) -> String {
        match self {
            Messages::BotConnected(name) => String::from(name).add(" is connected"),
            Messages::UnknownCommand => String::from("Unknown command!")
        }
    }
}