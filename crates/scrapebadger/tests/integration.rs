//! Live integration tests against the real ScrapeBadger API.
//!
//! These are **ignored by default** because they make real network calls and
//! consume API credits. Run them explicitly with a key set:
//!
//! ```bash
//! export SCRAPEBADGER_API_KEY=sb_live_xxx
//! cargo test -p scrapebadger --test integration -- --ignored --nocapture
//! ```
//!
//! Without `SCRAPEBADGER_API_KEY` each test prints a notice and returns early,
//! so `--ignored` runs stay green on machines with no key configured.

use futures_util::StreamExt;
use scrapebadger::ScrapeBadger;

/// Build a client from the environment, or `None` (with a printed notice) when
/// no key is configured so `--ignored` runs don't hard-fail.
fn client() -> Option<ScrapeBadger> {
    match ScrapeBadger::from_env() {
        Ok(c) => Some(c),
        Err(_) => {
            eprintln!("SCRAPEBADGER_API_KEY not set — skipping live test");
            None
        }
    }
}

#[tokio::test]
#[ignore = "live API call; needs SCRAPEBADGER_API_KEY"]
async fn account_info() {
    let Some(client) = client() else { return };
    let me = client
        .account()
        .get_account_info(Default::default())
        .await
        .expect("get_account_info");
    println!("account: {me:?}");
}

#[tokio::test]
#[ignore = "live API call; needs SCRAPEBADGER_API_KEY"]
async fn amazon_product() {
    let Some(client) = client() else { return };
    let product = client
        .amazon()
        .get_product("B08N5WRWNW", Default::default())
        .await
        .expect("amazon get_product");
    println!("product: {product:?}");
}

#[tokio::test]
#[ignore = "live API call; needs SCRAPEBADGER_API_KEY"]
async fn reddit_subreddit() {
    let Some(client) = client() else { return };
    let sub = client
        .reddit()
        .get_subreddit("rust", Default::default())
        .await
        .expect("reddit get_subreddit");
    println!("subreddit: {sub}");
}

#[tokio::test]
#[ignore = "live API call; needs SCRAPEBADGER_API_KEY"]
async fn twitter_search() {
    let Some(client) = client() else { return };
    let page = client
        .twitter()
        .advanced_search_tweets(scrapebadger::twitter::AdvancedSearchTweetsParams {
            query: Some("rust lang".into()),
            ..Default::default()
        })
        .await
        .expect("twitter advanced_search_tweets");
    println!("tweets: {}", page.data.unwrap_or_default().len());
}

#[tokio::test]
#[ignore = "live API call; needs SCRAPEBADGER_API_KEY"]
async fn twitter_search_stream_first_page() {
    let Some(client) = client() else { return };
    // Exercise the cursor_stream pagination adapter: pull up to 5 items.
    let stream = client.twitter().advanced_search_tweets_stream(
        scrapebadger::twitter::AdvancedSearchTweetsParams {
            query: Some("rust lang".into()),
            ..Default::default()
        },
    );
    futures_util::pin_mut!(stream);
    let mut count = 0;
    while let Some(item) = stream.next().await {
        item.expect("stream item");
        count += 1;
        if count >= 5 {
            break;
        }
    }
    println!("streamed {count} tweets");
}
