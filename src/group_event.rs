use serenity::{
    all::ReactionType,
    builder::{CreateEmbed, CreateMessage},
};

use crate::poll_response::PollResponse;

pub(crate) struct GroupEvent {
    pub title: String,
    pub description: String,
    pub options: Vec<PollResponse>,
}

impl GroupEvent {
    pub fn pickleball() -> Self {
        GroupEvent {
            title: "Weekly Pickleball".to_string(),
            description: "Westside Paper @ 7pm. You in?".to_string(),
            options: PollResponse::default_responses(),
        }
    }

    pub fn trivia() -> Self {
        GroupEvent {
            title: "Weekly Trivia".to_string(),
            description: "Eventide Brewing @ 7pm. You in?".to_string(),
            options: PollResponse::default_responses(),
        }
    }

    pub fn options_string(&self) -> String {
        let options_string = self.options.iter().fold(String::new(), |acc, response| {
            acc + &format!(
                "{} - {}\n",
                response.text,
                match &response.reaction {
                    ReactionType::Unicode(reaction) => reaction,
                    _ => "unknown",
                }
            )
        });

        options_string
    }

    pub fn reactions(&self) -> Vec<ReactionType> {
        self.options
            .iter()
            .map(|response: &PollResponse| response.reaction.clone())
            .collect()
    }

    pub fn build_message(&self) -> CreateMessage {
        let embed: CreateEmbed = CreateEmbed::new()
            .title(&self.title)
            .description(&self.description)
            .field("Options", self.options_string(), false);

        let message: CreateMessage = CreateMessage::new()
            .embed(embed)
            .reactions(self.reactions());

        message
    }
}
