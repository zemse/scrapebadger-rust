//! # ScrapeBadger
//!
//! Async Rust SDK for the [ScrapeBadger](https://scrapebadger.com) web-scraping
//! API. It mirrors the official Python/Node SDKs: one [`ScrapeBadger`] client
//! with a namespace per platform.
//!
//! ```no_run
//! use scrapebadger::ScrapeBadger;
//!
//! # async fn run() -> scrapebadger::Result<()> {
//! // Reads SCRAPEBADGER_API_KEY from the environment.
//! let client = ScrapeBadger::from_env()?;
//!
//! let account = client.account().get_account_info(Default::default()).await?;
//! let flights = client
//!     .google()
//!     .flights_search(scrapebadger::google::FlightsSearchParams {
//!         departure_id: Some("DEL".into()),
//!         arrival_id: Some("BOM".into()),
//!         outbound_date: Some("2026-07-01".into()),
//!         trip_type: Some("one_way".into()), // round_trip (default) also needs return_date
//!         ..Default::default()
//!     })
//!     .await?;
//! # let _ = (account, flights);
//! # Ok(())
//! # }
//! ```
//!
//! Most endpoint inputs are plain structs implementing [`Default`]; set the
//! fields you need and spread the rest with `..Default::default()`. Required
//! parameters are documented per method.

#![cfg_attr(docsrs, feature(doc_cfg))]

use std::time::Duration;

pub mod core;

// Platform namespaces. The `generated` submodule of each is emitted by
// `cargo run -p xtask -- gen` from the vendored OpenAPI specs in `specs/`.
pub mod account;
pub mod amazon;
pub mod ebay;
pub mod google;
pub mod reddit;
pub mod tiktok;
pub mod twitter;
pub mod vinted;
pub mod web;
pub mod youtube;

/// Full `scrapebadger` **CLI** command reference — every endpoint as a nested
/// subcommand. Generated from `specs/*.json` (the same source as this SDK), so
/// it never drifts. This module has no code; it exists to render the CLI tree
/// on docs.rs alongside the library API.
#[doc = include_str!("../docs/CLI.md")]
pub mod cli_reference {}

/// Spec-generated CLI command tree (binary implementation detail).
#[cfg(feature = "cli")]
#[doc(hidden)]
pub mod cli;

pub use crate::core::{Config, Error, Result, DEFAULT_BASE_URL};

use crate::core::{resolve_api_key, Client, Config as CoreConfig};

// Re-export the top-level platform handle types for convenience.
pub use account::Account;
pub use amazon::Amazon;
pub use ebay::Ebay;
pub use google::Google;
pub use reddit::Reddit;
pub use tiktok::Tiktok;
pub use twitter::Twitter;
pub use vinted::Vinted;
pub use web::Web;
pub use youtube::Youtube;

/// The ScrapeBadger API client. Cheap to clone (reference-counted inside).
#[derive(Clone)]
pub struct ScrapeBadger {
    client: Client,
}

impl ScrapeBadger {
    /// Build a client with an explicit API key and otherwise-default settings.
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        Self::builder().api_key(api_key).build()
    }

    /// Build a client reading the API key from `SCRAPEBADGER_API_KEY`.
    pub fn from_env() -> Result<Self> {
        Self::builder().build()
    }

    /// Start configuring a client (api key, base URL, timeout, retries).
    pub fn builder() -> ScrapeBadgerBuilder {
        ScrapeBadgerBuilder::default()
    }

    /// The underlying transport client (also used by streaming APIs).
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Account, credits, and plan information.
    pub fn account(&self) -> Account {
        Account::new(self.client.clone())
    }

    /// Amazon product, review, offer, and seller data.
    pub fn amazon(&self) -> Amazon {
        Amazon::new(self.client.clone())
    }

    /// eBay search, item details, reviews, sellers, and catalog reference data.
    pub fn ebay(&self) -> Ebay {
        Ebay::new(self.client.clone())
    }

    /// Google product APIs (Search, Maps, Flights, News, Trends, and more).
    pub fn google(&self) -> Google {
        Google::new(self.client.clone())
    }

    /// Reddit posts, comments, subreddits, and users.
    pub fn reddit(&self) -> Reddit {
        Reddit::new(self.client.clone())
    }

    /// TikTok videos, users, hashtags, music, search, trending, and ads.
    pub fn tiktok(&self) -> Tiktok {
        Tiktok::new(self.client.clone())
    }

    /// Twitter/X tweets, users, lists, spaces, trends, and real-time streams.
    pub fn twitter(&self) -> Twitter {
        Twitter::new(self.client.clone())
    }

    /// Vinted listings, users, and catalog reference data.
    pub fn vinted(&self) -> Vinted {
        Vinted::new(self.client.clone())
    }

    /// General-purpose website scraping and anti-bot detection.
    pub fn web(&self) -> Web {
        Web::new(self.client.clone())
    }

    /// YouTube videos, channels, playlists, comments, transcripts, and trending.
    pub fn youtube(&self) -> Youtube {
        Youtube::new(self.client.clone())
    }
}

/// Builder for [`ScrapeBadger`].
#[derive(Default)]
pub struct ScrapeBadgerBuilder {
    api_key: Option<String>,
    base_url: Option<String>,
    timeout: Option<Duration>,
    max_retries: Option<u32>,
    user_agent: Option<String>,
}

impl ScrapeBadgerBuilder {
    /// Set the API key explicitly (otherwise read from the environment).
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Override the API base URL (default <https://scrapebadger.com>).
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Per-request timeout (default 300s).
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Maximum retries for 502/503/504 and transient transport errors (default 10).
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    /// Override the `User-Agent` header.
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Resolve the API key and construct the client.
    pub fn build(self) -> Result<ScrapeBadger> {
        let api_key = resolve_api_key(self.api_key)?;
        let mut config = CoreConfig::new(api_key);
        if let Some(v) = self.base_url {
            config.base_url = v;
        }
        if let Some(v) = self.timeout {
            config.timeout = v;
        }
        if let Some(v) = self.max_retries {
            config.max_retries = v;
        }
        if let Some(v) = self.user_agent {
            config.user_agent = v;
        }
        Ok(ScrapeBadger {
            client: Client::from_config(config)?,
        })
    }
}
