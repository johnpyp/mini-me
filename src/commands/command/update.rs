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
#[only_in(guild)]
#[description = "Update a command"]
#[min_args(2)]
#[owner_privilege]
#[checks(command_moderator)]
pub async fn update(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let conn = data.get::<DbContainer>().unwrap();

    let req_command = args.single_quoted::<String>()?;
    let req_response = args.rest().to_string();

    let guild_id = match msg.guild_id {
        Some(v) => v,
        None => return Ok(()),
    }
    .to_string();

    let attachment_urls = get_attachment_urls(msg);
    if attachment_urls.is_none() && req_response.is_empty() {
        error!("Attachment urls & req_response body empty, skipping");
        return Ok(());
    }
    let command = DynamicCommand::get_command(conn, &guild_id, &req_command).await?;

    if let Some(mut command) = command {
        command
            .update(conn, &req_response, &attachment_urls)
            .await?;

        let response = MessageBuilder::new()
            .push_safe(&req_command)
            .push(" command updated")
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
