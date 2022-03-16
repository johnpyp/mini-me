use std::cmp;

use censor::Censor;
use rand::prelude::*;
use rand::Rng;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[only_in(guild)]
#[description = "zoo los schizo"]
#[min_args(1)]
pub async fn schizo(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let text = args.rest().to_string();

    if text.is_empty() {
        return Ok(());
    }

    let zero_width = '\u{200b}';
    let censor = Censor::Standard;
    let censored_text = censor.replace(&text, "!@#$%");
    let res_text = zero_width.to_string() + &do_schizo(&censored_text);

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.content(&res_text).allowed_mentions(|am| am.empty_parse())
        })
        .await?;

    return Ok(());
}

fn do_schizo(text: &str) -> String {
    let words = text.split(' ').collect::<Vec<&str>>();
    let mut index = 0;
    let num_words = words.len();
    let mut rng = rand::thread_rng();
    let mut string = String::new();
    while index < num_words {
        if rng.gen_range(0..4) != 0 {
            string.push_str(words[index]);
            string.push(' ');
            index += 1;
            continue;
        }
        let num_to_schizo = rng.gen_range(1..4);
        (index..cmp::min(index + num_to_schizo, num_words)).for_each(|i| {
            let num_dots = if rng.gen_range(0..5) == 0 {
                rng.gen_range(0..4)
            } else {
                0
            };
            let num_questions = if rng.gen_range(0..5) == 0 {
                rng.gen_range(0..4)
            } else {
                0
            };
            string.push_str(&format!(
                "{}{}{}",
                schizo_word(words[i], &mut rng),
                "?".repeat(num_questions),
                ".".repeat(num_dots)
            ));
            string.push(' ');
        });
        index += num_to_schizo;
    }
    string
}

fn schizo_word(word: &str, rng: &mut ThreadRng) -> String {
    let schizo_type = rng.gen_range(0..5);
    if schizo_type <= 3 {
        return word.to_uppercase();
    }
    return word
        .chars()
        .map(|c| {
            if rng.gen_range(0..2) == 0 {
                c.to_uppercase().collect()
            } else {
                String::from(c)
            }
        })
        .collect();
}
