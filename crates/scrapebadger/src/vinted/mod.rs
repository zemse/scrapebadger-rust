//! Vinted listings, user profiles, and catalog reference data across 26 markets.
//!
//! See <https://docs.scrapebadger.com/vinted/overview>.

mod generated;
pub use generated::*;

/// Hand-written `*_stream` pagination adapters (methods on [`Vinted`]).
mod pagination;
