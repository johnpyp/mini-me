use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use tracing::log::error;

use super::*;
use crate::models::DynamicCommand;
use crate::DbContainer;

#[command]
#[aliases("delete")]
#[only_in(guild)]
#[description = "Remove a command"]
#[num_args(1)]
#[checks(command_moderator)]
#[owner_privilege]
pub async fn remove(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let conn = data.get::<DbContainer>().unwrap();

    let req_command: String = args.single_quoted::<String>()?;

    let guild_id = match msg.guild_id {
        Some(v) => v,
        None => return Ok(()),
    }
    .to_string();

    let existing_command = DynamicCommand::get_command(conn, &guild_id, &req_command).await?;

    if let None = existing_command {
        msg.channel_id
            .say(
                &ctx.http,
                MessageBuilder::new()
                    .push_safe(req_command)
                    .push(" command doesn't exist"),
            )
            .await?;
        return Ok(());
    }
    if let Err(err) = DynamicCommand::delete_command(conn, &guild_id, &req_command).await {
        error!("Error when deleting command: {:?}", err);
        msg.channel_id
            .say(&ctx.http, "error removing command")
            .await?;
        return Ok(());
    }

    let response = MessageBuilder::new()
        .push(req_command)
        .push(" command deleted")
        .build();

    msg.channel_id.say(&ctx.http, &response).await?;

    Ok(())
}
