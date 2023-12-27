use crate::{say, Context};
use anyhow::Error;
use log::warn;

/// Creates or updates a custom command
#[poise::command(slash_command, prefix_command, owners_only)]
pub async fn setcommand(
    ctx: Context<'_>,
    #[description = "Command name"] name: String,
    #[description = "Response"] response: String,
) -> Result<(), Error> {
    let tree = &ctx.data().tree;

    if name.starts_with("--") {
        say!(ctx, "Commands may not start with --.");
        return Ok(());
    }

    if ctx
        .framework()
        .options
        .commands
        .iter()
        .any(|c| c.aliases.contains(&name.as_str()) || c.name == name)
    {
        say!(ctx, "Invalid name");
        return Ok(());
    }

    match tree.insert(name.to_ascii_lowercase(), response.as_str()) {
        Ok(_) => say!(ctx, "Command inserted into database"),
        Err(e) => {
            warn!("Error inserting command into database: {:?}", e);
            say!(ctx, "Error inserting command into database");
        }
    };

    Ok(())
}

/// Creates or updates a custom alias
#[poise::command(slash_command, prefix_command, owners_only)]
pub async fn setalias(
    ctx: Context<'_>,
    #[description = "Alias name"] name: String,
    #[description = "Command name"] command: String,
) -> Result<(), Error> {
    let tree = &ctx.data().tree;

    if ctx
        .framework()
        .options
        .commands
        .iter()
        .any(|c| c.aliases.contains(&name.as_str()) || c.name == name)
    {
        say!(ctx, "Invalid name");
        return Ok(());
    }

    if let Err(e) = tree.insert(
        format!("alias-{}", name.to_ascii_lowercase()),
        command.as_str(),
    ) {
        warn!("Error inserting alias into database: {:?}", e);
        say!(ctx, "Error inserting alias into database");
    } else {
        say!(ctx, "Alias inserted into database");
    }

    Ok(())
}

/// Removes a custom command
#[poise::command(slash_command, prefix_command, owners_only)]
pub async fn rmcommand(
    ctx: Context<'_>,
    #[description = "Command name"] name: String,
) -> Result<(), Error> {
    let tree = &ctx.data().tree;

    if let Err(e) = tree.remove(name.to_ascii_lowercase()) {
        warn!("Error removing command from database: {:?}", e);
        say!(ctx, "Error removing command from database");
    } else {
        say!(ctx, "Command removed from database");
    }

    Ok(())
}

/// Removes a custom alias
#[poise::command(slash_command, prefix_command, owners_only)]
pub async fn rmalias(
    ctx: Context<'_>,
    #[description = "Alias name"] name: String,
) -> Result<(), Error> {
    let tree = &ctx.data().tree;

    if let Err(e) = tree.remove(format!("alias-{}", name.to_ascii_lowercase())) {
        warn!("Error removing alias from database: {:?}", e);
        say!(ctx, "Error removing alias from database");
    } else {
        say!(ctx, "Alias removed from database");
    }

    Ok(())
}
