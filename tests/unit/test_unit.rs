// tests/unit/test_unit.rs

// Import necessary modules and symbols
use serenity::model::channel::Message;
use serenity::prelude::*;

// Import the function to be tested
use super::ping;

#[tokio::test]
async fn test_ping() {
    // Create a mock Context
    let ctx = Context::new_test(Config::default().clone());

    // Create a mock Message
    let msg = Message {
        id: MessageId(0),
        channel_id: ChannelId(0),
        guild_id: None,
        author: User::default(),
        content: String::from("!ping"),
        timestamp: None,
        edited_timestamp: None,
        tts: false,
        mention_everyone: false,
        mentions: vec![],
        mention_roles: vec![],
        attachments: vec![],
        embeds: vec![],
        reactions: vec![],
        pinned: false,
        webhook_id: None,
        activity: None,
        application: None,
    };

    // Call the function being tested
    let result = ping(&ctx, &msg).await;

    // Assert that the result is Ok
    assert!(result.is_ok());
}
