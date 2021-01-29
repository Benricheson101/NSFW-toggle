use serde_json::value::Value;
use serenity::{
    builder::{
        CreateInteractionResponse,
        CreateInteractionResponseData,
        EditChannel,
    },
    model::{
        channel::{Channel, GuildChannel},
        interactions::{
            Interaction,
        },
    },
    prelude::*,
};

pub async fn nsfw(ctx: Context, interaction: Interaction, _app_id: u64) {
    if let Some(data) = &interaction.data {
        let mut channel_id = interaction.channel_id.0;

        if let Some(opt) = data.options.iter().find(|x| x.name == "channel") {
            if let Some(val) = &opt.value {
                if let Value::String(chan) = val {
                    if let Ok(ch) = chan.parse::<u64>() {
                        channel_id = ch;
                    }
                }
            }
        };

        let mut channel: GuildChannel = match ctx.http.get_channel(channel_id).await {
            Ok(ch) => match ch {
                Channel::Guild(ch) => ch,
                _ => {
                    interaction.create_interaction_response(&ctx.http, |i: &mut CreateInteractionResponse| {
                        i.interaction_response_data(|d: &mut CreateInteractionResponseData| {
                            d.content(":x: You can only mark text channels as NSFW.")
                        })
                    }).await.ok();

                    return;
                }
            },
            Err(_) => {
                interaction.create_interaction_response(&ctx.http, |i: &mut CreateInteractionResponse| {
                    i.interaction_response_data(|d: &mut CreateInteractionResponseData| {
                        d.content(":x: An error occurred while finding that channel. Please ensure you are inputting a valid channel.")
                    })
                }).await.ok();

                return;
            }
        };

        let new_status = !channel.is_nsfw();

        let edited = channel.edit(&ctx.http, |e: &mut EditChannel| {
            e.nsfw(new_status)
        }).await;

        interaction.create_interaction_response(&ctx.http, |i: &mut CreateInteractionResponse| {
            i.interaction_response_data(|d: &mut CreateInteractionResponseData| {
                let content = match edited {
                    Ok(_) => format!(
                        ":white_check_mark: {} NSFW for <#{}>",
                        if new_status {"Enabled"} else {"Disabled"},
                        channel.id
                    ),
                    Err(_) => ":x: An error occurred.".to_string(),
                };

                d.content(content)
            })
        }).await.ok();
    }
}
