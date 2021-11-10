use serenity::framework::standard::macros::group;

pub mod command;
pub mod meta;
pub mod owner;
pub mod schizo;

use meta::*;
use owner::*;
use schizo::*;

#[group]
#[commands(ping, quit, schizo, set_moderator_role, set_prefix)]
pub struct General;
