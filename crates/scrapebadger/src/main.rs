//! `scrapebadger` command-line interface.
//!
//! Reads the API key from `--api-key` or the `SCRAPEBADGER_API_KEY` env var and
//! prints pretty-printed JSON to stdout. The `raw` subcommand can reach every
//! one of the API's endpoints; the named subcommands are typed conveniences
//! for the most common ones.

use anyhow::{Context, Result};
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

    let mut builder = ScrapeBadger::builder();
    if let Some(key) = cli.api_key {
        builder = builder.api_key(key);
    }
    let client = builder
        .build()
        .context("failed to build client (set SCRAPEBADGER_API_KEY or pass --api-key)")?;

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
    };

    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}
