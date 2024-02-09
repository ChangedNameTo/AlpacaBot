// tests/unit/test_group_event.rs

// Import necessary modules and symbols
use serenity::{
  builder::{CreateEmbed, CreateMessage},
  model::channel::ReactionType,
};

// Import the struct to be tested
use crate::group_event::GroupEvent;

#[test]
fn test_options_string() {
  // Create a GroupEvent instance with some options
  let event = GroupEvent {
      title: String::from("Test Event"),
      description: String::from("This is a test event"),
      channel_id: 1234567890,
      options: vec![
          PollResponse {
              text: String::from("Option 1"),
              reaction: ReactionType::Unicode(String::from("👍")),
          },
          PollResponse {
              text: String::from("Option 2"),
              reaction: ReactionType::Unicode(String::from("👎")),
          },
      ],
  };

  // Call the options_string method and check the result
  let options_string = event.options_string();
  assert_eq!(options_string, "👍 Option 1\n👎 Option 2\n");
}

#[test]
fn test_reactions() {
  // Create a GroupEvent instance with some options
  let event = GroupEvent {
      title: String::from("Test Event"),
      description: String::from("This is a test event"),
      channel_id: 1234567890,
      options: vec![
          PollResponse {
              text: String::from("Option 1"),
              reaction: ReactionType::Unicode(String::from("👍")),
          },
          PollResponse {
              text: String::from("Option 2"),
              reaction: ReactionType::Unicode(String::from("👎")),
          },
      ],
  };

  // Call the reactions method and check the result
  let reactions = event.reactions();
  assert_eq!(
      reactions,
      vec![
          ReactionType::Unicode(String::from("👍")),
          ReactionType::Unicode(String::from("👎")),
      ]
  );
}

// Add more test functions as needed to cover other methods of GroupEvent
