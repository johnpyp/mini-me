use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use tracing::info;
use uuid::Uuid;

use crate::models::DynamicCommand;
use crate::{DbContainer, ShardManagerContainer};

#[command]
#[owners_only]
async fn quit(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        msg.reply(ctx, "Shutting down!").await?;
        manager.lock().await.shutdown_all().await;
    } else {
        msg.reply(ctx, "There was a problem getting the shard manager")
            .await?;

        return Ok(());
    }

    Ok(())
}

#[command("copy-commands-from")]
#[owners_only]
#[only_in(guild)]
#[description = "Copy commands from a guild"]
#[min_args(1)]
async fn copy_commands_from(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    info!("Got command");
    let data = ctx.data.read().await;
    let conn = data.get::<DbContainer>().unwrap();

    let current_guild_id = match msg.guild_id {
        Some(v) => v,
        None => return Ok(()),
    };

    let req_origin_guild_id = args.single_quoted::<String>()?;

    let commands_in_origin =
        DynamicCommand::get_commands_by_guild(conn, &req_origin_guild_id).await?;

    let mut added_commands: Vec<String> = Vec::new();
    let mut not_added_commands: Vec<String> = Vec::new();

    for dyn_command in commands_in_origin {
        let current_guild_id_str = current_guild_id.to_string();
        let command_name = dyn_command.command.to_string();
        let existing_command =
            DynamicCommand::get_command(conn, &current_guild_id_str, &command_name).await?;
        if existing_command.is_none() {
            let new_command = DynamicCommand {
                id: Uuid::new_v4().to_string(),
                command: command_name.clone(),
                guild_id: current_guild_id_str,
                response: dyn_command.response,
                attachment_urls: dyn_command.attachment_urls,
            };

            new_command.add(conn).await?;
            added_commands.push(command_name.clone());
        } else {
            not_added_commands.push(command_name.clone());
        }
    }

    let mut builder = MessageBuilder::new();

    builder.push_line("**Added commands:**");
    for cmd_name in added_commands {
        builder.push_line_safe(format!("- {}", cmd_name));
    }

    builder.push_line("\n**Already existing commands (not added):**");
    for cmd_name in not_added_commands {
        builder.push_line_safe(format!("- {}", cmd_name));
    }

    let response = builder.build();

    msg.channel_id.say(&ctx.http, &response).await?;

    Ok(())
}
