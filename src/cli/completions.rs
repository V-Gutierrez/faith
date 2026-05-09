//! `faith completions <shell>` — emit shell completion script to stdout.
//!
//! The completion app is hand-built here (rather than reused from the
//! `main.rs` `clap::Parser` derive) so the library is self-contained and the
//! integration tests can exercise generation without crossing the binary
//! boundary. Keep this in sync with the subcommand surface in `main.rs`.

use std::io::Write;

use clap::{Arg, ArgAction, Command};
use clap_complete::{generate, Shell};

use crate::error::{FaithError, Result};
use crate::schema::ErrorOut;

/// Run the `completions` subcommand. `shell` is the lowercase shell name
/// (`bash`, `zsh`, `fish`, `powershell`, `elvish`). Unknown shells emit an
/// `E_REF_PARSE` envelope (exit 2) and return.
pub fn run<W: Write>(shell: &str, out: &mut W) -> Result<i32> {
    let s = match parse_shell(shell) {
        Some(s) => s,
        None => {
            let e = FaithError::RefParse {
                input: format!(
                    "unknown shell '{shell}' (try: bash, zsh, fish, powershell, elvish)"
                ),
            };
            let eo = ErrorOut::from_err(&e);
            serde_json::to_writer(&mut *out, &eo)?;
            writeln!(out)?;
            return Ok(e.exit_code_int());
        }
    };
    let mut app = build_app();
    generate(s, &mut app, "faith", out);
    Ok(0)
}

fn parse_shell(s: &str) -> Option<Shell> {
    match s.to_ascii_lowercase().as_str() {
        "bash" => Some(Shell::Bash),
        "zsh" => Some(Shell::Zsh),
        "fish" => Some(Shell::Fish),
        "powershell" | "pwsh" => Some(Shell::PowerShell),
        "elvish" => Some(Shell::Elvish),
        _ => None,
    }
}

fn build_app() -> Command {
    Command::new("faith")
        .about("Agent-first Bible CLI")
        .subcommand_required(true)
        .subcommand(
            Command::new("get")
                .arg(Arg::new("reference").required(true))
                .arg(Arg::new("tr").long("tr").value_delimiter(','))
                .arg(Arg::new("format").long("format")),
        )
        .subcommand(
            Command::new("batch")
                .arg(Arg::new("tr").long("tr").required(true))
                .arg(Arg::new("format").long("format")),
        )
        .subcommand(
            Command::new("list")
                .subcommand_required(true)
                .subcommand(
                    Command::new("translations")
                        .arg(Arg::new("lang").long("lang"))
                        .arg(
                            Arg::new("installed")
                                .long("installed")
                                .action(ArgAction::SetTrue),
                        ),
                )
                .subcommand(Command::new("books").arg(Arg::new("tr").long("tr").required(true))),
        )
        .subcommand(
            Command::new("install").arg(Arg::new("translations").required(true).num_args(1..)),
        )
        .subcommand(Command::new("manifest"))
        .subcommand(
            Command::new("info")
                .arg(Arg::new("book").required(true))
                .arg(Arg::new("tr").long("tr")),
        )
        .subcommand(
            Command::new("random")
                .arg(Arg::new("tr").long("tr"))
                .arg(Arg::new("lang").long("lang"))
                .arg(Arg::new("book").long("book"))
                .arg(Arg::new("scope").long("scope"))
                .arg(Arg::new("seed").long("seed")),
        )
        .subcommand(
            Command::new("diff")
                .arg(Arg::new("reference").required(true))
                .arg(
                    Arg::new("tr")
                        .long("tr")
                        .value_delimiter(',')
                        .required(true),
                ),
        )
        .subcommand(Command::new("stats").arg(Arg::new("tr").long("tr")))
        .subcommand(Command::new("completions").arg(Arg::new("shell").required(true)))
        .subcommand(
            Command::new("cache")
                .subcommand_required(true)
                .subcommand(Command::new("size"))
                .subcommand(
                    Command::new("clear").arg(
                        Arg::new("confirm")
                            .long("confirm")
                            .action(ArgAction::SetTrue),
                    ),
                )
                .subcommand(Command::new("path")),
        )
}
