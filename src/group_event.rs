use serenity::{
    all::{Mention, MessageId, ReactionType, RoleId},
    builder::{CreateEmbed, CreateMessage},
};

use crate::poll_response::PollResponse;

pub(crate) struct GroupEvent {
    pub title: String,
    pub description: String,
    pub channel_id: u64,
    pub options: Vec<PollResponse>,
    pub role: RoleId,
}

impl GroupEvent {
    pub fn pickleball() -> Self {
        GroupEvent {
            title: "Weekly Pickleball".to_string(),
            description: "Westside Paper @ 7pm. You in?".to_string(),
            channel_id: 1119044105792659526,
            options: PollResponse::default_responses(),
            role: 1119044184213565450.into(),
        }
    }

    pub fn trivia() -> Self {
        GroupEvent {
            title: "Weekly Trivia".to_string(),
            description: "Neighbors (Virginia Highlands) @ 7:30pm. You in?".to_string(),
            channel_id: 1109898694456787124,
            options: PollResponse::default_responses(),
            role: 1131001427834183810.into(),
        }
    }

    pub fn options_string(&self) -> String {
        self.options
            .iter()
            .fold(String::new(), |acc: String, response: &PollResponse| {
                acc + &format!(
                    "{} {}\n",
                    match &response.reaction {
                        ReactionType::Unicode(reaction) => reaction,
                        _ => "unknown",
                    },
                    response.text,
                )
            })
    }

    pub fn reactions(&self) -> Vec<ReactionType> {
        self.options
            .iter()
            .map(|response| response.reaction.clone())
            .collect()
    }

    pub fn build_message(&self) -> CreateMessage {
        let role_mention = Mention::Role(self.role);

        let embed: CreateEmbed = CreateEmbed::new()
            .title(&self.title)
            .description(&self.description)
            .field("Options", self.options_string(), false);

        let message: CreateMessage = CreateMessage::new()
            .content(format!("{}", role_mention))
            .embed(embed)
            .reactions(self.reactions());

        message
    }
}
