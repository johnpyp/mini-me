use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::models::GuildData;
use crate::DbContainer;

#[command]
#[bucket = "basic"]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command("set-moderator-role")]
#[required_permissions(MANAGE_GUILD)]
#[min_args(1)]
async fn set_moderator_role(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let req_role_id = args.single::<RoleId>()?;

    let data = ctx.data.read().await;
    let conn = data.get::<DbContainer>().unwrap();

    let guild_id = match msg.guild_id {
        Some(v) => v,
        None => return Ok(()),
    };

    let existing_guild: GuildData = GuildData::get(&conn, &guild_id.to_string())
        .await?
        .unwrap_or(GuildData::default());

    let guild_data = GuildData {
        guild_id: guild_id.to_string(),
        moderator_role_id: Some(req_role_id.to_string()),
        ..existing_guild
    };

    guild_data.upsert(conn).await?;

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.content(format_args!(
                "role {role} set as the minimum moderator role.",
                role = req_role_id.mention()
            ))
            .allowed_mentions(|am| am.empty_parse())
        })
        .await?;

    Ok(())
}
#[command("set-prefix")]
#[required_permissions(MANAGE_GUILD)]
#[min_args(1)]
async fn set_prefix(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let req_prefix = args.single::<String>()?;

    if req_prefix.len() < 1 {
        return Ok(());
    }
    if let Some(_) = serenity::utils::parse_mention(&req_prefix) {
        msg.channel_id
            .say(&ctx.http, "Can't use a mention as a prefix")
            .await?;
        return Ok(());
    }

    let data = ctx.data.read().await;
    let conn = data.get::<DbContainer>().unwrap();

    let guild_id = match msg.guild_id {
        Some(v) => v,
        None => return Ok(()),
    };

    let existing_guild: GuildData = GuildData::get(&conn, &guild_id.to_string())
        .await?
        .unwrap_or(GuildData::default());
    let guild_data = GuildData {
        guild_id: guild_id.to_string(),
        dynamic_prefix: Some(req_prefix.to_string()),
        ..existing_guild
    };

    guild_data.upsert(conn).await?;

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.content(format_args!(
                "prefix {prefix} set as the default prefix for this server.",
                prefix = req_prefix
            ))
            .allowed_mentions(|am| am.empty_parse())
        })
        .await?;

    Ok(())
}
