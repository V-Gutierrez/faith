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
    /// Manage Faith configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Debug, Subcommand)]
enum ConfigAction {
    /// Show current configuration
    Get {
        /// Config key to get (e.g., "lang", "translation")
        key: Option<String>,
    },
    /// Set configuration value
    Set {
        /// Config key (e.g., "lang", "translation", "format")
        key: String,
        /// Value to set
        value: String,
    },
    /// Show config file path
    Path,
    /// Reset to defaults (deletes config.toml)
    Reset {
        #[arg(long)]
        confirm: bool,
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

    // Load config for translation resolution
    let config = cli::config::load_config().unwrap_or_default();

    match cli.command {
        Cmd::Get {
            reference,
            tr,
            lang,
            format,
        } => {
            let store = Store::open(&path)?;
            let trs = resolve_lang_or_tr_with_config(&tr, lang.as_deref(), &config);
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
            // Resolve translation using config if neither --tr nor --lang provided
            let resolved = if tr.is_none() && lang.is_none() {
                let trs = resolve_lang_or_tr_with_config(&[], None, &config);
                Some(trs[0].clone())
            } else {
                tr
            };
            cli::random::run(
                &store,
                resolved.as_deref(),
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
            let trs = resolve_lang_or_tr_with_config(&tr, lang.as_deref(), &config);
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
            // Resolve translation with config support
            let fallback_tr = if tr.is_none() && lang.is_none() {
                let trs = resolve_lang_or_tr_with_config(&[], None, &config);
                Some(trs[0].clone())
            } else {
                None
            };
            let resolved_tr = match (&tr, &lang, &fallback_tr) {
                (Some(t), _, _) => Some(t.as_str()),
                (None, Some(l), _) => cli::resolve_by_lang(l),
                (None, None, Some(f)) => Some(f.as_str()),
                _ => None,
            };
            cli::search::run(&store, &query, resolved_tr, limit, format.into(), &mut out)
        }
        Cmd::Config { action } => handle_config(action, &mut out),
    }
}

/// Resolve translation with config support and 7-layer precedence:
/// 1. CLI --tr flag (highest priority)
/// 2. CLI --lang flag
/// 3. FAITH_LANG environment variable
/// 4. Config file translation preference
/// 5. Config file lang preference
/// 6. System locale (LANG env var)
/// 7. Hardcoded default (KJV)
fn resolve_lang_or_tr_with_config(
    cli_tr: &[String],
    cli_lang: Option<&str>,
    config: &cli::config::Config,
) -> Vec<String> {
    // 1. CLI --tr flag (highest priority)
    if !cli_tr.is_empty() {
        return cli_tr.to_vec();
    }

    // 2. CLI --lang flag
    if let Some(l) = cli_lang {
        if let Some(alias) = cli::resolve_by_lang(l) {
            return vec![alias.to_string()];
        }
    }

    // 3. FAITH_LANG environment variable
    if let Ok(env_lang) = std::env::var("FAITH_LANG") {
        if let Some(alias) = cli::resolve_by_lang(&env_lang) {
            return vec![alias.to_string()];
        }
    }

    // 4. Config file translation preference
    if let Some(ref tr) = config.preferences.translation {
        return vec![tr.clone()];
    }

    // 5. Config file lang preference
    if let Some(ref lang) = config.preferences.lang {
        if let Some(alias) = cli::resolve_by_lang(lang) {
            return vec![alias.to_string()];
        }
    }

    // 6. System locale (LANG env var)
    if let Ok(sys_lang) = std::env::var("LANG") {
        // Parse LANG=pt_BR.UTF-8 → "pt"
        if let Some(iso2) = sys_lang.split('_').next() {
            if let Some(alias) = cli::resolve_by_lang(iso2) {
                return vec![alias.to_string()];
            }
        }
    }

    // 7. Hardcoded default (English KJV)
    vec!["KJV".to_string()]
}

/// Handle the `faith config` subcommand.
fn handle_config(action: ConfigAction, out: &mut dyn Write) -> Result<i32, FaithError> {
    let config = cli::config::load_config()?;

    match action {
        ConfigAction::Get { key } => {
            if let Some(k) = key {
                // Show specific key
                let value = match k.as_str() {
                    "lang" => config.preferences.lang.as_deref().unwrap_or("(not set)"),
                    "translation" => config
                        .preferences
                        .translation
                        .as_deref()
                        .unwrap_or("(not set)"),
                    "format" => config.preferences.format.as_deref().unwrap_or("(not set)"),
                    "seed" => {
                        if let Some(s) = config.preferences.seed {
                            writeln!(out, "{}", s).ok();
                            return Ok(0);
                        } else {
                            "(not set)"
                        }
                    }
                    _ => {
                        return Err(FaithError::Io(format!("Unknown config key: {}", k)));
                    }
                };
                writeln!(out, "{}", value).ok();
            } else {
                // Show entire config
                let toml_str = toml::to_string_pretty(&config)
                    .map_err(|e| FaithError::Io(format!("Failed to serialize config: {}", e)))?;
                write!(out, "{}", toml_str).ok();
            }
            Ok(0)
        }
        ConfigAction::Set { key, value } => {
            let mut cfg = config.clone();
            match key.as_str() {
                "lang" => cfg.preferences.lang = Some(value.clone()),
                "translation" => cfg.preferences.translation = Some(value.clone()),
                "format" => cfg.preferences.format = Some(value.clone()),
                "seed" => {
                    let seed = value
                        .parse::<u64>()
                        .map_err(|_| FaithError::Io(format!("Invalid seed value: {}", value)))?;
                    cfg.preferences.seed = Some(seed);
                }
                _ => {
                    return Err(FaithError::Io(format!("Unknown config key: {}", key)));
                }
            }
            cli::config::save_config(&cfg)?;
            writeln!(out, "✓ Set {} = {}", key, value).ok();
            Ok(0)
        }
        ConfigAction::Path => {
            let path = cli::config::config_path()?;
            writeln!(out, "{}", path.display()).ok();
            Ok(0)
        }
        ConfigAction::Reset { confirm } => {
            if !confirm {
                writeln!(out, "⚠️  Use --confirm to reset configuration").ok();
                return Ok(1);
            }
            let path = cli::config::config_path()?;
            if path.exists() {
                std::fs::remove_file(&path)
                    .map_err(|e| FaithError::Io(format!("Failed to delete config: {}", e)))?;
                writeln!(out, "✓ Configuration reset (deleted {})", path.display()).ok();
            } else {
                writeln!(out, "No configuration file to reset").ok();
            }
            Ok(0)
        }
    }
}
