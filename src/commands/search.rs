use xivapi_rust::APIClient;

use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
async fn search(ctx: &Context, msg: &Message) -> CommandResult {
    let client = APIClient::new();

    let search_result = client.string_search(None, "string", None, 1).await.unwrap();

    msg.reply(ctx, format!("Got this response:\n{:#?}", search_result))
        .await
        .unwrap();

    Ok(())
}
