use clokwerk::Interval;
use clokwerk::{AsyncScheduler, Job, TimeUnits};
use core::time;
use serenity::all::{MessageReaction, ReactionType, User};
use serenity::{
    all::{ChannelId, Message},
    async_trait,
    builder::CreateMessage,
    prelude::{Context, EventHandler},
};
use std::time::Duration;

use crate::group_event::GroupEvent;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: serenity::model::gateway::Ready) {
        println!("{} is connected!", ready.user.name);

        // Initialize scheduler
        let mut scheduler: AsyncScheduler = AsyncScheduler::new();

        // Context Copies
        let pb_ctx: Context = ctx.clone();
        let trivia_ctx: Context = ctx.clone();

        // Duration
        let attendance_report_wait: Duration = time::Duration::from_secs(21600);

        // Pickleball
        scheduler.every(Interval::Wednesday).at("15:00").run(move || {
            let x: Context = pb_ctx.clone();
            async move {
                let message = send_message(x, GroupEvent::pickleball()).await;
                tokio::time::sleep(attendance_report_wait);
            }
        });

        // Trivia
        scheduler
            .every(Interval::Thursday)
            .at("15:00")
            .run(move || {
                let x: Context = trivia_ctx.clone();
                async move {
                    send_message(x, GroupEvent::trivia()).await;
                }
            });

        // Still alive
        scheduler
            .every(1.hours())
            .run(|| async { println!("Still living") });

        // Start scheduler when bot is ready
        tokio::spawn(async move {
            // Start the scheduler
            loop {
                scheduler.run_pending().await;
                tokio::time::sleep(Duration::from_secs(60)).await; // Check every minute
            }
        });
    }
}

async fn send_message(ctx: Context, event: GroupEvent) -> Message {
    // Get the channel ID where you want to send the message
    let channel_id: ChannelId = event.channel_id.into();

    let message: CreateMessage = event.build_message();

    let sent_message = channel_id
        .send_message(&ctx.http, message)
        .await
        .expect("Message failed to send!");

    sent_message
}

async fn send_attendance_report(ctx: Context, event: GroupEvent, prior_message: Message) {
    let yes_responses: Vec<User> = prior_message
        .reaction_users(
            ctx.http.clone(),
            ReactionType::Unicode("👍".to_string()),
            Some(100),
            None,
        )
        .await
        .expect("No reactions found!");

    let no_responses: Vec<User> = prior_message
        .reaction_users(
            ctx.http.clone(),
            ReactionType::Unicode("👎".to_string()),
            Some(100),
            None,
        )
        .await
        .expect("No reactions found!");

    let maybe_responses: Vec<User> = prior_message
        .reaction_users(
            ctx.http.clone(),
            ReactionType::Unicode("👎".to_string()),
            Some(100),
            None,
        )
        .await
        .expect("No reactions found!");

    let late_responses: Vec<User> = prior_message
        .reaction_users(
            ctx.http.clone(),
            ReactionType::Unicode("👎".to_string()),
            Some(100),
            None,
        )
        .await
        .expect("No reactions found!");

    // Get the channel ID where you want to send the message
    let channel_id: ChannelId = event.channel_id.into();

    let message: CreateMessage = event.build_message();

    channel_id
        .send_message(&ctx.http, message)
        .await
        .expect("Message failed to send!");
}
