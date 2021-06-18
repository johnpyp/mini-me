use serenity::client::Context;
use serenity::framework::standard::macros::{check, group};
use serenity::framework::standard::{Args, CommandOptions, Reason};

pub mod add;
pub mod get;
pub mod remove;
pub mod rename;

use add::*;
use get::*;
use remove::*;
use rename::*;
use serenity::http::CacheHttp;
use serenity::model::channel::Message;
use serenity::model::id::RoleId;

use crate::models::GuildData;
use crate::DbContainer;

#[group()]
#[commands(add, get, remove, rename)]
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

    let guild_id = msg.guild_id.ok_or(Reason::Unknown)?;

    let guild_data = GuildData::get(conn, &guild_id.to_string())
        .await
        .map_err(|_| Reason::Unknown)?
        .ok_or(Reason::Unknown)?;

    if let Some(moderator_role_id) = guild_data.moderator_role_id {
        let num_role_id: u64 = moderator_role_id.parse().map_err(|_| Reason::Unknown)?;
        let role_id: RoleId = num_role_id.into();

        if msg
            .author
            .has_role(&ctx.http(), guild_id, role_id)
            .await
            .map_err(|_| Reason::Unknown)?
        {
            return Ok(());
        }
    }

    return Err(Reason::User("Lacked required role or higher".to_string()));
}
