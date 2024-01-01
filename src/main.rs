mod commands;
mod event;
mod utils;

extern crate log;

use crate::event::event_handler;
use crate::utils::fatal;
use anyhow::Error;
use dotenv::dotenv;
use log::debug;
use poise::serenity_prelude::{GatewayIntents, UserId};
use std::collections::HashSet;
use std::env;
use std::string::ToString;

const PREFIX: &str = "!";

struct Data {
    tree: sled::Db,
}

type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    if cfg!(debug_assertions) {
        debug!("Running in debug mode, loading .env file..");

        dotenv()
            .ok()
            .unwrap_or_else(|| fatal("Couldn't load .env file", ""));
    }

    let token =
        env::var("DISCORD_TOKEN").unwrap_or_else(|e| fatal("DISCORD_TOKEN not found in env!", e));
    let db_path = env::var("DB_PATH").unwrap_or_else(|e| fatal("DB_PATH not found in env!", e));

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::custom::setcommand(),
                commands::custom::rmalias(),
                commands::custom::rmcommand(),
                commands::custom::setalias(),
                commands::dev::register(),
                commands::fun::yawn(),
                commands::fun::setyawn(),
                commands::packwiz::packwiz(),
                commands::git::commit(),
                commands::git::pull_request(),
                commands::git::reset(),
                commands::dev::register(),
                commands::dev::bash(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(PREFIX.into()),
                case_insensitive_commands: true,
                ..Default::default()
            },
            event_handler: |ctx, event, framework, state| {
                Box::pin(event_handler(ctx, event, framework, state))
            },
            owners: {
                let ids = [
                    // khaoslatet
                    UserId::from(702874912955695139),
                    // pixelagent007
                    UserId::from(487247155741065229),
                ];
                HashSet::from(ids)
            },
            ..Default::default()
        })
        .token(token)
        .intents(GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT)
        .setup(|_ctx, _ready, _framework| {
            Box::pin(async move {
                // poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    tree: sled::open(db_path).unwrap_or_else(|e| {
                        fatal("Error opening database, check DB_PATH env variable", e)
                    }),
                })
            })
        });

    framework.run().await.unwrap();
}
