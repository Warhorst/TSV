use std::ops::Add;

///Enum for every Team-Speak-Qute TSV can play.
///
/// It provides a method to get the path to the corresponding audio-file.
pub enum Quotes {
    UserJoinedYourChannel,
    UserLeftYourChannel,
    UserKickedFromServer,
}

impl Quotes {
    const BASE_PATH: &'static str = "voices/";

    pub fn get_path(&self) -> String {
        match self {
            Quotes::UserJoinedYourChannel => self.file_with_base_path("user_joined_your_channel.wav"),
            Quotes::UserLeftYourChannel => self.file_with_base_path("user_left_your_channel.wav"),
            Quotes::UserKickedFromServer => self.file_with_base_path("user_kicked_from_server.wav"),
        }
    }

    fn file_with_base_path(&self, file: &str) -> String {
        String::from(Quotes::BASE_PATH).add(file)
    }
}