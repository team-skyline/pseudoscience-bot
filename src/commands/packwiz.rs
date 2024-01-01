use crate::utils::fatal;
use crate::{check_output, say, Context};
use anyhow::Error;
use std::env::var;
use std::process::Command;

const DISALLOWED_SUBCOMMANDS: [&str; 4] = ["init", "completion", "utils", "serve"];
const HELP_SUBCOMMANDS: [&str; 2] = ["help", "--help"];

const HELP_1: &str = r###"
```
A command line tool for creating Minecraft modpacks

Usage:
  packwiz [command]

Available Commands:
  completion  Generate the autocompletion script for the specified shell
  curseforge  Manage curseforge-based mods
  help        Help about any command
  list        List all the mods in the modpack
  migrate     Migrate your Minecraft and loader versions to newer versions.
  modrinth    Manage modrinth-based mods
  pin         Pin a file so it does not get updated automatically
  refresh     Refresh the index file
  remove      Remove an external file from the modpack; equivalent to manually removing the file and running packwiz refresh
  settings    Manage pack settings
  unpin       Unpin a file so it receives updates
  update      Update an external file (or all external files) in the modpack
  url         Add external files from a direct download link, for sites that are not directly supported by packwiz
```
"###;

const HELP_2: &str = r###"
```
Flags:
      --cache string              The directory where packwiz will cache downloaded mods (default "/Users/oskar/Library/Caches/packwiz/cache")
      --config string             The config file to use (default "/Users/oskar/Library/Application Support/packwiz/.packwiz.toml")
  -h, --help                      help for packwiz
      --meta-folder string        The folder in which new metadata files will be added, defaulting to a folder based on the category (mods, resourcepacks, etc; if the category is unknown the current directory is used)
      --meta-folder-base string   The base folder from which meta-folder will be resolved, defaulting to the current directory (so you can put all mods/etc in a subfolder while still using the default behaviour) (default ".")
      --pack-file string          The modpack metadata file to use (default "pack.toml")
  -y, --yes                       Accept all prompts with the default or "yes" option (non-interactive mode) - may pick unwanted options in search results

Use "packwiz [command] --help" for more information about a command.
```
"###;

/// Run packwiz commands.  
/// Should be safe to run outside a container, but be cautious. Might allow RCE.
#[poise::command(slash_command, prefix_command, owners_only)]
pub async fn packwiz(
    ctx: Context<'_>,
    #[description = "Arguments to pass to packwiz"]
    #[rest]
    args: Option<String>,
) -> Result<(), Error> {
    let mut cmd = Command::new("packwiz");

    let cwd = var("PACKWIZ_REPO_PATH")
        .unwrap_or_else(|e| fatal("PACKWIZ_REPO_PATH not found in env!", e));

    cmd.current_dir(cwd);

    if let Some(args) = args {
        let command: String;

        match args.contains(' ') {
            true => {
                let split: Vec<&str> = args.split_whitespace().collect();
                command = split[0].to_string();

                if split[0] == "bulkinstall" {
                    let args = args
                        .strip_prefix("bulkinstall ")
                        .unwrap_or_else(|| unreachable!());
                    let lines: Vec<&str> = args.split('\n').collect();

                    for line in lines {
                        if line.starts_with('#') {
                            continue;
                        }

                        if line.is_empty() {
                            continue;
                        }

                        if line.contains("modrinth.com") {
                            check_output!(
                                "packwiz",
                                ["mr", "install", "-y", line],
                                "install mod",
                                ctx
                            );
                        }

                        if line.contains("curseforge.com") {
                            check_output!(
                                "packwiz",
                                ["cf", "install", "-y", line],
                                "install mod",
                                ctx
                            );
                        }
                    }

                    return Ok(());
                }

                cmd.args(split);
            }
            false => {
                command = args.clone();
                cmd.arg(args);
            }
        };

        let command = command.as_str();

        if HELP_SUBCOMMANDS.contains(&command) {
            say!(ctx, "{}", HELP_1);
            say!(ctx, "{}", HELP_2);
            return Ok(());
        }

        if DISALLOWED_SUBCOMMANDS.contains(&command) {
            say!(ctx, "That command is disabled");
            return Ok(());
        }
    }

    match cmd.output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            if stdout.is_empty() && stderr.is_empty() {
                say!(ctx, "Command ran with no output");
                return Ok(());
            }

            // Don't send the output if it's just the help message
            // that would be too long anyways
            if stdout.contains("Available Commands:") {
                say!(ctx, "{}", HELP_1);
                say!(ctx, "{}", HELP_2);

                return Ok(());
            }

            if !stderr.is_empty() {
                say!(ctx, "```\n{}```", stderr);
            }

            say!(ctx, "```\n{}```", stdout);
        }
        Err(_) => say!(ctx, "Error running packwiz"),
    }

    Ok(())
}
