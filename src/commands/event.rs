use std::{sync::Arc, time::Duration};

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

// Handles a message timeout with a message and deleting it
async fn handle_timeout(ctx: &Context, message: Message) {
    message.reply(&ctx, "Timed out").await.unwrap();
    message.delete(&ctx).await.unwrap();
}

// Retrieves the select menu options for events
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

// Quick function to create and return a message with interactable components
async fn send_interactable(
    ctx: &Context,
    msg: &Message,
    content: String,
    components: CreateComponents,
) -> Result<Message, serenity::Error> {
    msg.channel_id
        .send_message(ctx, |message| {
            message.content(content).set_components(components)
        })
        .await
}

// Asks for an event title
async fn ask_title(msg: &Message, ctx: &Context) -> Option<Arc<Message>> {
    msg.channel_id
        .await_reply(ctx)
        .timeout(Duration::from_secs(60 * 3))
        .await
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

    let mut components = CreateComponents::default();

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
    });

    let message = send_interactable(
        ctx,
        msg,
        format!(
            "Good {}! I will be helping you manage your events {}",
            time_of_day, day_night
        ),
        components,
    )
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

            if let Some(title) = ask_title(msg, ctx).await {
                let mut components = CreateComponents::default();

                components.create_action_row(|row| {
                    row.create_button(|btn| {
                        btn.custom_id("confirm")
                            .label("Yes")
                            .style(ButtonStyle::Success)
                    });
                    row.create_button(|btn| {
                        btn.custom_id("cancel")
                            .label("No")
                            .style(ButtonStyle::Danger)
                    });
                    row.create_button(|btn| {
                        btn.custom_id("quit")
                            .label("Cancel event creation")
                            .style(ButtonStyle::Danger)
                    })
                });

                send_interactable(
                    ctx,
                    msg,
                    format!("Is the title {} correct?", title.content),
                    components,
                )
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

                println!("{}", confirmation);
            } else {
                handle_timeout(ctx, message).await
            }

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
