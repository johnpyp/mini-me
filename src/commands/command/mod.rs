use serenity::client::Context;
use serenity::framework::standard::macros::{check, group};
use serenity::framework::standard::{Args, CommandOptions, Reason};

pub mod add;
pub mod get;
pub mod remove;
pub mod rename;
pub mod update;

use add::*;
use get::*;
use remove::*;
use rename::*;
use serenity::model::channel::Message;
use serenity::model::id::{GuildId, RoleId, UserId};
use update::*;

use crate::models::GuildData;
use crate::{DbContainer, OwnersContainer};

#[group()]
#[commands(add, get, remove, update, rename)]
#[prefix = "command"]
pub struct Command;

#[check]
async fn command_moderator(
    ctx: &Context,
    msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> Result<(), Reason> {
    let data = ctx.data.read().await;
    let conn = data.get::<DbContainer>().unwrap();
    let owners = data.get::<OwnersContainer>().unwrap();
    {
        let owners = owners.read().await;
        if owners.contains(&msg.author.id) {
            return Ok(());
        }
    }

    let guild_id = msg.guild_id.ok_or(Reason::Unknown)?;

    let guild_data = GuildData::get(conn, &guild_id.to_string())
        .await
        .map_err(|_| Reason::Unknown)?
        .ok_or(Reason::Unknown)?;

    if let Some(moderator_role_id) = guild_data.moderator_role_id {
        let num_role_id: u64 = moderator_role_id.parse().map_err(|_| Reason::Unknown)?;
        let role_id: RoleId = num_role_id.into();

        let has_perms = user_role_position_check(ctx, &guild_id, &msg.author.id, &role_id).await;
        if has_perms.is_some() {
            return Ok(());
        }
    }

    return Err(Reason::User("Lacked required role or higher".to_string()));
}

async fn user_role_position_check(
    ctx: &Context,
    guild_id: &GuildId,
    user_id: &UserId,
    required_role_id: &RoleId,
) -> Option<()> {
    let guild = guild_id.to_guild_cached(&ctx.cache)?;
    let member = guild.member(&ctx.http, user_id).await.ok()?;

    let required_role = required_role_id.to_role_cached(&ctx.cache)?;

    let member_roles = member.roles(&ctx.cache)?;

    let has_perms = member_roles
        .iter()
        .any(|role| role.position >= required_role.position);
    match has_perms {
        true => Some(()),
        false => None,
    }
}
