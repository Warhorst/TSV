use serde::Deserialize;

/// Carries all values used to configure TSV.
#[derive(Deserialize, Debug)]
pub struct TSVConfiguration {
    user_joined_file_uri: String,
    user_left_file_uri: String,
    user_kicked_file_uri: String
}

impl TSVConfiguration {
    pub fn get_user_joined_file_uri(&self) -> &String {
        &self.user_joined_file_uri
    }

    pub fn get_user_left_file_uri(&self) -> &String {
        &self.user_left_file_uri
    }

    pub fn get_user_kicked_file_uri(&self) -> &String {
        &self.user_kicked_file_uri
    }
}