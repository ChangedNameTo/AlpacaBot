use clokwerk::Interval;
use clokwerk::{AsyncScheduler, Job, TimeUnits};
use serenity::{
    all::ChannelId,
    async_trait,
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

        // Pickleball
        scheduler.every(Interval::Monday).at("10:00").run(move || {
            let x: Context = pb_ctx.clone();
            async move {
                send_message(x, GroupEvent::pickleball()).await;
            }
        });

        // Trivia
        scheduler
            .every(Interval::Thursday)
            .at("10:00")
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

async fn send_message(ctx: Context, event: GroupEvent) {
    // Get the channel ID where you want to send the message
    let channel_id: ChannelId = event.channel_id.into();

    let message: serenity::builder::CreateMessage = event.build_message();

    channel_id
        .send_message(&ctx.http, message)
        .await
        .expect("Message failed to send!");
}
