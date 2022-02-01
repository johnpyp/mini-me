use serenity::framework::standard::macros::group;

pub mod command;
pub mod meta;
pub mod owner;
pub mod schizo;
pub mod uwu;
pub mod wordle;

use meta::*;
use owner::*;
use schizo::*;
use uwu::*;
use wordle::*;

#[group]
#[commands(ping, quit, schizo, uwu, wordle, set_moderator_role, set_prefix)]
pub struct General;
