use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

use crate::models::DynamicCommand;
use crate::DbContainer;

#[command]
#[only_in(guild)]
#[description = "Get a command"]
#[num_args(1)]
#[bucket = "basic"]
pub async fn get(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let conn = data.get::<DbContainer>().unwrap();

    let req_command: String = args.single_quoted::<String>()?;

    let guild_id = match msg.guild_id {
        Some(v) => v,
        None => return Ok(()),
    }
    .to_string();

    let command = DynamicCommand::get_command(conn, &guild_id, &req_command).await?;

    if let Some(command) = command {
        let response = MessageBuilder::new().push(command.response).build();

        msg.channel_id.say(&ctx.http, &response).await?;
        return Ok(());
    }
    let response = MessageBuilder::new()
        .push("command '")
        .push(req_command)
        .push("' not found")
        .build();
    msg.channel_id.say(&ctx.http, &response).await?;

    Ok(())
}
