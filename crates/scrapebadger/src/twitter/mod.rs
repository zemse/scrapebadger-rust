//! Twitter/X tweets, users, communities, lists, spaces, geo, trends, and
//! real-time Stream Monitors / Filter Rules.
//!
//! See <https://docs.scrapebadger.com/api-reference/introduction> and
//! <https://docs.scrapebadger.com/twitter-streams/overview>.

mod generated;
pub use generated::*;

#[cfg(feature = "stream")]
#[cfg_attr(docsrs, doc(cfg(feature = "stream")))]
pub mod stream;
