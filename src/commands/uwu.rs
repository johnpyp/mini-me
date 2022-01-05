use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use uwuifier::uwuify_str_sse;

use crate::censor::get_custom_censor;

#[command]
#[only_in(guild)]
#[description = "UwU"]
#[min_args(1)]
pub async fn uwu(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let text = args.rest().to_string();

    if text.is_empty() {
        return Ok(());
    }

    let zero_width = '\u{200b}';
    let censor = get_custom_censor();
    let censored_text = censor.replace(&text, "!@#$%");

    let res_text = zero_width.to_string() + &uwuify_str_sse(&censored_text);

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.content(&res_text).allowed_mentions(|am| am.empty_parse())
        })
        .await?;

    return Ok(());
}
