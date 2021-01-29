use serenity::{
    builder::{
        CreateInteraction,
        CreateInteractionOption,
    },
    model::{
        id::GuildId,
        interactions::{
            ApplicationCommandOptionType,
            Interaction,
        }
    },
    prelude::*,
};

pub async fn create_cmds(ctx: &Context, app_id: u64) {
    let guild_id = GuildId::from(579466138992508928);

    Interaction::create_guild_application_command(
        &ctx.http,
        guild_id,
        app_id,
        |i: &mut CreateInteraction| {
            i.name("ping")
                .description("Pong! Test how long it takes for the bot to send a message.")
        }
    ).await.unwrap();

    Interaction::create_guild_application_command(
        &ctx.http,
        guild_id,
        app_id,
        |i: &mut CreateInteraction| i
            .name("nsfw")
            .description("Enable or disable NSFW for a channel")
            .create_interaction_option(|o: &mut CreateInteractionOption| {
                o.name("channel")
                    .description("The channel to enable/disable NSFW for")
                    .kind(ApplicationCommandOptionType::Channel)
                    .required(false)
            })
            .create_interaction_option(|o: &mut CreateInteractionOption| {
                o.name("enabled")
                    .description("Choose whether or not NSFW should be enabled.")
                    .kind(ApplicationCommandOptionType::Boolean)
                    .required(false)
            })
    ).await.unwrap();
}
