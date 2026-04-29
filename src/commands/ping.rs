use serenity::model::channel::Message;
use serenity::prelude::*;

pub async fn run(ctx: &Context, msg: &Message) {
    // Sending a message can fail, due to a network error, an authentication error, or lack
    // of permissions to post in the channel, so log to stdout when some error happens,
    // with a description of it.
    if let Err(e) = msg.channel_id.say(&ctx.http, "Pong!").await {
        eprintln!("Error sending message: {:?}", e);
    }
}