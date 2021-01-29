mod cmds;
mod util;

use std::env;
use serenity::{
    async_trait,
    model::{
        gateway::{Activity, Ready},
        user::OnlineStatus,
        interactions::{
            Interaction,
        },
    },
    prelude::*,
};
use cmds::{
    ping::*,
    nsfw::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        if let Some(shard) = ready.shard {
            if shard[0] == 0 {
                util::create_cmds::create_cmds(&ctx, ready.user.id.0).await;
            }

            println!("Shard {} connected", shard[0]);
        } else {
            println!("Connected");
        }

        let activity = Activity::playing("with shash commands!");
        let status = OnlineStatus::Idle;

        ctx.set_presence(Some(activity), status).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Some(data) = &interaction.data {
            let app_id: u64 = env::var("APPLICATION_ID")
                .expect("No `APPLICATION_ID` was provided")
                .parse()
                .unwrap();

            match data.name.as_str() {
                "ping" => ping(ctx, interaction, app_id).await,
                "nsfw" => nsfw(ctx, interaction, app_id).await,
                &_ => (),
            };
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected `Discord_TOKEN`");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(e) = client.start_autosharded().await {
        eprintln!("Client error: {:#?}", e);
    }
}

// TODO: add notice if you ping the bot to use slash commands
// TODO: perms
// TODO: setup rustfmt config
// TODO: make command creation have a dev and prod mode (switch between guild and global)
// TODO: cmds
//   TODO: support
//   TODO: help
