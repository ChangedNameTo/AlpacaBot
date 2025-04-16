use serenity::all::ReactionType;

pub(crate) struct PollResponse {
    pub text: String,
    pub reaction: ReactionType,
}

impl PollResponse {
    pub fn yes_response() -> Self {
        PollResponse {
            text: "Yes!".to_string(),
            reaction: ReactionType::Unicode("👍".to_string()),
        }
    }

    pub fn no_response() -> Self {
        PollResponse {
            text: "No".to_string(),
            reaction: ReactionType::Unicode("👎".to_string()),
        }
    }
    
    pub fn bought_response() -> Self {
        PollResponse {
            text: "Bought Ticket".to_string(),
            reaction: ReactionType::Unicode("🎟️".to_string()),
        }
    }

    pub fn maybe_response() -> Self {
        PollResponse {
            text: "Maybe".to_string(),
            reaction: ReactionType::Unicode("❓".to_string()),
        }
    }

    pub fn late_response() -> Self {
        PollResponse {
            text: "I'll be late".to_string(),
            reaction: ReactionType::Unicode("⌛".to_string()),
        }
    }

    pub fn default_responses() -> Vec<PollResponse> {
        vec![
            PollResponse::yes_response(),
            PollResponse::no_response(),
            PollResponse::maybe_response(),
            PollResponse::late_response(),
        ]
    }

    pub fn bought_responses() -> Vec<PollResponse> {
        vec![
            PollResponse::bought_response(),
            PollResponse::no_response(),
            PollResponse::maybe_response(),
        ]
    }
}
