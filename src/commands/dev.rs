use crate::{check_output, say, Context};
use anyhow::Error;

/// Buttons to register slash commands
#[poise::command(prefix_command, owners_only, hide_in_help)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

/// Run shell commands. Dangerous.
#[poise::command(prefix_command, owners_only, hide_in_help)]
pub async fn bash(
    ctx: Context<'_>,
    #[description = "Command to run"]
    #[rest]
    command: String,
) -> Result<(), Error> {
    // pixelagent007
    if ctx.author().id.0 != 487247155741065229 {
        say!(ctx, "Go away gooberoo, this is dangerous.");

        return Ok(());
    }

    check_output!("bash", ["-c", &command], "execute command", ctx);
    Ok(())
}
