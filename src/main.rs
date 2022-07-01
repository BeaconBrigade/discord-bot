use std::env;

use serenity::async_trait;
use serenity::framework::StandardFramework;
use serenity::framework::standard::macros::group;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

mod command;
use command::{ping::*, hello::*, cheese::*};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[group]
#[commands(ping, hello, cheese)]
struct General;

#[tokio::main]
async fn main() {
   
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")).group(&GENERAL_GROUP);

    let mut client =
        Client::builder(&token, intents)
            .framework(framework)
            .event_handler(Handler)
            .await
            .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
