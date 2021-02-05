use std::{fs::File, io::Read, sync::Arc};

use serde::Deserialize;
use serenity::prelude::TypeMapKey;

#[derive(Deserialize, Debug)]
pub struct BotConfig {
    pub bot_token: String,
    pub support_server: String,
    pub bot_invite: String,

    #[cfg(feature = "cmd_log")]
    pub cmd_log: Webhook,

    #[cfg(feature = "server_log")]
    pub server_log: Webhook,

    #[cfg(any(feature = "cmd_log", feature = "server_log"))]
    pub emojis: Emojis,

    #[cfg(feature = "bot_list_guild_count")]
    pub bot_lists: Vec<BotList>,
}

#[cfg(any(feature = "cmd_log", feature = "server_log"))]
#[derive(Deserialize, Debug)]
pub struct Webhook {
    pub id: u64,
    pub token: String,
}

#[cfg(any(feature = "cmd_log", feature = "server_log"))]
#[derive(Deserialize, Debug)]
pub struct Emojis {
    #[cfg(feature = "cmd_log")]
    pub cmd: String,

    #[cfg(feature = "server_log")]
    pub join_server: String,

    #[cfg(feature = "server_log")]
    pub leave_server: String,
}

#[cfg(feature = "bot_list_guild_count")]
#[derive(Deserialize, Debug)]
pub struct BotList {
    pub name: String, // TODO: enum
    pub auth: String,
}

impl TypeMapKey for BotConfig {
    type Value = Arc<BotConfig>;
}

pub fn parse_cfg(file_path: &str) -> BotConfig {
    let mut file = File::open(file_path)
        .expect(&format!("Unable to open bot config @ `{}`", &file_path));

    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let parsed: BotConfig = toml::from_str(&data)
        .expect(&format!("Error deserializing `{}`", file_path));

    parsed
}
