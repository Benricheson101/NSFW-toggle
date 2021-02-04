use serenity::{model::interactions::Interaction, prelude::*};

use crate::{default_response, BotConfig};

pub async fn support(ctx: &Context, interaction: &Interaction, _app_id: u64) {
    let config = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<BotConfig>()
            .expect("Expected `BotConfig` in TypeMap.")
            .clone()
    };

    default_response(&ctx, &interaction, &config.support_server).await;
}
