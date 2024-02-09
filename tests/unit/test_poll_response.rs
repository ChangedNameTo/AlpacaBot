// tests/unit/test_poll_response.rs

// Import necessary modules and symbols
use serenity::model::channel::ReactionType;

// Import the struct to be tested
use crate::poll_response::PollResponse;

#[test]
fn test_yes_response() {
    // Call the yes_response method and check the result
    let response = PollResponse::yes_response();
    assert_eq!(response.text, "Yes!");
    assert_eq!(response.reaction, ReactionType::Unicode("👍".to_string()));
}

#[test]
fn test_no_response() {
    // Call the no_response method and check the result
    let response = PollResponse::no_response();
    assert_eq!(response.text, "No");
    assert_eq!(response.reaction, ReactionType::Unicode("👎".to_string()));
}

#[test]
fn test_maybe_response() {
    // Call the maybe_response method and check the result
    let response = PollResponse::maybe_response();
    assert_eq!(response.text, "Maybe");
    assert_eq!(response.reaction, ReactionType::Unicode("❓".to_string()));
}

#[test]
fn test_late_response() {
    // Call the late_response method and check the result
    let response = PollResponse::late_response();
    assert_eq!(response.text, "I'll be late");
    assert_eq!(response.reaction, ReactionType::Unicode("⌛".to_string()));
}

#[test]
fn test_default_responses() {
    // Call the default_responses method and check the result
    let responses = PollResponse::default_responses();
    assert_eq!(responses.len(), 4);

    // Check each response in the default list
    assert_eq!(responses[0].text, "Yes!");
    assert_eq!(responses[0].reaction, ReactionType::Unicode("👍".to_string()));

    assert_eq!(responses[1].text, "No");
    assert_eq!(responses[1].reaction, ReactionType::Unicode("👎".to_string()));

    assert_eq!(responses[2].text, "Maybe");
    assert_eq!(responses[2].reaction, ReactionType::Unicode("❓".to_string()));

    assert_eq!(responses[3].text, "I'll be late");
    assert_eq!(responses[3].reaction, ReactionType::Unicode("⌛".to_string()));
}
