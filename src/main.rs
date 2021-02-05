#![allow(unused_imports)]

mod cmds;
mod util;

use std::{sync::Arc, time::Duration};

pub use bot_cfg::BotConfig;
use cmds::{invite::*, nsfw::*, ping::*, support::*};
#[cfg(any(feature = "cmd_log", feature = "server_log"))]
use logger::ServerLogAction;
use regex::Regex;
use serenity::{
    async_trait,
    model::{
        channel::Message,
        gateway::{Activity, Ready},
        guild::{Guild, GuildUnavailable},
        interactions::Interaction,
        user::OnlineStatus,
    },
    prelude::*,
};
use tokio::time::sleep;
pub use util::responses::*;
use util::{bot_cfg, bot_list, logger};

// cache stuff

/// The guilds the bot is in
pub struct Guilds;
/// The application ID (used for slash command/interaction stuff)
pub struct ApplicationId;
/// The number of shards spawned
pub struct Shards;

impl TypeMapKey for ApplicationId {
    type Value = Arc<u64>;
}

impl TypeMapKey for Guilds {
    type Value = Arc<Mutex<Vec<u64>>>;
}

impl TypeMapKey for Shards {
    type Value = Arc<u64>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_create(&self, ctx: Context, guild: Guild) {
        let data_read = ctx.data.read().await;

        let guilds = data_read
            .get::<Guilds>()
            .expect("Expected Guilds in TypeMap.")
            .clone();

        let mut g = guilds.lock().await;

        if !g.contains(&guild.id.0) {
            g.push(guild.id.0);

            #[cfg(feature = "server_log")]
            logger::server_log(&ctx, ServerLogAction::Join(&guild), g.len())
                .await;
        }
    }

    async fn guild_delete(&self, ctx: Context, guild: GuildUnavailable) {
        // Guild is unavailable due to an outage but still in the guild
        if guild.unavailable {
            return;
        }

        let data_read = ctx.data.read().await;
        let guilds = data_read
            .get::<Guilds>()
            .expect("Expected `Guilds` in TypeMap.")
            .clone();

        let mut g = guilds.lock().await;

        if let Some(pos) = g.iter().position(|x| *x == guild.id.0) {
            g.remove(pos);

            #[cfg(feature = "server_log")]
            logger::server_log(&ctx, ServerLogAction::Leave(&guild), g.len())
                .await;
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let data_read = ctx.data.read().await;

        let config = data_read
            .get::<BotConfig>()
            .expect("Expected `BotConfig` in TypeMap.")
            .clone();

        let client_id = data_read
            .get::<ApplicationId>()
            .expect("Expected `ApplicationId` in TypeMap.")
            .clone();

        let mention_regex = Regex::new(&format!(
            "^<@!?{}>\\s?(help|commands|toggle|nsfw|invite|support)$",
            client_id
        ))
        .unwrap();

        if mention_regex.is_match(msg.content.as_str()) {
            msg.channel_id.say(
                &ctx.http,
                &format!(
                    ":information_source: I use slash commands now! You can view a list of my commands by pressing the `/` key.\n> If no commands appear, you may need to reauthorize with this link: <{}>",
                    &config.bot_invite
                )
            ).await.ok();
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        {
            let data_read = ctx.data.read().await;

            let mut new_guilds =
                ready.guilds.iter().map(|x| x.id().0).collect::<Vec<u64>>();

            let g = data_read
                .get::<Guilds>()
                .expect("Expected `Guilds` in TypeMap.")
                .clone();

            let mut guild_cache = g.lock().await;

            guild_cache.append(&mut new_guilds);
        }

        if let Some(shard) = ready.shard {
            if shard[0] == 0 {
                util::create_cmds::create_cmds(&ctx, ready.user.id.0).await;

                {
                    let mut data = ctx.data.write().await;
                    data.insert::<ApplicationId>(Arc::new(ready.user.id.0));
                    data.insert::<Shards>(Arc::new(shard[1]));
                }
            }

            println!(
                "Shard {} ready with {} guilds",
                shard[0],
                ready.guilds.len()
            );

            #[cfg(feature = "bot_list_guild_count")]
            if shard[0] == shard[1] - 1 {
                bot_list::bod(&ctx).await;
                bot_list::del(&ctx).await;
                println!("Last shard ready");
            }
        }

        loop {
            let guild_count = {
                let data_read = ctx.data.read().await;

                data_read
                    .get::<Guilds>()
                    .expect("Expected `Guilds` in TypeMap.")
                    .clone()
                    .lock()
                    .await
                    .len()
            };

            let activity = Activity::listening(&format!(
                "/nsfw | {} servers",
                guild_count,
            ));

            ctx.set_presence(Some(activity), OnlineStatus::Idle).await;

            sleep(Duration::from_secs(30)).await;
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Some(data) = &interaction.data {
            let id = {
                let data_read = &ctx.data.read().await;

                *data_read
                    .get::<ApplicationId>()
                    .expect("Expected `ApplicationId` in TypeMap.")
                    .clone()
            };

            match data.name.as_str() {
                "ping" => ping(&ctx, &interaction, id).await,
                "nsfw" => nsfw(&ctx, &interaction, id).await,
                "support" => support(&ctx, &interaction, id).await,
                "invite" => invite(&ctx, &interaction, id).await,
                &_ => (),
            };

            #[cfg(feature = "cmd_log")]
            logger::cmd_log(&ctx, &interaction).await;
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let cfg = bot_cfg::parse_cfg("config.toml");

    let mut client: Client = Client::builder(&cfg.bot_token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;

        data.insert::<BotConfig>(Arc::new(cfg));
        data.insert::<Guilds>(Arc::new(Mutex::new(Vec::new())));
    }

    if let Err(e) = client.start_autosharded().await {
        eprintln!("Client error: {:#?}", e);
    }
}

// TODO: make command creation have a dev and prod mode (switch between guild
// and global)
// TODO: post server count to bot lists
// TODO: get rid of warnings if not using default features?
