//! `faith` CLI entry point.
//!
//! Pre-alpha scaffold. Subcommands land per the SPEC roadmap.

use std::process::ExitCode;

fn main() -> ExitCode {
    eprintln!(
        "faith {} — pre-alpha scaffold. See docs/SPEC.md for the v0.1 plan.",
        env!("CARGO_PKG_VERSION")
    );
    ExitCode::from(0)
}
