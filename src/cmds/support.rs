use crate::{
    default_response,
    SUPPORT_INVITE_URL,
};
use serenity::{
    model::interactions::Interaction,
    prelude::*,
};

pub async fn support(ctx: Context, interaction: Interaction, _app_id: u64) {
    default_response(
        &ctx,
        &interaction,
        SUPPORT_INVITE_URL.as_ref()
    ).await;
}
