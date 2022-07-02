use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use meval::eval_str;

#[command]
pub async fn calc(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let expr = args.raw().map(|s| s.to_string()).collect::<Vec<String>>().join(" ");

    let answer = eval_str(&expr);
    let response = if let Ok(ans) = answer {
        format!("{} = {}", expr, ans)
    } else {
        format!("Error parsing expression: {}", expr)
    };

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}
