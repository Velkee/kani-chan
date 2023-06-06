use std::time::Duration;

use chrono::prelude::*;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::component::ButtonStyle;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn event(ctx: &Context, msg: &Message) -> CommandResult {
    let hour = Local::now().hour();

    let greeting = if hour >= 17 {
        "Good evening!"
    } else if hour >= 12 {
        "Good afternoon!"
    } else {
        "Good morning!"
    };

    let greet = format!("{greeting} I'll be helping you organize your event today. If you could please provide me with some info, and we can get right ahead.");

    let message = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.content(&greet).components(|c| {
                c.create_action_row(|row| {
                    row.create_button(|btn| {
                        btn.custom_id("create")
                            .label("Create an event")
                            .style(ButtonStyle::Success)
                    });
                    row.create_button(|btn| {
                        btn.custom_id("edit")
                            .label("Edit an event")
                            .style(ButtonStyle::Primary)
                    });
                    row.create_button(|btn| {
                        btn.custom_id("delete")
                            .label("Delete an event")
                            .style(ButtonStyle::Danger)
                    })
                })
            })
        })
        .await?;

    msg.channel_id
        .send_message(&ctx, |m| m.content(response))
        .await?;

    Ok(())
}
