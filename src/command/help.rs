use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

const HELP_MESSAGE: &str = 
"Help:
`~random [THINGS-TO-CHOOSE-FROM]`   Choose one of the words after `random`
`~hello`                            Complete this sentence
`~ping`                             PONG!!
`~cheese [TYPE-OF-CHEESE]`          Is this a good cheese?
`~source`                           About the authour and program
`~help`                             You need some help?";

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, HELP_MESSAGE).await?;

    Ok(())
}
