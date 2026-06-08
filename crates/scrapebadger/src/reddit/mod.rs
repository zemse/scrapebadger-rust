//! Reddit posts, comments, subreddits, users, and wiki content.
//!
//! The upstream OpenAPI spec does not type its 200 bodies, so the response
//! models in [`models`] were reverse-engineered from live API responses (see
//! `scripts/collect_reddit_samples.py`). The generated [`Reddit`] methods return
//! those typed structs; each tolerates missing/null/unknown fields.
//!
//! See <https://docs.scrapebadger.com/reddit/overview>.

mod generated;
pub use generated::*;

pub mod models;
pub use models::*;

/// Hand-written `*_stream` pagination adapters (methods on [`Reddit`]).
mod pagination;
