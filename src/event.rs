use crate::utils::get_alias;
use crate::Data;
use anyhow::Error;
use log::warn;
use poise::serenity_prelude::{Context, Message};
use poise::Event;
use std::str::from_utf8;

/// Handle all incoming events  
/// We're only interested in Messages right now, to implement custom logic
/// for handling custom commands
pub async fn event_handler(
    ctx: &Context,
    event: &Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    #[allow(clippy::single_match)]
    match event {
        Event::Message { new_message } => handle_message(new_message, data, ctx).await,
        _ => (),
    }

    Ok(())
}

pub async fn handle_message(new_message: &Message, data: &Data, ctx: &Context) {
    if new_message.guild(&ctx.cache).is_none() {
        return;
    }

    let content = new_message.content.strip_prefix(crate::PREFIX);

    if content.is_none() {
        return;
    }

    let mut command = content.unwrap().to_ascii_lowercase();

    if let Some(alias) = get_alias(&data.tree, command.clone()).await {
        command = alias;
    }

    match data.tree.get(command) {
        Ok(Some(response)) => {
            let message = from_utf8(&response).unwrap_or_else(|e| {
                warn!("Error converting command response to string: {:?}", e);
                "Error converting command response to string"
            });

            let channel = new_message.channel(&ctx).await;

            if channel.is_err() {
                return;
            }

            let _ = channel
                .unwrap()
                .guild()
                .unwrap()
                .send_message(&ctx, |m| m.content(message))
                .await;
        }
        Ok(None) => (),
        Err(e) => {
            warn!("Error getting command from database: {:?}", e);

            let _ = new_message
                .reply(&ctx, "Error getting command from database")
                .await;
        }
    }
}
