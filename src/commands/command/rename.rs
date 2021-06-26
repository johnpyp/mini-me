use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

use super::*;
use crate::models::DynamicCommand;
use crate::DbContainer;

#[command]
#[only_in(guild)]
#[description = "Rename a command"]
#[min_args(2)]
#[owner_privilege]
#[checks(command_moderator)]
pub async fn rename(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let conn = data.get::<DbContainer>().unwrap();

    let req_command = args.single_quoted::<String>()?;
    let req_new_command = args.rest().to_string();

    let guild_id = match msg.guild_id {
        Some(v) => v,
        None => return Ok(()),
    }
    .to_string();

    let command = DynamicCommand::get_command(conn, &guild_id, &req_command).await?;

    if let Some(mut command) = command {
        command.rename(conn, &req_new_command).await?;

        let response = MessageBuilder::new()
            .push_safe(&req_command)
            .push(" renamed to ")
            .push_safe(&req_new_command)
            .build();

        msg.channel_id.say(&ctx.http, &response).await?;
        return Ok(());
    }
    let response = MessageBuilder::new()
        .push_safe(&req_command)
        .push(" command not found")
        .build();

    msg.channel_id.say(&ctx.http, &response).await?;
    return Ok(());
}
