use if_chain::if_chain;
use serde_json::value::Value;
use serenity::{
    builder::EditChannel,
    http::error::Error as HttpError,
    model::{
        channel::{Channel, ChannelType, GuildChannel},
        interactions::Interaction,
        permissions::Permissions,
    },
    prelude::*,
    Error as SerenityError,
};

use crate::default_response;

pub async fn nsfw(ctx: &Context, interaction: &Interaction, _app_id: u64) {
    if let Some(data) = &interaction.data {
        let perms = if_chain! {
            if let Some(perms) = &interaction.member.permissions;
            if let Ok(p) = perms.parse::<u64>();
            then {
                Permissions::from_bits_truncate(p)
            } else {
                return;
            }
        };

        if !(perms.manage_channels() || perms.administrator()) {
            default_response(
                &ctx,
                &interaction,
                ":x: You do not have permission to use this command.",
            )
            .await;

            return;
        }

        let mut channel_id = interaction.channel_id.0;

        if_chain! {
            if let Some(channel) = data.options.iter().find(|x| x.name == "channel");
            if let Some(Value::String(channel)) = &channel.value;
            if let Ok(channel) = channel.parse::<u64>();
            then {
                channel_id = channel;
            }
        }

        let mut channel: GuildChannel = match ctx
            .http
            .get_channel(channel_id)
            .await
        {
            Ok(ch) => match ch {
                Channel::Guild(ch) if ch.kind == ChannelType::Text => ch,
                _ => {
                    default_response(
                        &ctx,
                        &interaction,
                        ":x: You can only mark text channels as NSFW.",
                    )
                    .await;

                    return;
                },
            },
            Err(_) => {
                default_response(
                    &ctx,
                    &interaction,
                    ":x: An error occurred while finding that channel. Please ensure you are inputting a valid channel.",
                ).await;

                return;
            },
        };

        let mut new_status = !channel.is_nsfw();

        if_chain! {
            if let Some(enabled) = &data.options.iter().find(|x| x.name == "enabled");
            if let Some(enabled) = &enabled.value;
            if let Value::Bool(enabled) = enabled;
            then {
                new_status = *enabled;
            }
        }

        if new_status == channel.is_nsfw() {
            default_response(
                &ctx,
                &interaction,
                &format!(
                    ":x: That channel has already been marked as {}",
                    if channel.is_nsfw() { "NSFW" } else { "SFW" }
                ),
            )
            .await;

            return;
        }

        let res = channel
            .edit(&ctx.http, |e: &mut EditChannel| e.nsfw(new_status))
            .await;

        let msg = match res {
            Ok(_) => format!(
                ":white_check_mark: Successfully **{}** NSFW for <#{}>",
                if new_status { "enabled" } else { "disabled" },
                channel.id
            ),

            Err(err) => {
                if_chain! {
                    if let SerenityError::Http(err) = err;
                    if let HttpError::UnsuccessfulRequest(err) = *err;
                    if err.error.code == 50013;
                    then {
                        ":x: I do not have permission to edit this channel. Make sure I have `MANAGE_CHANNELS`".to_string()
                    } else {
                        ":x: An error occurred. Please try again".to_string()
                    }
                }
            },
        };

        default_response(&ctx, &interaction, &msg).await;
    }
}
