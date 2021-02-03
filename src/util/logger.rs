use std::collections::HashMap;

use serde_json::Value;
use serenity::{
    model::{
        guild::{Guild, GuildUnavailable},
        interactions::{ApplicationCommandInteractionDataOption, Interaction},
    },
    prelude::Context,
    utils::hashmap_to_json_map,
};

use crate::BotConfig;

#[cfg(feature = "cmd_log")]
pub async fn cmd_log(ctx: &Context, interaction: &Interaction) {
    if let Some(data) = &interaction.data {
        let config = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<BotConfig>()
                .expect("Expected `BotConfig` in TypeMap.")
                .clone()
        };

        let msg = format!(
            ":wrench: {}#{:04} (`{}`) used command `{}` in channel `{}` of guild `{}`\n> `{}`",
            interaction.member.user.name,
            interaction.member.user.discriminator,
            interaction.member.user.id,
            &data.name,
            &interaction.channel_id,
            &interaction.guild_id,
            interaction_to_string(&interaction)
        );

        let mut map = HashMap::new();
        map.insert("content", Value::String(msg));

        ctx.http
            .execute_webhook(
                config.cmd_log.id,
                &config.cmd_log.token,
                false,
                &hashmap_to_json_map(map),
            )
            .await
            .ok();
    }
}

#[cfg(feature = "server_log")]
pub async fn server_log<'a>(ctx: &Context, action: ServerLogAction<'a>) {
    let config = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<BotConfig>()
            .expect("Expected `BotConfig` in TypeMap.")
            .clone()
    };

    let msg = match action {
        ServerLogAction::Join(g) => {
            format!("{} `{}`", config.emojis.join_server, g.id.0)
        },
        ServerLogAction::Leave(g) => {
            format!("{} `{}`", config.emojis.leave_server, g.id.0)
        },
    };

    let mut map = HashMap::new();
    map.insert("content", Value::String(msg));

    ctx.http
        .execute_webhook(
            config.server_log.id,
            &config.server_log.token,
            false,
            &hashmap_to_json_map(map),
        )
        .await
        .ok();
}

#[cfg(feature = "server_log")]
pub enum ServerLogAction<'a> {
    Join(&'a Guild),
    Leave(&'a GuildUnavailable),
}

// TODO: less clone
#[cfg(feature = "cmd_log")]
fn interaction_to_string(interaction: &Interaction) -> String {
    let mut full_cmd = Vec::new();

    if let Some(data) = &interaction.data {
        full_cmd.push(data.name.clone());

        if data.options.len() != 0 {
            full_cmd.push(_interaction_to_string(&data.options));
        }
    }

    format!("/{}", full_cmd.join(" "))
}

#[cfg(feature = "cmd_log")]
fn _interaction_to_string(
    ops: &Vec<ApplicationCommandInteractionDataOption>,
) -> String {
    let mut chunks = Vec::new();

    for op in ops {
        chunks.push(
            if op.value.is_some() {
                format!("{}:", op.name)
            } else {
                op.name.clone()
            },
        );

        if op.options.len() != 0 {
            _interaction_to_string(&op.options);
        }

        if let Some(val) = &op.value {
            let v = match val {
                Value::String(v) => v.clone(),
                Value::Bool(v) => v.to_string(),
                Value::Number(v) => v.to_string(),
                _ => "".to_string(),
            };

            chunks.push(v);
        }
    }

    chunks.join(" ")
}
