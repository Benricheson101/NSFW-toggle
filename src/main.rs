mod cmds;
mod util;

use lazy_static::lazy_static;
pub use util::responses::*;
use tokio::time::sleep;
use std::{
    env,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use serenity::{
    async_trait,
    model::{
        gateway::{Activity, Ready},
        guild::{Guild, GuildUnavailable},
        interactions::{
            Interaction,
        },
        user::OnlineStatus,
    },
    prelude::*,
};
use cmds::{
    ping::*,
    nsfw::*,
};

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
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        if let Some(shard) = ready.shard {
            if shard[0] == 0 {
                util::create_cmds::create_cmds(&ctx, ready.user.id.0).await;
            }

            println!("Shard {} ready with {} guilds", shard[0], ready.guilds.len());
        }

        loop {
            let guild_count = {
                let data_read = ctx.data.read().await;
                data_read.get::<GuildCount>().expect("Expected GuildCount in TypeMap.").clone()
            };

            let activity = Activity::listening(
                &format!("for /nsfw | {} servers", guild_count.load(Ordering::SeqCst))
            );

            ctx.set_presence(Some(activity), OnlineStatus::Idle).await;

            sleep(Duration::from_secs(30)).await;
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Some(data) = &interaction.data {
            match data.name.as_str() {
                "ping" => ping(ctx, interaction, *APPLICATION_ID).await,
                "nsfw" => nsfw(ctx, interaction, *APPLICATION_ID).await,
                &_ => (),
            };
        }
    }

    async fn guild_create(&self, ctx: Context, _guild: Guild) {
        let guild_count = {
            let data_read = ctx.data.read().await;
            data_read.get::<GuildCount>().expect("Expected GuildCount in TypeMap.").clone()
        };

        guild_count.fetch_add(1, Ordering::SeqCst);
    }

    async fn guild_delete(&self, ctx: Context, _guild: GuildUnavailable) {
        let guild_count = {
            let data_read = ctx.data.read().await;
            data_read.get::<GuildCount>().expect("Expected GuildCount in TypeMap.").clone()
        };

        guild_count.fetch_sub(1, Ordering::SeqCst);
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected `DISCORD_TOKEN`");

    let mut client = Client::builder(&token)
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
// TODO: make command creation have a dev and prod mode (switch between guild and global)
// TODO: cmds
//   TODO: support
//   TODO: help
