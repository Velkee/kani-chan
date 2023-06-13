use std::time::Duration;

use chrono::prelude::*;
use serenity::builder::CreateSelectMenuOption;
use serenity::framework::standard::CommandResult;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::component::ButtonStyle;
use serenity::prelude::*;
use serenity::{framework::standard::macros::command, model::prelude::*};

use crate::database::events::get_events;

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

    let message = msg
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

    let interaction = match message
        .await_component_interaction(ctx)
        .timeout(Duration::from_secs(60 * 3))
        .await
    {
        Some(x) => x,
        None => {
            message.reply(&ctx, "Timed out").await?;
            message.delete(&ctx).await?;
            return Ok(());
        }
    };

    let event_option = &interaction.data.custom_id;

    if event_option == "edit" {
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
    };

    Ok(())
}

fn get_options() -> Vec<CreateSelectMenuOption> {
    let mut options: Vec<CreateSelectMenuOption> = vec![];

    for event in get_events() {
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
