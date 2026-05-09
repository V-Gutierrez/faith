use std::io::{self, Write};
use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};

use faith::cli;
use faith::cli::OutputFormat;
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
    Tsv,
    Csv,
}

impl From<Format> for OutputFormat {
    fn from(f: Format) -> Self {
        match f {
            Format::Json => OutputFormat::Json,
            Format::Text => OutputFormat::Text,
            Format::Tsv => OutputFormat::Tsv,
            Format::Csv => OutputFormat::Csv,
        }
    }
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
        #[arg(long, value_enum, default_value_t = Format::Json, global = true)]
        format: Format,
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
        #[arg(long, value_enum, default_value_t = Format::Json)]
        format: Format,
    },
    Diff {
        reference: String,
        #[arg(long, value_delimiter = ',', required = true)]
        tr: Vec<String>,
        #[arg(long, value_enum, default_value_t = Format::Json)]
        format: Format,
    },
    Stats {
        #[arg(long)]
        tr: Option<String>,
    },
    Completions {
        shell: String,
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
            cli::get::run(&store, &reference, &tr, format.into(), &mut out)
        }
        Cmd::Batch { tr, format } => {
            let store = Store::open(&path)?;
            let stdin = io::stdin();
            let mut lock = stdin.lock();
            cli::batch::run(&store, &tr, format.into(), &mut lock, &mut out)
        }
        Cmd::List { what, format } => {
            let store = Store::open(&path)?;
            match what {
                ListKind::Translations { lang, installed } => cli::list::run_translations(
                    &store,
                    lang.as_deref(),
                    installed,
                    format.into(),
                    &mut out,
                ),
                ListKind::Books { tr } => {
                    cli::list::run_books(&store, &tr, format.into(), &mut out)
                }
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
            format,
        } => {
            let store = Store::open(&path)?;
            let s = match scope {
                ScopeArg::All => cli::random::Scope::All,
                ScopeArg::Ot => cli::random::Scope::Ot,
                ScopeArg::Nt => cli::random::Scope::Nt,
            };
            cli::random::run(
                &store,
                tr.as_deref(),
                book.as_deref(),
                s,
                seed,
                format.into(),
                &mut out,
            )
        }
        Cmd::Diff {
            reference,
            tr,
            format,
        } => {
            let store = Store::open(&path)?;
            cli::diff::run(&store, &reference, &tr, format.into(), &mut out)
        }
        Cmd::Stats { tr } => {
            let store = Store::open(&path)?;
            let dir = store::data_dir()?;
            cli::stats::run(&store, tr.as_deref(), &dir, &mut out)
        }
        Cmd::Completions { shell } => cli::completions::run(&shell, &mut out),
    }
}
