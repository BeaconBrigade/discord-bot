use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use std::fs;
use rand::{thread_rng, seq::IteratorRandom};

#[command]
pub async fn meme(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let files = fs::read_dir("./memes").expect("No meme directory found");
    let meme = files
        .choose(&mut thread_rng())
        .expect("No memes found??? uncultured...").unwrap()
        .file_name();

    let random_meme = meme.to_str().expect("Error converting file name to string");
    let full = "./memes/".to_owned() + random_meme;

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.content("Meme!")
                .embed(|e| {
                    e.title("Meme!")
                        .description("A funny random meme!")
                        .image("attachment://".to_owned() + random_meme)
        }).add_file(&*full)
        }).await.unwrap();

    Ok(())
}
