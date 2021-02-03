use serenity::{
    model::{
        id::GuildId,
        interactions::{ApplicationCommandOptionType, Interaction},
    },
    prelude::*,
};

pub async fn create_cmds(ctx: &Context, app_id: u64) {
    let guild_id = GuildId::from(579466138992508928);

    Interaction::create_guild_application_command(
        &ctx.http,
        guild_id,
        app_id,
        |i| {
            i.name("ping").description(
                "Pong! Test how long it takes for the bot to send a message.",
            )
        },
    )
    .await
    .unwrap();

    Interaction::create_guild_application_command(
        &ctx.http,
        guild_id,
        app_id,
        |i| {
            i.name("nsfw")
                .description("Enable or disable NSFW for a channel")
                .create_interaction_option(|o| {
                    o.name("channel")
                        .description("The channel to enable/disable NSFW for")
                        .kind(ApplicationCommandOptionType::Channel)
                        .required(false)
                })
                .create_interaction_option(|o| {
                    o.name("enabled")
                        .description(
                            "Choose whether or not NSFW should be enabled.",
                        )
                        .kind(ApplicationCommandOptionType::Boolean)
                        .required(false)
                })
        },
    )
    .await
    .unwrap();

    Interaction::create_guild_application_command(
        &ctx.http,
        guild_id,
        app_id,
        |i| {
            i.name("support")
                .description("Get the link to the bot's support server")
        },
    )
    .await
    .unwrap();

    Interaction::create_guild_application_command(
        &ctx.http,
        guild_id,
        app_id,
        |i| {
            i.name("invite")
                .description("Get an invite link for the bot")
        },
    )
    .await
    .unwrap();
}
