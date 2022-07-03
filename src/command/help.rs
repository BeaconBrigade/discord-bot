use serenity::framework::standard::macros::help;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

const HELP_MESSAGE: &str = 
"Help:
`~random [THINGS-TO-CHOOSE-FROM]`   Choose one of the words after `random`
`~hello`                            Complete this sentence
`~ping`                             PONG!!
`~cheese [TYPE-OF-CHEESE]`          Is this a good cheese?
`~calc   [MATH_EXPRESSION]`         Evaluate a math expression
`~meme`                             Get a random meme
`~source`                           About the authour and program";

#[help]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, HELP_MESSAGE).await?;

    Ok(())
}
