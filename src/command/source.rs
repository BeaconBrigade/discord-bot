use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::utils::MessageBuilder;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn source(ctx: &Context, msg: &Message) -> CommandResult {
    let my_id: UserId = 756201184146096189.into();
    let start = "
Authour: 
- Discord: ";

    let end = "
- Gmail:   beaconbrigade@gmail.com
- Github:  https://github.com/BeaconBrigade 
Source:
- Github:  https://github.com/BeaconBrigade/discord-bot";

    let resp = MessageBuilder::new()
        .push(start)
        .user(my_id)
        .push(end)
        .build();

    msg.channel_id.say(&ctx.http, resp).await?;

    Ok(())
}
