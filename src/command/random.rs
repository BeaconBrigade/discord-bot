use rand::seq::SliceRandom;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

const NO_OPTION: &str = "No options to choose from... What did you expect would happen??";

#[command]
pub async fn random(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let args = args.raw().collect::<Vec<&str>>();

    let response = args.choose(&mut rand::thread_rng())
        .unwrap_or(&NO_OPTION);

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}
