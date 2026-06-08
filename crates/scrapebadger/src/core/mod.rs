//! Transport core shared by every platform namespace: configuration, the
//! reqwest-backed [`Client`], the typed [`Error`] model, query assembly, and
//! pagination helpers.

mod client;
mod config;
mod error;
pub(crate) mod flex;
mod request;

pub mod pagination;

pub use client::Client;
pub use config::{Config, API_KEY_ENV, DEFAULT_BASE_URL};
pub use error::{Error, Result};
pub use request::QueryParams;

#[cfg(feature = "stream")]
pub(crate) use client::jitter;
pub(crate) use client::resolve_api_key;

/// Re-export reqwest's `Method` so generated code and callers share one type.
pub use reqwest::Method;
