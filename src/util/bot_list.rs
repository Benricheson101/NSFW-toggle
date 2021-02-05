use std::{sync::Arc, time::Duration};

use reqwest::Client as ReqwestClient;
use serde::{Deserialize, Serialize};
use serenity::prelude::*;
use tokio::time::sleep;

use crate::{ApplicationId, BotConfig, Guilds, Shards};

#[cfg(feature = "bot_list_guild_count")]
#[derive(Deserialize, Serialize)]
struct DelBody {
    #[serde(rename = "guildCount")]
    guild_count: usize,
    #[serde(rename = "shardCount")]
    shard_count: u64,
}

#[cfg(feature = "bot_list_guild_count")]
#[derive(Deserialize, Serialize)]
struct BodBody {
    #[serde(rename = "guildCount")]
    guild_count: usize,
}

/// Post the server count to https://bots.ondiscord.xyz every 2 minutes
#[cfg(feature = "bot_list_guild_count")]
pub async fn bod(ctx: &Context) {
    println!(
        "Posting server count to https://bots.ondiscord.xyz every 120 seconds"
    );
    loop {
        let (config, bot_id, guild_count, ..) = data_from_cache(&ctx).await;

        let body = BodBody { guild_count };

        let list = config
            .bot_lists
            .iter()
            .find(|x| x.name == "botsondiscord")
            .unwrap();

        let client = ReqwestClient::new();

        client
            .post(&format!(
                "https://bots.ondiscord.xyz/bot-api/bots/{}/guilds",
                bot_id
            ))
            .header("authorization", list.auth.clone())
            .json(&body)
            .send()
            .await
            .ok();

        sleep(Duration::from_secs(120)).await;
    }
}

/// Post the server count to https://discordextremelist.xyz every 2 minutes
#[cfg(feature = "bot_list_guild_count")]
pub async fn del(ctx: &Context) {
    println!(
        "Posting server count to https://discordextremelist.xyz every 120 seconds"
    );
    loop {
        let (config, bot_id, guild_count, shard_count) =
            data_from_cache(&ctx).await;

        let list = config
            .bot_lists
            .iter()
            .find(|x| x.name == "discordextremelist")
            .unwrap();

        let client = ReqwestClient::new();

        let body = DelBody {
            guild_count,
            shard_count,
        };

        let res = client
            .post(&format!(
                "https://api.discordextremelist.xyz/v2/bot/{}/stats",
                bot_id
            ))
            .header("authorization", list.auth.clone())
            .json(&body)
            .send()
            .await
            .ok();

        sleep(Duration::from_secs(120)).await;
    }
}

#[cfg(feature = "bot_list_guild_count")]
async fn data_from_cache(ctx: &Context) -> (Arc<BotConfig>, u64, usize, u64) {
    let data_read = ctx.data.read().await;

    let guild_cache = data_read
        .get::<Guilds>()
        .expect("Expected `Guilds` in TypeMap.")
        .clone();

    let guild_cache = guild_cache.lock().await;

    let config = data_read
        .get::<BotConfig>()
        .expect("Expected `BotConfig` in TypeMap.")
        .clone();

    let bot_id = data_read
        .get::<ApplicationId>()
        .expect("Expected `ApplicationId` in TypeMap.")
        .clone();

    let shard_count = data_read
        .get::<Shards>()
        .expect("Expected `ApplicationId` in TypeMap.")
        .clone();

    (config, *bot_id, guild_cache.len(), *shard_count)
}
