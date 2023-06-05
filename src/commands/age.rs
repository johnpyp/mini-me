use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

#[command]
#[only_in(guild)]
#[description = "Age of users"]
#[min_args(1)]
pub async fn age(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut user_ids = args
        .iter::<UserId>()
        .filter_map(|x| x.ok())
        .collect::<Vec<UserId>>();

    if user_ids.is_empty() || user_ids.len() > 50 {
        return Ok(());
    }

    user_ids.sort_by_key(|a| a.created_at());

    let mut response = MessageBuilder::new();

    response.push_line("User ages:");
    for user_id in user_ids {
        let unix_created_at = &user_id.created_at().unix_timestamp();
        let timestamp_format = format!("<t:{}:f>", unix_created_at);
        let user_id_mention = &user_id.mention();
        let user_age = format!(
            "{user} created at {date}",
            user = user_id_mention,
            date = timestamp_format
        );
        response.push_line(user_age);
    }

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.content(response).allowed_mentions(|am| am.empty_parse());
            m
        })
        .await?;

    Ok(())
}

#[command]
#[only_in(guild)]
#[description = "Oldest user in server"]
#[min_args(1)]
#[required_permissions(MANAGE_GUILD)]
#[owner_privilege]
pub async fn oldest(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let count = args.single::<u32>()?;

    let users = msg
        .guild(&ctx.cache)
        .unwrap()
        .members(&ctx.http, Some(200), None)
        .await?;

    let mut user_ids = users
        .into_iter()
        .map(|x| x.user.id)
        .collect::<Vec<UserId>>();

    if user_ids.is_empty() || user_ids.len() > 50 {
        return Ok(());
    }

    user_ids.sort_by_key(|a| a.created_at());

    let oldest_count_users: Vec<UserId> = user_ids.into_iter().take(count as usize).collect();

    let mut response = MessageBuilder::new();

    response.push_line("User ages:");
    for user_id in oldest_count_users {
        let unix_created_at = &user_id.created_at().unix_timestamp();
        let timestamp_format = format!("<t:{}:f>", unix_created_at);
        let user_id_mention = &user_id.mention();
        let user_age = format!(
            "{user} created at {date}",
            user = user_id_mention,
            date = timestamp_format
        );
        response.push_line(user_age);
    }

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.content(response).allowed_mentions(|am| am.empty_parse());
            m
        })
        .await?;

    Ok(())
}
