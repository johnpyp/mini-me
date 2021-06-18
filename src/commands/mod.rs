use serenity::framework::standard::macros::group;

pub mod command;
pub mod math;
pub mod meta;
pub mod owner;

use math::*;
use meta::*;
use owner::*;

#[group]
#[commands(multiply, ping, quit, set_moderator_role)]
pub struct General;
