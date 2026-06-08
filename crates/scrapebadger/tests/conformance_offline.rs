//! Offline type-conformance: deserialize committed sanitized fixtures into the
//! typed response models. Runs in CI with no API key.
//!
//! Fixtures in `tests/fixtures/` are real responses run through a type-preserving
//! sanitizer (`scripts/make_fixtures.py`): content is scrubbed but every field
//! keeps its exact JSON shape, so this guards against regressions in the typed
//! models / codegen for the response shapes the live sweep validated. The live
//! `examples/conformance.rs` complements this with full-fidelity checks.

use std::path::PathBuf;

use scrapebadger::{account, amazon, reddit, twitter, vinted};

fn de<T: serde::de::DeserializeOwned>(name: &str) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(format!("{name}.json"));
    let raw = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
    if let Err(e) = serde_json::from_str::<T>(&raw) {
        panic!("fixture {name} does not match its typed model: {e}");
    }
}

#[test]
fn reddit_fixtures_match_models() {
    de::<reddit::PostsResponse>("reddit_subreddit_posts");
    de::<reddit::SubredditResponse>("reddit_subreddit");
    de::<reddit::SubredditsResponse>("reddit_subreddits_new");
    de::<reddit::SubredditRulesResponse>("reddit_subreddit_rules");
    de::<reddit::UserResponse>("reddit_user");
    de::<reddit::UserCommentsResponse>("reddit_user_comments");
    de::<reddit::UserModeratedResponse>("reddit_user_moderated");
    de::<reddit::UserTrophiesResponse>("reddit_user_trophies");
    de::<reddit::WikiPagesResponse>("reddit_subreddit_wiki_pages");
    de::<reddit::WikiPageResponse>("reddit_subreddit_wiki_page");
    de::<reddit::PostResponse>("reddit_post");
    de::<reddit::PostCommentsResponse>("reddit_post_comments");
    de::<reddit::PostDuplicatesResponse>("reddit_post_duplicates");
    de::<reddit::UsersResponse>("reddit_search_users");
}

#[test]
fn other_platform_fixtures_match_models() {
    de::<account::GetAccountInfoResponse>("account_me");
    de::<amazon::SearchProductsResponse>("amazon_search");
    de::<amazon::ListMarketsResponse>("amazon_markets");
    de::<vinted::SearchItemsResponse>("vinted_search_items");
    de::<vinted::ListMarketsResponse>("vinted_markets");
    de::<twitter::TweetsResponse>("twitter_advanced_search");
}
