mod commands;

use std::collections::{HashMap, HashSet};
use std::env;
use std::sync::Arc;
use std::time::Duration;

use commands::command::*;
use commands::*;
use db::DbConn;
use models::DynamicCommand;
use rand::prelude::*;
use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::macros::hook;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::channel::{AttachmentType, Message};
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::model::id::UserId;
use serenity::prelude::*;
use tokio::time::Instant;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use crate::models::GuildData;

pub mod db;
pub mod models;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct DbContainer;

impl TypeMapKey for DbContainer {
    type Value = DbConn;
}

pub struct DynCommandRateLimit;

impl TypeMapKey for DynCommandRateLimit {
    type Value = Arc<RwLock<HashMap<String, Instant>>>;
}

pub struct OwnersContainer;

impl TypeMapKey for OwnersContainer {
    type Value = Arc<RwLock<HashSet<UserId>>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[hook]
async fn unrecognised_command_hook(ctx: &Context, msg: &Message, unrecognised_command_name: &str) {
    let min_channel_limit = Duration::from_secs(3);

    let data = ctx.data.read().await;

    let conn = data.get::<DbContainer>().unwrap();

    let guild_id = match msg.guild_id {
        Some(v) => v,
        None => return,
    }
    .to_string();

    let command = DynamicCommand::get_command(conn, &guild_id, unrecognised_command_name)
        .await
        .unwrap_or(None);

    if let Some(command) = command {
        let rw_rate_limit = data.get::<DynCommandRateLimit>().unwrap();

        let channel_id = msg.channel_id;
        let now = Instant::now();
        {
            let rate_limit = rw_rate_limit.read().await;
            if let Some(prev_time) = rate_limit.get(&channel_id.to_string()) {
                if let Some(distance) = now.checked_duration_since(*prev_time) {
                    if distance.lt(&min_channel_limit) {
                        return;
                    }
                }
            }
        }
        {
            let mut rate_limit = rw_rate_limit.write().await;
            rate_limit.insert(channel_id.to_string(), now);
        }

        let split_on_or: Vec<&str> = command.response.split("%or").collect();
        let mut response: &str = split_on_or.get(0).unwrap();
        if split_on_or.len() > 1 {
            let mut rng = thread_rng();
            response = split_on_or.choose(&mut rng).unwrap()
        }

        if let Err(err) = msg
            .channel_id
            .send_message(&ctx.http, |m| {
                m.content(response);
                if let Some(attachment_urls) = &command.attachment_urls {
                    let files: Vec<AttachmentType> = attachment_urls
                        .iter()
                        .map(|url| AttachmentType::from(url.as_str()))
                        .collect();

                    m.add_files(files);
                };
                m
            })
            .await
        {
            error!("Error replying with dynamic command: {:?}", err);
        };
        return;
    }
}

#[tokio::main]
async fn main() {
    // This will load the environment variables located at `./.env`, relative to
    // the CWD. See `./.env.example` for an example on how to structure this.
    dotenv::dotenv().ok();

    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `RUST_LOG` to debug`.
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new(&token);

    // We will fetch your bot's owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Create the framework
    let framework: StandardFramework = StandardFramework::new()
        .configure(|c| {
            c.owners(owners.clone())
                .dynamic_prefix(|ctx, msg| {
                    Box::pin(async move {
                        let guild_id = match msg.guild_id {
                            Some(v) => v,
                            None => return None,
                        };

                        let data = ctx.data.read().await;
                        let conn = data.get::<DbContainer>().unwrap();
                        let guild_data = GuildData::get(conn, &guild_id.to_string()).await;
                        if let Ok(guild_data) = guild_data {
                            if let Some(prefix) = guild_data.and_then(|g| g.dynamic_prefix) {
                                return Some(prefix);
                            }
                        }
                        Some("?".to_string())
                    })
                })
                .prefix("??")
        })
        .unrecognised_command(unrecognised_command_hook)
        .group(&GENERAL_GROUP)
        .group(&COMMAND_GROUP)
        .bucket("basic", |b| b.delay(2).time_span(10).limit(3))
        .await;

    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    let db_conn = db::establish_connection(
        &env::var("DATABASE_URL").expect("Expected DATABASE_URL in the environment"),
    )
    .await;

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<DbContainer>(db_conn);
        data.insert::<DynCommandRateLimit>(Arc::new(RwLock::new(HashMap::new())));
        data.insert::<OwnersContainer>(Arc::new(RwLock::new(owners)));
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    info!("Starting bot!");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
