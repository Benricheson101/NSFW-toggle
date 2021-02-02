use serenity::{
    builder::{CreateInteractionResponse, CreateInteractionResponseData},
    model::interactions::{
        Interaction,
        InteractionApplicationCommandCallbackDataFlags as Flags,
        InteractionResponseType,
    },
    prelude::*,
};

/// Respond with an ephemeral message
pub async fn default_response(
    ctx: &Context,
    interaction: &Interaction,
    msg: &str,
) {
    interaction
        .create_interaction_response(
            &ctx.http,
            |i: &mut CreateInteractionResponse| {
                i.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(
                        |d: &mut CreateInteractionResponseData| {
                            d.flags(Flags::EPHEMERAL).content(msg)
                        },
                    )
            },
        )
        .await
        .ok();
}
