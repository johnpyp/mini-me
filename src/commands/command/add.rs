use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use tracing::log::error;
use uuid::Uuid;

use super::*;
use crate::models::DynamicCommand;
use crate::DbContainer;

#[command]
#[only_in(guild)]
#[description = "Add a command"]
#[min_args(1)]
#[owner_privilege]
#[checks(command_moderator)]
pub async fn add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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

    let new_command = DynamicCommand {
        id: Uuid::new_v4().to_string(),
        command: req_command.clone(),
        attachment_urls,
        response: req_response,
        guild_id: guild_id.to_string(),
    };

    let existing_command = DynamicCommand::get_command(conn, &guild_id, &req_command).await?;

    if existing_command.is_some() {
        msg.channel_id
            .say(
                &ctx.http,
                MessageBuilder::new()
                    .push_safe(req_command)
                    .push(" command already exists"),
            )
            .await?;
        return Ok(());
    }

    if let Err(err) = new_command.add(conn).await {
        error!("Error when adding command: {:?}", err);
        msg.channel_id
            .say(&ctx.http, "error adding command")
            .await?;
        return Ok(());
    };

    let response = MessageBuilder::new()
        .push_safe(&req_command)
        .push(" command created")
        .build();

    msg.channel_id.say(&ctx.http, &response).await?;

    Ok(())
}

pub fn get_attachment_urls(msg: &Message) -> Option<Vec<String>> {
    let raw_attachment_urls: Vec<String> = msg
        .attachments
        .iter()
        .map(|attachment| attachment.proxy_url.clone())
        .collect();

    if raw_attachment_urls.is_empty() {
        None
    } else {
        Some(raw_attachment_urls)
    }
}
