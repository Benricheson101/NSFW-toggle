use serenity::{
    model::{
        interactions::{ApplicationCommandOptionType, Interaction},
    },
    prelude::*,
};

pub async fn create_cmds(ctx: &Context, app_id: u64) {
    Interaction::create_global_application_command(
        &ctx.http,
        app_id,
        |i| {
            i.name("ping").description(
                "Pong! Test how long it takes for the bot to send a message.",
            )
        },
    )
    .await
    .unwrap();

    Interaction::create_global_application_command(
        &ctx.http,
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

    Interaction::create_global_application_command(
        &ctx.http,
        app_id,
        |i| {
            i.name("support")
                .description("Get the link to the bot's support server")
        },
    )
    .await
    .unwrap();

    Interaction::create_global_application_command(
        &ctx.http,
        app_id,
        |i| {
            i.name("invite")
                .description("Get an invite link for the bot")
        },
    )
    .await
    .unwrap();
}
