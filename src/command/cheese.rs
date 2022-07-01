use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn cheese(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let full_text = args.raw().map(|s| s.to_string()).collect::<Vec<String>>().join(" ");
    let response = if full_text.contains("cheddar") {
        format!("{} is a cultured individual, cheddar is the best cheese", msg.author)
    } else {
        format!("WTF {}!!! What kind of cheese even is \"{}\"??? Is it cheese???", msg.author, full_text)
    };

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}
