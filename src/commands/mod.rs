use serenity::framework::standard::macros::group;

pub mod command;
pub mod meta;
pub mod owner;

use meta::*;
use owner::*;

#[group]
#[commands(ping, quit, set_moderator_role, set_prefix)]
pub struct General;
