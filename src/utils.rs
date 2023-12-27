use log::{debug, error, warn};
use std::fmt::Debug;
use std::process::exit;
use std::str::from_utf8;

/// Log a message as an error and exit with code 1
pub fn fatal(message: &str, error: impl Debug) -> ! {
    error!("{}", message);
    debug!("got error: {:?}", error);
    exit(1)
}

/// Get the associated command for an alias
pub async fn get_alias(tree: &sled::Db, command: String) -> Option<String> {
    match tree.get(format!("alias-{}", command)) {
        Ok(Some(val)) => {
            let alias = from_utf8(val.as_ref()).unwrap_or_else(|e| {
                warn!("Error converting alias to string: {:?}", e);
                ""
            });

            Some(alias.to_string())
        }
        Ok(None) => None,
        Err(_) => None,
    }
}

/// Send a message in the current context.  
/// Supports standard `format!` formatting.  
/// Errors are automatically ignored.
/// Usage: `say!(ctx, <message>);`  
/// Example usage: `say!(ctx, "Hello {}!", "world");`  
#[macro_export]
macro_rules! say {
    ($ctx:expr, $($fmt:tt)*) => {{
        let _ = $ctx.say(format!($($fmt)*)).await;
    }};
}

/// Run a command and send the output to the current context inside a codeblock.  
/// The command is run inside the directory specified by the `PACKWIZ_REPO_PATH` environment variable.
/// Utilizes `say!` and `send_output` internally.  
/// Usage: `check_output!(<binary>, <arguments>, <short description of action>, <ctx>);`  
/// Example usage: `check_output!("git", ["add", "-A"], "add changes to commit", ctx);`  
#[macro_export]
macro_rules! check_output {
    ($program:expr, $args:expr, $action:expr, $ctx:expr) => {
        $crate::check_output!(
            $program,
            $args,
            $action,
            format!("{} {}", $program, $args.join(" ")),
            $ctx
        )
    };
    ($program:expr, $args:expr, $action:expr, $display_command:expr, $ctx:expr) => {{
        let cwd = std::env::var("PACKWIZ_REPO_PATH")
            .unwrap_or_else(|e| $crate::utils::fatal("PACKWIZ_REPO_PATH not found in env!", e));

        match std::process::Command::new($program)
            .args($args)
            .current_dir(cwd)
            .output()
        {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);

                let mut message = match (!stdout.is_empty(), !stderr.is_empty()) {
                    // stdout is not empty, stderr is empty
                    (true, false) => format!("```\n> {}\n{}```", $display_command, stdout),
                    // stdout is empty, stderr is not empty
                    (false, true) => format!("```\n> {}\n{}```", $display_command, stderr),
                    // both aren't empty
                    (true, true) => {
                        format!("```\n> {}\n{}\n{}```", $display_command, stdout, stderr)
                    }
                    // both are empty
                    (false, false) => {
                        format!("```\n> {}```\nCommand ran with no output", $display_command)
                    }
                };

                if !output.status.success() {
                    message.push_str(format!(
                        "\nFailed to {} - {} returned non-zero exit code",
                        $action, $program
                    ));
                }

                $crate::say!($ctx, message);
            }
            Err(_) => $crate::say!($ctx, "Failed to {} - command execution failed.", $action),
        };
    }};
}
