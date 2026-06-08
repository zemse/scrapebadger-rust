//! Amazon product, review, offer, and seller data across 20 marketplaces.
//!
//! See <https://docs.scrapebadger.com/amazon/overview>.

mod generated;
pub use generated::*;

/// Hand-written `*_stream` pagination adapters (methods on [`Amazon`]).
mod pagination;
