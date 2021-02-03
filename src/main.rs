mod cmds;
mod util;

use std::{
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use bot_cfg::BotConfig;
use cmds::{invite::*, nsfw::*, ping::*, support::*};
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
use util::{bot_cfg, logger};

/// The number of guilds the bot is in
pub struct GuildCount;
/// The application ID (used for slash command/interaction stuff)
pub struct ApplicationId;
/// If the bot is ready
pub struct IsReady;

impl TypeMapKey for GuildCount {
    type Value = Arc<AtomicUsize>;
}

impl TypeMapKey for ApplicationId {
    type Value = Arc<u64>;
}

impl TypeMapKey for IsReady {
    type Value = Arc<AtomicBool>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_create(&self, ctx: Context, guild: Guild) {
        let data_read = ctx.data.read().await;
        let guild_count = data_read
            .get::<GuildCount>()
            .expect("Expected GuildCount in TypeMap.")
            .clone();

        guild_count.fetch_add(1, Ordering::SeqCst);

        // #[cfg(feature = "server_log")]
        // {
        //     let is_ready = data_read
        //         .get::<IsReady>()
        //         .unwrap()
        //         .clone()
        //         .load(Ordering::Relaxed);

        //     if is_ready {
        //         logger::server_log(&ctx, ServerLogAction::Join(&guild)).await;
        //     }
        // }
    }

    async fn guild_delete(&self, ctx: Context, guild: GuildUnavailable) {
        #[cfg(feature = "server_log")]
        logger::server_log(&ctx, ServerLogAction::Leave(&guild)).await;

        let guild_count = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<GuildCount>()
                .expect("Expected GuildCount in TypeMap.")
                .clone()
        };

        guild_count.fetch_sub(1, Ordering::SeqCst);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let (config, client_id) = {
            let data_read = ctx.data.read().await;

            let config = data_read
                .get::<BotConfig>()
                .expect("Expected `BotConfig` in TypeMap.")
                .clone();

            let client_id = data_read
                .get::<ApplicationId>()
                .expect("Expected `ApplicationId` in TypeMap.")
                .clone();

            (config, client_id)
        };

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
        if let Some(shard) = ready.shard {
            if shard[0] == 0 {
                util::create_cmds::create_cmds(&ctx, ready.user.id.0).await;

                {
                    let mut data = ctx.data.write().await;
                    data.insert::<ApplicationId>(Arc::new(ready.user.id.0));
                    data.get::<IsReady>()
                        .unwrap()
                        .clone()
                        .swap(true, Ordering::Relaxed);
                }
            }

            println!(
                "Shard {} ready with {} guilds",
                shard[0],
                ready.guilds.len()
            );
        }

        loop {
            let guild_count = {
                let data_read = ctx.data.read().await;
                data_read
                    .get::<GuildCount>()
                    .expect("Expected GuildCount in TypeMap.")
                    .clone()
            };

            let activity = Activity::listening(&format!(
                "for /nsfw | {} servers",
                guild_count.load(Ordering::SeqCst)
            ));

            ctx.set_presence(Some(activity), OnlineStatus::Idle).await;

            sleep(Duration::from_secs(30)).await;
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        #[cfg(feature = "cmd_log")]
        logger::cmd_log(&ctx, &interaction).await;

        if let Some(data) = &interaction.data {
            let id = {
                let data_read = &ctx.data.read().await;

                *data_read
                    .get::<ApplicationId>()
                    .expect("Expected `ApplicationId` in TypeMap.")
                    .clone()
            };

            match data.name.as_str() {
                "ping" => ping(ctx, interaction, id).await,
                "nsfw" => nsfw(ctx, interaction, id).await,
                "support" => support(ctx, interaction, id).await,
                "invite" => invite(ctx, interaction, id).await,
                &_ => (),
            };
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

        data.insert::<GuildCount>(Arc::new(AtomicUsize::new(0)));
        data.insert::<BotConfig>(Arc::new(cfg));
        data.insert::<IsReady>(Arc::new(AtomicBool::new(false)));
    }

    if let Err(e) = client.start_autosharded().await {
        eprintln!("Client error: {:#?}", e);
    }
}

// TODO: make command creation have a dev and prod mode (switch between guild
// and global)
// TODO: post server count to bot lists
// TODO: get rid of warnings if not using default features?
// TODO: cache guild id instead of guild count
// TODO: make sure guild join log waits until after startup
