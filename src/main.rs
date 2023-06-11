use std::{env, time::Duration};

use chrono::prelude::*;
use serenity::{
    async_trait,
    framework::{
        standard::{macros::*, CommandResult},
        StandardFramework,
    },
    model::prelude::{
        application::interaction::InteractionResponseType, component::ButtonStyle, *,
    },
    prelude::*,
};

use dotenv::dotenv;

#[group]
#[commands(event)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name)
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("kc!"))
        .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_TOKEN").expect("Can't find token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occured while running the client: {:?}", why);
    }
}

#[command]
async fn event(ctx: &Context, msg: &Message) -> CommandResult {
    let hour = Local::now().hour();

    let (time_of_day, day_night) = if hour >= 17 {
        ("evening", "tonight")
    } else if hour >= 12 {
        ("afternoon", "today")
    } else {
        ("morning", "today")
    };

    let first_contact = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.content(format!(
                "Good {time_of_day}! I will be helping you manage your events {day_night}"
            ))
            .components(|c| {
                c.create_action_row(|r| {
                    r.create_button(|btn| {
                        btn.custom_id("create")
                            .label("Create")
                            .style(ButtonStyle::Success)
                    });
                    r.create_button(|btn| {
                        btn.custom_id("edit")
                            .label("Edit")
                            .style(ButtonStyle::Primary)
                    });
                    r.create_button(|btn| {
                        btn.custom_id("delete")
                            .label("Delete")
                            .style(ButtonStyle::Danger)
                    })
                })
            })
        })
        .await?;

    let interaction = match first_contact
        .await_component_interaction(ctx)
        .timeout(Duration::from_secs(60 * 3))
        .await
    {
        Some(x) => x,
        None => {
            first_contact.reply(&ctx, "Timed out").await?;
            return Ok(());
        }
    };

    let event_option = &interaction.data.custom_id;

    interaction
        .create_interaction_response(&ctx, |r| {
            r.kind(InteractionResponseType::UpdateMessage)
                .interaction_response_data(|d| {
                    d.ephemeral(true)
                        .content(format!(
                            "Understood, what event would you like to {}?",
                            event_option
                        ))
                        .components(|c| {
                            c.create_action_row(|r| {
                                r.create_button(|b| {
                                    b.custom_id("test")
                                        .label("Test")
                                        .style(ButtonStyle::Primary)
                                })
                            })
                        })
                })
        })
        .await?;

    Ok(())
}
