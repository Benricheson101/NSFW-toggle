mod cmds;
mod util;

use std::{
    env,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use cmds::{invite::*, nsfw::*, ping::*, support::*};
use lazy_static::lazy_static;
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

/// The number of guilds the bot is in
pub struct GuildCount;

impl TypeMapKey for GuildCount {
    type Value = Arc<AtomicUsize>;
}

lazy_static! {
    static ref APPLICATION_ID: u64 = env::var("APPLICATION_ID")
        .expect("Expected `APPLICATION_ID`")
        .parse()
        .unwrap();

    pub static ref BOT_INVITE_URL: String = env::var("BOT_INVITE_URL")
        .unwrap_or(
            format!(
                "https://discord.com/api/oauth2/authorize?client_id={}&permissions=2064&scope=bot%20applications.commands",
                *APPLICATION_ID
            )
        );

    pub static ref SUPPORT_INVITE_URL: String = env::var("SUPPORT_INVITE_URL")
        .unwrap_or(":x: The bot does not have a support server invite configured".to_string());

    static ref MENTION_REGEX: Regex = Regex::new(&format!("^<@!?{}>\\s?(help|commands|toggle|nsfw|invite|support)$", *APPLICATION_ID)).unwrap();
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_create(&self, ctx: Context, _guild: Guild) {
        let guild_count = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<GuildCount>()
                .expect("Expected GuildCount in TypeMap.")
                .clone()
        };

        guild_count.fetch_add(1, Ordering::SeqCst);
    }

    async fn guild_delete(&self, ctx: Context, _guild: GuildUnavailable) {
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
        if MENTION_REGEX.is_match(msg.content.as_str()) {
            msg.channel_id.say(
                &ctx.http,
                &format!(
                    ":information_source: I use slash commands now! You can view a list of my commands by pressing the `/` key.\n> If no commands appear, you may need to reauthorize with this link: <{}>",
                    *BOT_INVITE_URL
                )
            ).await.ok();
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        if let Some(shard) = ready.shard {
            if shard[0] == 0 {
                util::create_cmds::create_cmds(&ctx, ready.user.id.0).await;
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
        if let Some(data) = &interaction.data {
            match data.name.as_str() {
                "ping" => ping(ctx, interaction, *APPLICATION_ID).await,
                "nsfw" => nsfw(ctx, interaction, *APPLICATION_ID).await,
                "support" => support(ctx, interaction, *APPLICATION_ID).await,
                "invite" => invite(ctx, interaction, *APPLICATION_ID).await,
                &_ => (),
            };
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected `DISCORD_TOKEN`");

    let mut client: Client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;

        data.insert::<GuildCount>(Arc::new(AtomicUsize::new(0)));
    }

    if let Err(e) = client.start_autosharded().await {
        eprintln!("Client error: {:#?}", e);
    }
}

// TODO: add notice if you ping the bot to use slash commands
// TODO: setup rustfmt config
// TODO: make command creation have a dev and prod mode (switch between guild
// and global) TODO: cmds
//   TODO: support
//   TODO: help
