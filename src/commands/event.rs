use chrono::prelude::*;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::prelude::*;
use serenity::model::prelude::*;

#[command]
pub async fn event(ctx: &Context, msg: &Message, ) -> CommandResult {
    let hour = Local::now().hour();

    let greeting = if hour >= 17 {
        "Good evening!"
    } else if hour >= 12 {
        "Good afternoon!"
    } else {
        "Good mornig!"
    };

    let message = format!("{greeting} I'll be helping you organize your event today. If you could please provide me with some info, and we can get right ahead.");

    msg.reply(ctx, message).await?;

    Ok(())
}