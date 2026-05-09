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
        #[arg(long)]
        lang: Option<String>,
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
        lang: Option<String>,
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
        #[arg(long)]
        lang: Option<String>,
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
    Cache {
        #[command(subcommand)]
        subcommand: CacheKind,
    },
    Search {
        query: String,
        #[arg(long)]
        tr: Option<String>,
        #[arg(long)]
        lang: Option<String>,
        #[arg(long)]
        limit: Option<u32>,
        #[arg(long, value_enum, default_value_t = Format::Json)]
        format: Format,
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

#[derive(Debug, Subcommand)]
enum CacheKind {
    /// Show cache and DB sizes
    Size,
    /// Clear cache directory (requires --confirm)
    Clear {
        #[arg(long)]
        confirm: bool,
    },
    /// Print cache directory path
    Path,
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
            lang,
            format,
        } => {
            let store = Store::open(&path)?;
            let trs = resolve_lang_or_tr(&tr, lang.as_deref());
            cli::get::run(&store, &reference, &trs, format.into(), &mut out)
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
            lang,
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
                lang.as_deref(),
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
            lang,
            format,
        } => {
            let store = Store::open(&path)?;
            let trs = resolve_lang_or_tr(&tr, lang.as_deref());
            cli::diff::run(&store, &reference, &trs, format.into(), &mut out)
        }
        Cmd::Stats { tr } => {
            let store = Store::open(&path)?;
            let dir = store::data_dir()?;
            cli::stats::run(&store, tr.as_deref(), &dir, &mut out)
        }
        Cmd::Completions { shell } => cli::completions::run(&shell, &mut out),
        Cmd::Cache { subcommand } => {
            let sub_name = match subcommand {
                CacheKind::Size => "size",
                CacheKind::Clear { .. } => "clear",
                CacheKind::Path => "path",
            };
            let confirm = matches!(subcommand, CacheKind::Clear { confirm: true });
            cli::cache::run(sub_name, confirm, &mut out)
        }
        Cmd::Search {
            query,
            tr,
            lang,
            limit,
            format,
        } => {
            let store = Store::open(&path)?;
            let resolved_tr = match (&tr, &lang) {
                (Some(t), _) => Some(t.as_str()),
                (None, Some(l)) => cli::resolve_by_lang(l).or(None),
                _ => None,
            };
            cli::search::run(&store, &query, resolved_tr, limit, format.into(), &mut out)
        }
    }
}

/// When `--tr` is empty but `--lang` is set, resolve to a single-element vec.
/// Falls through to original tr list otherwise.
fn resolve_lang_or_tr(tr: &[String], lang: Option<&str>) -> Vec<String> {
    if !tr.is_empty() {
        return tr.to_vec();
    }
    if let Some(l) = lang {
        if let Some(alias) = cli::resolve_by_lang(l) {
            return vec![alias.to_string()];
        }
    }
    tr.to_vec()
}
