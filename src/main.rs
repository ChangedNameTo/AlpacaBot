extern crate dotenv;

use dotenv::dotenv;
use serenity::all::{ReactionType, Ready};
use serenity::builder::{CreateEmbed, CreateMessage};
use std::env;

use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, Configuration, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[group]
#[commands(ping)]
struct General;

struct Handler;

struct PollResponse {
    text: String,
    reaction: ReactionType,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!hello" {
            let responses: Vec<PollResponse> = vec![
                PollResponse {
                    text: "Yes".to_string(),
                    reaction: ReactionType::Unicode("👍".to_string()),
                },
                PollResponse {
                    text: "No".to_string(),
                    reaction: ReactionType::Unicode("👎".to_string()),
                },
                PollResponse {
                    text: "I'll be late".to_string(),
                    reaction: ReactionType::Unicode("❓".to_string()),
                },
                PollResponse {
                    text: "Maybe".to_string(),
                    reaction: ReactionType::Unicode("⏳".to_string()),
                },
            ];

            let embed: CreateEmbed = CreateEmbed::new()
                .title("Weekly Pickleball")
                .description("Westside Paper @ 7pm. You in?")
                .field(
                    "Options",
                    ":thumbsup: Yes!\n:thumbsdown: No!\n:hourglass_flowing_sand: I'll be late\n:question: Maybe!\n",
                    false,
                );
            let builder: CreateMessage = CreateMessage::new().embed(embed).reactions(vec![
                ReactionType::Unicode("👍".to_string()),
                ReactionType::Unicode("👎".to_string()),
                ReactionType::Unicode("⏳".to_string()),
                ReactionType::Unicode("❓".to_string()),
            ]);
            let msg: Result<Message, SerenityError> =
                msg.channel_id.send_message(&ctx.http, builder).await;

            if let Err(why) = msg {
                println!("Error sending message: {why:?}");
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework: StandardFramework = StandardFramework::new().group(&GENERAL_GROUP);
    framework.configure(Configuration::new().prefix("~")); // set the bot's prefix to "~"

    // Login with a bot token from the environment
    let token: String = env::var("DISCORD_TOKEN").expect("token");
    let intents: GatewayIntents =
        GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client: Client = Client::builder(token, intents)
        .event_handler(Handler)
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

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
