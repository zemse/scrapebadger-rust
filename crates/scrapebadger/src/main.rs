//! `scrapebadger` command-line interface.
//!
//! Resolves the API key from (in order) `--api-key`, the `SCRAPEBADGER_API_KEY`
//! env var, then the global config file (`scrapebadger config set-key …`), and
//! prints pretty-printed JSON to stdout. The `raw` subcommand can reach every
//! one of the API's endpoints; the named subcommands are typed conveniences
//! for the most common ones.

use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use clap::{Args, Parser, Subcommand};
use scrapebadger::core::Method;
use scrapebadger::ScrapeBadger;

#[derive(Parser)]
#[command(
    name = "scrapebadger",
    version,
    about = "CLI for the ScrapeBadger web-scraping API",
    propagate_version = true
)]
struct Cli {
    /// API key (overrides SCRAPEBADGER_API_KEY).
    #[arg(long, global = true, env = "SCRAPEBADGER_API_KEY", hide_env_values = true)]
    api_key: Option<String>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Show account info, credit balances, and plan tier.
    Account,

    /// Scrape a URL and print the extracted content.
    Scrape(ScrapeArgs),

    /// Search Google Flights.
    Flights(FlightsArgs),

    /// Search Google web results.
    GoogleSearch { query: String },

    /// Search Amazon products.
    AmazonSearch { query: String },

    /// Fetch an Amazon product by ASIN.
    AmazonProduct { asin: String },

    /// Advanced Twitter/X tweet search.
    TwitterSearch { query: String },

    /// Fetch a Reddit post by ID.
    RedditPost { post_id: String },

    /// Low-level request to any endpoint (covers the full API surface).
    Raw(RawArgs),

    /// Manage the global config (stored API key).
    Config(ConfigArgs),
}

#[derive(Args)]
struct ConfigArgs {
    #[command(subcommand)]
    command: ConfigCmd,
}

#[derive(Subcommand)]
enum ConfigCmd {
    /// Store an API key in the global config file.
    SetKey {
        /// The API key (e.g. sb_live_…). Read from stdin if omitted.
        key: Option<String>,
    },
    /// Print the config file path.
    Path,
    /// Show the current config (API key masked).
    Show,
}

#[derive(Args)]
struct ScrapeArgs {
    /// URL to scrape.
    url: String,
    /// Output format: html, markdown, or text.
    #[arg(long, default_value = "markdown")]
    format: String,
    /// Render JavaScript before extracting.
    #[arg(long)]
    render_js: bool,
}

#[derive(Args)]
struct FlightsArgs {
    /// Departure airport/city code (e.g. DEL).
    #[arg(long)]
    from: String,
    /// Arrival airport/city code (e.g. BOM).
    #[arg(long)]
    to: String,
    /// Outbound date (YYYY-MM-DD).
    #[arg(long)]
    date: String,
    /// Return date (YYYY-MM-DD) for round trips.
    #[arg(long)]
    return_date: Option<String>,
}

#[derive(Args)]
struct RawArgs {
    /// HTTP method.
    #[arg(long, default_value = "GET")]
    method: String,
    /// Request path, e.g. /v1/amazon/products/B08N5WRWNW (or /api/v1/... for Google).
    path: String,
    /// Repeatable query parameter, key=value.
    #[arg(long = "query", short = 'q', value_name = "KEY=VALUE")]
    query: Vec<String>,
    /// JSON request body (for POST/PATCH).
    #[arg(long, short = 'd')]
    body: Option<String>,
}

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("error: {err:#}");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();

    // Config management runs without needing a resolved key.
    if let Command::Config(args) = &cli.command {
        return handle_config(&args.command);
    }

    // Resolution order: --api-key / env (both surface via cli.api_key) > config file.
    let key = cli.api_key.clone().or_else(load_config_key);
    let mut builder = ScrapeBadger::builder();
    if let Some(key) = key {
        builder = builder.api_key(key);
    }
    let client = builder.build().context(
        "no API key found: pass --api-key, set SCRAPEBADGER_API_KEY, or run `scrapebadger config set-key <KEY>`",
    )?;

    let value: serde_json::Value = match cli.command {
        Command::Account => {
            serde_json::to_value(client.account().get_account_info(Default::default()).await?)?
        }
        Command::Scrape(a) => {
            let params = scrapebadger::web::ScrapeUrlParams {
                url: Some(a.url),
                format: Some(a.format),
                render_js: Some(a.render_js),
                ..Default::default()
            };
            serde_json::to_value(client.web().scrape_url(params).await?)?
        }
        Command::Flights(a) => {
            let params = scrapebadger::google::FlightsSearchParams {
                departure_id: Some(a.from),
                arrival_id: Some(a.to),
                outbound_date: Some(a.date),
                return_date: a.return_date,
                ..Default::default()
            };
            serde_json::to_value(client.google().flights_search(params).await?)?
        }
        Command::GoogleSearch { query } => {
            let params = scrapebadger::google::SearchParams {
                q: Some(query),
                ..Default::default()
            };
            serde_json::to_value(client.google().search(params).await?)?
        }
        Command::AmazonSearch { query } => {
            let params = scrapebadger::amazon::SearchProductsParams {
                query: Some(query),
                ..Default::default()
            };
            serde_json::to_value(client.amazon().search_products(params).await?)?
        }
        Command::AmazonProduct { asin } => {
            serde_json::to_value(client.amazon().get_product(asin, Default::default()).await?)?
        }
        Command::TwitterSearch { query } => {
            let params = scrapebadger::twitter::AdvancedSearchTweetsParams {
                query: Some(query),
                ..Default::default()
            };
            serde_json::to_value(client.twitter().advanced_search_tweets(params).await?)?
        }
        Command::RedditPost { post_id } => {
            serde_json::to_value(client.reddit().get_post(post_id, Default::default()).await?)?
        }
        Command::Raw(a) => {
            let method = a
                .method
                .parse::<Method>()
                .with_context(|| format!("invalid HTTP method: {}", a.method))?;
            let query: Vec<(String, String)> = a
                .query
                .iter()
                .map(|kv| {
                    let (k, v) = kv
                        .split_once('=')
                        .with_context(|| format!("query must be key=value: {kv}"))?;
                    Ok::<_, anyhow::Error>((k.to_string(), v.to_string()))
                })
                .collect::<Result<_>>()?;
            let body = match a.body {
                Some(b) => Some(serde_json::from_str(&b).context("--body is not valid JSON")?),
                None => None,
            };
            client.client().send(method, &a.path, &query, body).await?
        }
        // Handled before the client is built.
        Command::Config(_) => unreachable!("config is handled before client setup"),
    };

    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}

/// Path to the global config file (`$XDG_CONFIG_HOME/scrapebadger/config.json`,
/// falling back to `~/.config/scrapebadger/config.json`).
fn config_path() -> Result<PathBuf> {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))
        .ok_or_else(|| anyhow!("cannot determine config directory (set HOME or XDG_CONFIG_HOME)"))?;
    Ok(base.join("scrapebadger").join("config.json"))
}

/// Load the stored API key from the global config file, if present.
fn load_config_key() -> Option<String> {
    let path = config_path().ok()?;
    let text = fs::read_to_string(path).ok()?;
    let value: serde_json::Value = serde_json::from_str(&text).ok()?;
    value
        .get("api_key")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

/// Persist an API key to the global config file with `0600` permissions (unix).
fn save_config_key(key: &str) -> Result<PathBuf> {
    let path = config_path()?;
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))?;
    }
    // Merge into any existing config rather than clobbering other keys.
    let mut value: serde_json::Value = fs::read_to_string(&path)
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok())
        .unwrap_or_else(|| serde_json::json!({}));
    value["api_key"] = serde_json::Value::String(key.to_string());
    fs::write(&path, serde_json::to_string_pretty(&value)?)
        .with_context(|| format!("writing {}", path.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600))?;
    }
    Ok(path)
}

fn mask(key: &str) -> String {
    match key.len() {
        0 => "(empty)".to_string(),
        1..=12 => "*".repeat(key.len()),
        n => format!("{}…{} ({n} chars)", &key[..8], &key[n - 4..]),
    }
}

fn handle_config(cmd: &ConfigCmd) -> Result<()> {
    match cmd {
        ConfigCmd::SetKey { key } => {
            let key = match key {
                Some(k) => k.trim().to_string(),
                None => {
                    use std::io::Read;
                    let mut buf = String::new();
                    std::io::stdin().read_to_string(&mut buf)?;
                    buf.trim().to_string()
                }
            };
            if key.is_empty() {
                return Err(anyhow!("no key provided"));
            }
            let path = save_config_key(&key)?;
            println!("Stored API key ({}) in {}", mask(&key), path.display());
        }
        ConfigCmd::Path => println!("{}", config_path()?.display()),
        ConfigCmd::Show => match load_config_key() {
            Some(k) => println!("api_key = {}", mask(&k)),
            None => println!("no API key stored (run `scrapebadger config set-key <KEY>`)"),
        },
    }
    Ok(())
}
