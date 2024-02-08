use clokwerk::{AsyncScheduler, Interval, Job};
use serenity::{
    all::ChannelId,
    async_trait,
    prelude::{Context, EventHandler},
};
use std::time::Duration;

use crate::group_event::GroupEvent;

const CHANNEL_ID: u64 = 812142236295888910;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: serenity::model::gateway::Ready) {
        println!("{} is connected!", ready.user.name);

        // Initialize scheduler
        let mut scheduler: AsyncScheduler = AsyncScheduler::new();

        // Pickleball
        scheduler
            .every(Interval::Monday)
            .at("12:55")
            .run(send_message(ctx, GroupEvent::pickleball()));

        // Start scheduler when bot is ready
        tokio::spawn(async move {
            // Start the scheduler
            loop {
                println!("Flushing scheduler");
                scheduler.run_pending();
                tokio::time::sleep(Duration::from_secs(60)).await; // Check every minute
            }
        });
    }
}

async fn send_message(ctx: Context, event: GroupEvent) {
    // Get the channel ID where you want to send the message
    let channel_id: ChannelId = CHANNEL_ID.into();

    let message = event.build_message(); // Assuming GroupEvent has a method build_message()

    if let Err(why) = channel_id.send_message(&ctx.http, message).await {
        println!("Error sending message: {:?}", why);
    }
}
