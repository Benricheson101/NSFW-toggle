use crate::{
    default_response,
    BOT_INVITE_URL,
};
use serenity::{
    model::interactions::Interaction,
    prelude::*,
};

pub async fn invite(ctx: Context, interaction: Interaction, _app_id: u64) {
    default_response(
        &ctx,
        &interaction,
        BOT_INVITE_URL.as_ref()
    ).await;
}
