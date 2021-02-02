use std::time::Instant;
use serenity::{
    model::interactions::{
        InteractionResponseType,
        Interaction
    },
    prelude::*,
};

pub async fn ping(ctx: Context, interaction: Interaction, app_id: u64) {
    let now = Instant::now();

    interaction.create_interaction_response(&ctx.http, |i| {
        i.kind(InteractionResponseType::ChannelMessage)
            .interaction_response_data(|d| {
                d.content(":ping_pong: Pong!")
            })
    }).await.ok();

    let elapsed_ms = now.elapsed().as_millis();

    interaction.edit_original_interaction_response(
        &ctx.http,
        app_id,
        |e| {
            e.content(&format!(":ping_pong: Pong! Message sent in {}ms", elapsed_ms))
        }
    ).await.ok();
}
