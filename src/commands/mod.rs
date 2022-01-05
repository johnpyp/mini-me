use serenity::framework::standard::macros::group;

pub mod command;
pub mod meta;
pub mod owner;
pub mod schizo;
pub mod uwu;

use meta::*;
use owner::*;
use schizo::*;
use uwu::*;

#[group]
#[commands(ping, quit, schizo, uwu, set_moderator_role, set_prefix)]
pub struct General;
