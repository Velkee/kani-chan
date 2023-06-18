mod commands;

use crate::commands::event::EVENT_COMMAND;
use crate::commands::hello::HELLO_COMMAND;

use std::env;

use serenity::{
    async_trait,
    framework::{standard::macros::*, StandardFramework},
    model::prelude::*,
    prelude::*,
};

use dotenv::dotenv;

#[group]
#[commands(event, hello)]
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
