use serenity::client::{Cache, Context};
use serenity::framework::standard::macros::{check, group};
use serenity::framework::standard::{Args, CommandOptions, Reason};

pub mod add;
pub mod get;
pub mod remove;

use add::*;
use get::*;
use remove::*;
use serenity::http::CacheHttp;
use serenity::model::channel::Message;
use serenity::model::guild::Role;
use serenity::model::id::{GuildId, RoleId, UserId};

use crate::models::GuildData;
use crate::{DbContainer, OwnersContainer};

#[group()]
#[commands(add, get, remove)]
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

        let has_perms =
            user_role_position_check(&ctx.cache, &guild_id, &msg.author.id, &role_id).await;
        if has_perms {
            return Ok(());
        }
    }

    return Err(Reason::User("Lacked required role or higher".to_string()));
}

async fn user_role_position_check(
    cache: &Cache,
    guild_id: &GuildId,
    user_id: &UserId,
    required_role_id: &RoleId,
) -> bool {
    let member = match cache.member(guild_id, user_id).await {
        Some(v) => v,
        None => return false,
    };

    let required_role = match required_role_id.to_role_cached(cache).await {
        Some(v) => v,
        None => return false,
    };

    let member_roles = match member.roles(cache).await {
        Some(v) => v,
        None => return false,
    };

    return member_roles
        .iter()
        .any(|role| role.position >= required_role.position);
}
