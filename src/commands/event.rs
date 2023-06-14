use std::time::Duration;

use chrono::prelude::*;

use diesel::SqliteConnection;
use serenity::{
    builder::{CreateComponents, CreateSelectMenuOption},
    framework::standard::{macros::command, CommandResult},
    model::{
        application::interaction::InteractionResponseType,
        prelude::{component::ButtonStyle, *},
    },
    prelude::*,
};

use kani_chan::{establish_connection, get_events};

async fn handle_timeout(ctx: &Context, message: Message) {
    message.reply(&ctx, "Timed out").await.unwrap();
    message.delete(&ctx).await.unwrap();
}

fn get_options(connection: &mut SqliteConnection) -> Vec<CreateSelectMenuOption> {
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

#[command]
async fn event(ctx: &Context, msg: &Message) -> CommandResult {
    let connection = &mut establish_connection();

    let hours = Local::now().hour();

    let (time_of_day, day_night) = if hours >= 17 {
        ("evening", "tonight")
    } else if hours >= 12 {
        ("afternoon", "today")
    } else {
        ("morning", "today")
    };

    let message = msg
        .channel_id
        .send_message(&ctx, |message| {
            message
                .content(format!(
                    "Good {}! I will be helping you manage your events {}",
                    time_of_day, day_night
                ))
                .components(|components| {
                    components.create_action_row(|row| {
                        row.create_button(|btn| {
                            btn.custom_id("create")
                                .label("Create")
                                .style(ButtonStyle::Success)
                        });
                        row.create_button(|btn| {
                            btn.custom_id("edit")
                                .label("Edit")
                                .style(ButtonStyle::Primary)
                        });
                        row.create_button(|btn| {
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
                .create_interaction_response(&ctx, |response| {
                    response
                        .kind(InteractionResponseType::UpdateMessage)
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
                .send_message(&ctx, |message| {
                    message.content(format!("Is the title {} correct?", title))
                })
                .await?;

            let interaction = if let Some(interaction) = message
                .await_component_interaction(ctx)
                .timeout(Duration::from_secs(60 * 3))
                .await
            {
                interaction
            } else {
                handle_timeout(ctx, message).await;
                return Ok(());
            };

            let confirmation = &interaction.data.custom_id;

            Ok(())
        }
        "edit" => {
            let options = get_options(connection);
            interaction
                .create_interaction_response(&ctx, |response| {
                    response
                        .kind(InteractionResponseType::UpdateMessage)
                        .interaction_response_data(|d| {
                            d.content("Of course! Which event would you like to edit?")
                                .components(|components| {
                                    components.create_action_row(|row| {
                                        row.create_select_menu(|message| {
                                            message
                                                .custom_id("event select")
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
