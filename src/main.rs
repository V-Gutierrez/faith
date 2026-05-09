use std::io::{self, Write};
use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};

use faith::cli;
use faith::error::FaithError;
use faith::store::{self, Store};

#[derive(Debug, Parser)]
#[command(name = "faith", version, about = "Agent-first Bible CLI")]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Format {
    Json,
    Text,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    Get {
        reference: String,
        #[arg(long, value_delimiter = ',')]
        tr: Vec<String>,
        #[arg(long, value_enum, default_value_t = Format::Json)]
        format: Format,
    },
    Batch {
        #[arg(long)]
        tr: String,
        #[arg(long, value_enum, default_value_t = Format::Json)]
        format: Format,
    },
    List {
        #[command(subcommand)]
        what: ListKind,
    },
    Install {
        #[arg(required = true)]
        translations: Vec<String>,
    },
    Manifest,
    Info {
        book: String,
        #[arg(long)]
        tr: Option<String>,
    },
    Random {
        #[arg(long)]
        tr: Option<String>,
        #[arg(long)]
        book: Option<String>,
        #[arg(long, value_enum, default_value_t = ScopeArg::All)]
        scope: ScopeArg,
        #[arg(long)]
        seed: Option<u64>,
    },
    Diff {
        reference: String,
        #[arg(long, value_delimiter = ',', required = true)]
        tr: Vec<String>,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum ScopeArg {
    All,
    Ot,
    Nt,
}

#[derive(Debug, Subcommand)]
enum ListKind {
    Translations {
        #[arg(long)]
        lang: Option<String>,
        #[arg(long)]
        installed: bool,
    },
    Books {
        #[arg(long)]
        tr: String,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match dispatch(cli) {
        Ok(code) => ExitCode::from(code as u8),
        Err(e) => {
            let err = faith::schema::ErrorOut::from_err(&e);
            let stdout = io::stdout();
            let mut h = stdout.lock();
            let _ = serde_json::to_writer(&mut h, &err);
            let _ = writeln!(h);
            ExitCode::from(e.exit_code_int() as u8)
        }
    }
}

fn dispatch(cli: Cli) -> Result<i32, FaithError> {
    let path = store::default_db_path()?;
    let stdout = io::stdout();
    let mut out = stdout.lock();

    match cli.command {
        Cmd::Get {
            reference,
            tr,
            format,
        } => {
            let store = Store::open(&path)?;
            cli::get::run(
                &store,
                &reference,
                &tr,
                matches!(format, Format::Text),
                &mut out,
            )
        }
        Cmd::Batch { tr, format } => {
            let store = Store::open(&path)?;
            let stdin = io::stdin();
            let mut lock = stdin.lock();
            cli::batch::run(
                &store,
                &tr,
                matches!(format, Format::Text),
                &mut lock,
                &mut out,
            )
        }
        Cmd::List { what } => {
            let store = Store::open(&path)?;
            match what {
                ListKind::Translations { lang, installed } => {
                    cli::list::run_translations(&store, lang.as_deref(), installed, &mut out)
                }
                ListKind::Books { tr } => cli::list::run_books(&store, &tr, &mut out),
            }
        }
        Cmd::Install { translations } => {
            let mut store = Store::open(&path)?;
            cli::install::run(&mut store, &translations, &mut out)
        }
        Cmd::Manifest => {
            let store = Store::open(&path)?;
            cli::manifest::run(&store, &mut out)
        }
        Cmd::Info { book, tr } => {
            let store = Store::open(&path)?;
            cli::info::run(&store, &book, tr.as_deref(), &mut out)
        }
        Cmd::Random {
            tr,
            book,
            scope,
            seed,
        } => {
            let store = Store::open(&path)?;
            let s = match scope {
                ScopeArg::All => cli::random::Scope::All,
                ScopeArg::Ot => cli::random::Scope::Ot,
                ScopeArg::Nt => cli::random::Scope::Nt,
            };
            cli::random::run(&store, tr.as_deref(), book.as_deref(), s, seed, &mut out)
        }
        Cmd::Diff { reference, tr } => {
            let store = Store::open(&path)?;
            cli::diff::run(&store, &reference, &tr, &mut out)
        }
    }
}
