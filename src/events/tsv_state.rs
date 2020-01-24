use serenity::model::id::{ChannelId, UserId};

///Represents the current state of TSV
pub struct TSVState {
    tsv_id: Option<UserId>,
    tsv_channel_id: Option<ChannelId>,
}

impl TSVState {
    pub fn new() -> Self {
        TSVState {
            tsv_id: None,
            tsv_channel_id: None,
        }
    }

    pub fn get_id(&self) -> UserId {
        match self.tsv_id {
            Some(id) => id,
            None => panic!("user_id not set")
        }
    }

    pub fn get_channel(&self) -> ChannelId {
        match self.tsv_channel_id {
            Some(id) => id,
            None => panic!("channel_id not set")
        }
    }

    pub fn set_id(&mut self, id: UserId) {
        self.tsv_id = Some(id)
    }

    pub fn set_channel(&mut self, channel_id: ChannelId) {
        self.tsv_channel_id = Some(channel_id)
    }
}