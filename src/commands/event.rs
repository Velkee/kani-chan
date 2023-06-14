use std::sync::Arc;
use std::time::Duration;

use chrono::prelude::*;
use serenity::builder::{CreateComponents, CreateSelectMenuOption};
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::component::ButtonStyle;
use serenity::model::prelude::interaction::message_component::MessageComponentInteraction;
use serenity::model::prelude::*;
use serenity::prelude::*;

use kani_chan::{establish_connection, get_events};

#[command]
async fn event(ctx: &Context, msg: &Message) -> CommandResult {
    fn get_options() -> Vec<CreateSelectMenuOption> {
        let connection = &mut establish_connection();
        let mut options: Vec<CreateSelectMenuOption> = Vec::new();

        for event in get_events(connection) {
            let mut option = CreateSelectMenuOption::default();
            option.label(event.title).value(event.id);
            match event.description {
                Some(description) => option.description(description),
                None => &mut option,
            };

            options.push(option);
        }

        options
    }

    let hour = Local::now().hour();

    let (time_of_day, day_night) = if hour >= 17 {
        ("evening", "tonight")
    } else if hour >= 12 {
        ("afternoon", "today")
    } else {
        ("morning", "today")
    };

    let message = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.content(format!(
                "Good {}! I will be helping you manage your events {}",
                time_of_day, day_night
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

    let interaction = if let Some(interaction) = message
        .await_component_interaction(ctx)
        .timeout(Duration::from_secs(60 * 3))
        .await
    {
        interaction
    } else {
        message.reply(&ctx, "Timed out").await?;
        message.delete(&ctx).await?;
        return Ok(());
    };

    let event_option = &interaction.data.custom_id;

    match event_option.as_str() {
        "create" => {
            interaction
                .create_interaction_response(&ctx, |r| {
                    r.kind(InteractionResponseType::UpdateMessage)
                        .interaction_response_data(|d| {
                            d.content("Understood! What would you like to call the event?")
                                .set_components(CreateComponents::default())
                        })
                })
                .await?;

            let title = if let Some(event_title) = msg
                .channel_id
                .await_reply(ctx)
                .timeout(Duration::from_secs(60 * 3))
                .await
            {
                event_title.content.to_owned()
            } else {
                message.reply(&ctx, "Timed out").await?;
                message.delete(&ctx).await?;
                return Ok(());
            };

            let message = msg
                .channel_id
                .send_message(&ctx, |m| {
                    m.content(format!("Is the title {} correct?", title))
                })
                .await?;

            let interaction = if let Some(interaction) = message
                .await_component_interaction(ctx)
                .timeout(Duration::from_secs(60 * 3))
                .await
            {
                interaction
            } else {
                message.reply(&ctx, "Timed out").await?;
                message.delete(&ctx).await?;
                return Ok(());
            };

            let confirmation = &interaction.data.custom_id;

            Ok(())
        }
        "edit" => {
            let options = get_options();
            interaction
                .create_interaction_response(&ctx, |r| {
                    r.kind(InteractionResponseType::UpdateMessage)
                        .interaction_response_data(|d| {
                            d.content("Of course! Which event would you like to edit?")
                                .components(|c| {
                                    c.create_action_row(|r| {
                                        r.create_select_menu(|m| {
                                            m.custom_id("event select")
                                                .options(|o| o.set_options(options))
                                        })
                                    })
                                })
                        })
                })
                .await?;

            Ok(())
        }
        _ => {
            message
                .reply(
                    &ctx,
                    "Invalid interaction, please contact your local Rust developer",
                )
                .await?;
            message.delete(&ctx).await?;

            Ok(())
        }
    }
}
