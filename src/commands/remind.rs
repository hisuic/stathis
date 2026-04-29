use serenity::all::MessageBuilder;
use serenity::futures::channel;
use serenity::model::channel::Message;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::collector::ComponentInteractionCollector;

use std::time::Duration;

use serenity::builder::{
    CreateButton,
    CreateInteractionResponse,
    CreateInteractionResponseMessage,
    CreateMessage,
    CreateSelectMenu,
    CreateSelectMenuKind,
    CreateSelectMenuOption,
};

pub async fn run(ctx: &Context, msg: &Message) {
    let channel = match msg.channel_id.to_channel(&ctx).await {
        Ok(channel) => channel,
        Err(why) => {
            println!("Error getting channel: {why:?}");

            return;
        },
    };

    let show_buttons = msg
        .channel_id
        .send_message(
            &ctx,
            CreateMessage::new()
                .content(format!("When should I remind you? \n\n"))
                .button(CreateButton::new("remind_30_min").label("30 mins"))
                .button(CreateButton::new("remind_1_h").label("1 hour"))
                .button(CreateButton::new("remind_3_h").label("3 hours"))
                .button(CreateButton::new("remind_5_h").label("5 hours"))
                .button(CreateButton::new("remind_1_d").label("1 day"))
                .button(CreateButton::new("remind_3_d").label("3 days"))
                .button(CreateButton::new("remind_1_w").label("1 week"))
                .button(CreateButton::new("remind_2_w").label("2 weeks"))
                .button(CreateButton::new("remind_1_m").label("1 month")),
        )
        .await
        .unwrap();

    let interaction = match ComponentInteractionCollector::new(&ctx.shard)
        .message_id(show_buttons.id)
        .timeout(Duration::from_secs(60 * 3))
        .await 
        {
            Some(i) => i,
            None => {
                show_buttons.reply(&ctx, "Timed out").await.unwrap();
                return;
            }
        };

        let delay = match interaction.data.custom_id.as_str() {
            "remind_30_min" => Duration::from_secs(60 * 30),
            "remind_1_h" => Duration::from_secs(60 * 60),
            "remind_3_h" => Duration::from_secs(60 * 60 * 3),
            "remind_5_h" => Duration::from_secs(60 * 60 * 5),
            "remind_1_d" => Duration::from_secs(60 * 60 * 24),
            "remind_3_d" => Duration::from_secs(60 * 60 * 24 * 3),
            "remind_1_w" => Duration::from_secs(60 * 60 * 24 * 7),
            "remind_2_w" => Duration::from_secs(60 * 60 * 24 * 7 * 2),
            "remind_1_m" => Duration::from_secs(60 * 60 * 24 * 30),
            _ => {
                println!("Unexpected custom_id: {}", interaction.data.custom_id);
                show_buttons.reply(&ctx.http, "Unknown button").await.unwrap();
                return;
            }
        };
}