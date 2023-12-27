use crate::{say, Context};
use anyhow::Error;
use log::warn;
use std::str::from_utf8;

const DEFAULT_YAWN: &str = r"
Hello everyone! Just wanted to say good morning/good afternoon/good evening/good night/hello/good bye to all of you! Also, well pole.
";

/// Sets a users yawn
#[poise::command(slash_command, prefix_command, check = "check")]
pub async fn setyawn(
    ctx: Context<'_>,
    #[description = "Arguments"] yawn: String,
) -> Result<(), Error> {
    let tree = &ctx.data().tree;

    match tree.insert(format!("--yawn-{}", ctx.author().id.0), yawn.as_str()) {
        Ok(_) => say!(ctx, "Yawn set"),
        Err(e) => {
            warn!("Error inserting yawn into database: {:?}", e);
            say!(ctx, "Error inserting yawn into database");
        }
    };

    Ok(())
}

/// Gets the users yawn
#[poise::command(slash_command, prefix_command)]
pub async fn yawn(ctx: Context<'_>) -> Result<(), Error> {
    let tree = &ctx.data().tree;

    match tree.get(format!("--yawn-{}", ctx.author().id.0)) {
        Ok(Some(response)) => {
            let message = from_utf8(&response).unwrap_or_else(|e| {
                warn!("Error converting yawn to string: {:?}", e);
                "Error converting yawn to string"
            });

            say!(ctx, "{}", message);
        }
        Ok(None) => say!(ctx, "{}", DEFAULT_YAWN),
        Err(e) => {
            warn!("Error getting yawn from database: {:?}", e);
            say!(ctx, "Error getting yawn from database");
        }
    }

    Ok(())
}

async fn check(ctx: Context<'_>) -> Result<bool, Error> {
    // member2, member1, Khaos
    let roles = [
        1145786846404677662,
        1126619331007107143,
        1145786499934199932,
    ];

    for role in roles {
        if let Ok(b) = ctx
            .author()
            .has_role(&ctx, ctx.guild_id().unwrap().0, role)
            .await
        {
            if b {
                return Ok(b);
            }
        }
    }

    say!(ctx, "Thou art not worthy of this command, peasant.");

    Ok(false)
}
