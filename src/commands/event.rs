use std::time::Duration;

use chrono::prelude::*;
use serenity::framework::standard::CommandResult;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::component::ButtonStyle;
use serenity::prelude::*;
use serenity::{framework::standard::macros::command, model::prelude::*};

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
