use crate::{check_output, Context};
use anyhow::Error;
use chrono::Utc;

// idk what clippy is smoking here, this isn't dead code
#[allow(dead_code)]
const CURRENT_ITERATION: &str = "v1";

/// Commit all current changes.
#[poise::command(slash_command, prefix_command, owners_only)]
pub async fn commit(
    ctx: Context<'_>,
    #[description = "Commit message. Use conventional commmits. See https://www.conventionalcommits.org/en/v1.0.0/"]
    #[rest]
    message: String,
) -> Result<(), Error> {
    check_output!("git", ["add", "-A"], "add changes to commit", ctx);
    check_output!("git", ["commit", "-am", &message], "commit changes", ctx);

    Ok(())
}

/// Discard all current changes. Beware.
#[poise::command(slash_command, prefix_command, owners_only)]
pub async fn reset(ctx: Context<'_>) -> Result<(), Error> {
    check_output!("git", ["pull", "origin"], "fetch latest changes", ctx);
    check_output!("git", ["clean", "-fd"], "clean working directory", ctx);

    check_output!(
        "git",
        ["checkout", CURRENT_ITERATION],
        "checkout remote state",
        ctx
    );

    check_output!(
        "git",
        ["reset", "--hard", &format!("origin/{}", CURRENT_ITERATION)],
        "reset to remote latest state",
        ctx
    );

    Ok(())
}

/// Open a pull request with current changes. Make sure to commit beforehand.
#[poise::command(slash_command, prefix_command, owners_only)]
pub async fn pull_request(
    ctx: Context<'_>,
    #[description = "The title of the pull request. Visible in the changelog."]
    #[rest]
    title: String,
) -> Result<(), Error> {
    let branch = format!("pull-request-{}", Utc::now().timestamp_millis());

    check_output!(
        "git",
        ["checkout", "-b", &branch],
        "create and checkout to new branch",
        ctx
    );

    // this is a hack to hide the remote lines on push
    // telling the user to create a pull request
    // this shouldn't remove any important errors, just the create a pull request line(s)
    check_output!(
        "sh",
        [
            "-c",
            &format!("git push -u origin {} 2>&1 | grep -v 'Create a pull request' | grep -v '/pull/new' | sed 's/^remote: $//' | sed '/^$/d'", branch)
        ],
        "push branch",
        format!("git push -u origin {}", branch),
        ctx,
        false
    );

    check_output!(
        "gh",
        [
            "pr",
            "create",
            "--title",
            &title,
            "--fill",
            "--base",
            CURRENT_ITERATION,
        ],
        "create pull request",
        ctx
    );

    Ok(())
}
