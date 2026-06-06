//! Reddit posts, comments, subreddits, users, and wiki content.
//!
//! Reddit responses are returned as [`serde_json::Value`] because the upstream
//! OpenAPI spec does not type its 200 bodies.
//!
//! See <https://docs.scrapebadger.com/reddit/overview>.

mod generated;
pub use generated::*;
