#![allow(deprecated)]

extern crate dotenv;

use dotenv::dotenv;
use std::env;

use serenity::framework::standard::{Configuration, StandardFramework};
use serenity::prelude::*;

pub mod group_event;
pub mod poll_response;
pub mod scheduler;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework: StandardFramework = StandardFramework::new();
    framework.configure(Configuration::new().prefix("~")); // set the bot's prefix to "~"

    // Login with a bot token from the environment
    let token: String = env::var("DISCORD_TOKEN").expect("token");
    let intents: GatewayIntents =
        GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client: Client = Client::builder(token, intents)
        .event_handler(scheduler::Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.shutdown_all().await;
    });

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
