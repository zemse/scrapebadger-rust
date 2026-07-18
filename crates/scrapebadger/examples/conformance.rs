//! Live type-conformance sweep for the SDK's response models.
//!
//! Calls read-only endpoints against the real API and checks each response
//! deserializes into its typed model. The SDK only returns `Ok` when the body
//! deserialized into the method's return type, so:
//!   * **✓ pass**     — response matched the typed model,
//!   * **✗ TYPE**     — `Error::Decode`: the typed model is WRONG (the signal we want),
//!   * **· api**      — other error (bad/stale id, rate limit, 4xx/5xx): not a type problem,
//!   * **— skip**     — not exercised here (needs a hard-to-source id or mutates state).
//!
//! IDs for detail endpoints are chained from live list/search responses where
//! possible, so the sweep stays valid over time without hardcoded ids.
//!
//! Run:
//! ```bash
//! SCRAPEBADGER_API_KEY=sb_live_xxx cargo run -p scrapebadger --example conformance
//! ```
//! Exit code is non-zero if any endpoint produced a TYPE failure.

// Every params struct is built with a uniform `..d()` spread for consistency and
// so it keeps compiling if the API gains a field; some structs are fully covered.
#![allow(clippy::needless_update)]

use std::future::Future;
use std::time::Duration;

use scrapebadger::{account, amazon, ebay, google, reddit, tiktok, twitter, vinted, web, youtube};
use scrapebadger::{
    depop, idealista, immobiliare, leboncoin, linkedin, loopnet, realtor, redfin, zillow,
};
use scrapebadger::{Error, Result, ScrapeBadger};

#[derive(Default)]
struct Report {
    pass: Vec<String>,
    type_fail: Vec<(String, String)>,
    api_err: Vec<(String, String)>,
    skip: Vec<(String, String)>,
}

/// `Default::default()` shorthand for params structs.
fn d<T: Default>() -> T {
    T::default()
}

/// Pull a scalar id out of an untyped (`serde_json::Value`) response for
/// chaining detail calls. `path` navigates objects and arrays: a segment like
/// `"results"` indexes a field, `"0"` indexes an array element. Numbers are
/// stringified so numeric ids (zpid, list_id, …) chain the same as string ones.
fn pick(v: &Option<serde_json::Value>, path: &[&str]) -> Option<String> {
    let mut cur = v.as_ref()?;
    for seg in path {
        cur = match seg.parse::<usize>() {
            Ok(i) => cur.as_array()?.get(i)?,
            Err(_) => cur.get(seg)?,
        };
    }
    match cur {
        serde_json::Value::String(s) if !s.is_empty() => Some(s.clone()),
        serde_json::Value::Number(n) => Some(n.to_string()),
        _ => None,
    }
}

impl Report {
    /// Await an endpoint call, classify the outcome, pace the next call, and
    /// return the success value (for id chaining).
    async fn run<T>(&mut self, label: &str, fut: impl Future<Output = Result<T>>) -> Option<T> {
        // Optional `CONFORMANCE_FILTER` (comma-separated label prefixes) lets you
        // exercise a subset — e.g. `CONFORMANCE_FILTER=google` to spend credits
        // on just the Google endpoints. Unset = run everything.
        if let Ok(f) = std::env::var("CONFORMANCE_FILTER") {
            if !f.split(',').any(|p| label.starts_with(p.trim())) {
                return None;
            }
        }
        let res = fut.await;
        match &res {
            Ok(_) => {
                println!("✓ pass  {label}");
                self.pass.push(label.to_string());
            }
            Err(Error::Decode(m)) => {
                println!("✗ TYPE  {label}: {m}");
                self.type_fail.push((label.to_string(), m.clone()));
            }
            Err(e) => {
                println!("· api   {label}: {e}");
                self.api_err.push((label.to_string(), e.to_string()));
            }
        }
        // The client already retries 429; a small gap keeps us well-behaved.
        tokio::time::sleep(Duration::from_millis(250)).await;
        res.ok()
    }

    fn skip(&mut self, label: &str, why: &str) {
        println!("— skip  {label}: {why}");
        self.skip.push((label.to_string(), why.to_string()));
    }
}

#[tokio::main]
async fn main() {
    let c = match ScrapeBadger::from_env() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("set SCRAPEBADGER_API_KEY: {e}");
            std::process::exit(2);
        }
    };
    let mut r = Report::default();

    // ---- account ----
    r.run(
        "account.get_account_info",
        c.account()
            .get_account_info(d::<account::GetAccountInfoParams>()),
    )
    .await;

    // ---- amazon ----
    r.run("amazon.list_markets", c.amazon().list_markets(d()))
        .await;
    r.run("amazon.list_categories", c.amazon().list_categories(d()))
        .await;
    r.run("amazon.get_deals", c.amazon().get_deals(d())).await;
    r.run("amazon.get_bestsellers", c.amazon().get_bestsellers(d()))
        .await;
    r.run("amazon.get_new_releases", c.amazon().get_new_releases(d()))
        .await;
    r.run(
        "amazon.autocomplete",
        c.amazon().autocomplete(amazon::AutocompleteParams {
            query: Some("laptop".into()),
            ..d()
        }),
    )
    .await;
    r.run(
        "amazon.search_products",
        c.amazon().search_products(amazon::SearchProductsParams {
            query: Some("laptop".into()),
            ..d()
        }),
    )
    .await;
    const ASIN: &str = "B08N5WRWNW";
    r.run("amazon.get_product", c.amazon().get_product(ASIN, d()))
        .await;
    r.run("amazon.get_offers", c.amazon().get_offers(ASIN, d()))
        .await;
    r.run("amazon.get_reviews", c.amazon().get_reviews(ASIN, d()))
        .await;
    r.skip("amazon.browse_category", "needs a category `node` id");
    r.skip(
        "amazon.get_seller[_feedback|_products]",
        "needs a seller id",
    );

    // ---- google ----
    // Helper closures to build single-`q` param structs concisely.
    let gq = |q: &str| google::SearchParams {
        q: Some(q.into()),
        ..d()
    };
    r.run("google.search", c.google().search(gq("rust lang")))
        .await;
    r.run(
        "google.ai_mode_search",
        c.google().ai_mode_search(google::AiModeSearchParams {
            q: Some("what is rust".into()),
            ..d()
        }),
    )
    .await;
    r.run(
        "google.ai_overview",
        c.google().ai_overview(google::AiOverviewParams {
            q: Some("what is rust".into()),
            ..d()
        }),
    )
    .await;
    r.run(
        "google.autocomplete",
        c.google().autocomplete(google::AutocompleteParams {
            q: Some("rust".into()),
            ..d()
        }),
    )
    .await;
    r.run(
        "google.images_search",
        c.google().images_search(google::ImagesSearchParams {
            q: Some("rust crab".into()),
            ..d()
        }),
    )
    .await;
    r.run(
        "google.videos_search",
        c.google().videos_search(google::VideosSearchParams {
            q: Some("rust tutorial".into()),
            ..d()
        }),
    )
    .await;
    r.run(
        "google.shorts_search",
        c.google().shorts_search(google::ShortsSearchParams {
            q: Some("rust".into()),
            ..d()
        }),
    )
    .await;
    r.run(
        "google.news_search",
        c.google().news_search(google::NewsSearchParams {
            q: Some("rust".into()),
            ..d()
        }),
    )
    .await;
    r.run(
        "google.news_topics",
        c.google().news_topics(google::NewsTopicsParams {
            topic: Some("TECHNOLOGY".into()),
            ..d()
        }),
    )
    .await;
    r.run("google.news_trending", c.google().news_trending(d()))
        .await;
    r.run(
        "google.maps_search",
        c.google().maps_search(google::MapsSearchParams {
            q: Some("coffee in San Francisco".into()),
            ..d()
        }),
    )
    .await;
    r.run(
        "google.jobs_search",
        c.google().jobs_search(google::JobsSearchParams {
            q: Some("rust developer".into()),
            ..d()
        }),
    )
    .await;
    r.run(
        "google.shopping_search",
        c.google().shopping_search(google::ShoppingSearchParams {
            q: Some("laptop".into()),
            ..d()
        }),
    )
    .await;
    r.run(
        "google.patents_search",
        c.google().patents_search(google::PatentsSearchParams {
            q: Some("battery".into()),
            ..d()
        }),
    )
    .await;
    r.run(
        "google.scholar_search",
        c.google().scholar_search(google::ScholarSearchParams {
            q: Some("graphene".into()),
            ..d()
        }),
    )
    .await;
    r.run(
        "google.scholar_cite",
        c.google().scholar_cite(google::ScholarCiteParams {
            q: Some("graphene".into()),
            ..d()
        }),
    )
    .await;
    r.run(
        "google.scholar_profiles",
        c.google().scholar_profiles(google::ScholarProfilesParams {
            mauthors: Some("einstein".into()),
            ..d()
        }),
    )
    .await;
    r.run(
        "google.finance_quote",
        c.google().finance_quote(google::FinanceQuoteParams {
            q: Some("AAPL:NASDAQ".into()),
            ..d()
        }),
    )
    .await;
    r.run(
        "google.trends_interest",
        c.google().trends_interest(google::TrendsInterestParams {
            q: Some("rust".into()),
            ..d()
        }),
    )
    .await;
    r.run(
        "google.trends_regions",
        c.google().trends_regions(google::TrendsRegionsParams {
            q: Some("rust".into()),
            ..d()
        }),
    )
    .await;
    r.run(
        "google.trends_related",
        c.google().trends_related(google::TrendsRelatedParams {
            q: Some("rust".into()),
            ..d()
        }),
    )
    .await;
    r.run(
        "google.trends_autocomplete",
        c.google()
            .trends_autocomplete(google::TrendsAutocompleteParams {
                q: Some("rust".into()),
                ..d()
            }),
    )
    .await;
    r.run("google.trends_trending", c.google().trends_trending(d()))
        .await;
    r.run(
        "google.flights_search",
        c.google().flights_search(google::FlightsSearchParams {
            departure_id: Some("JFK".into()),
            arrival_id: Some("LAX".into()),
            outbound_date: Some("2026-07-15".into()),
            trip_type: Some("one_way".into()),
            ..d()
        }),
    )
    .await;
    r.run(
        "google.hotels_search",
        c.google().hotels_search(google::HotelsSearchParams {
            q: Some("hotels in Paris".into()),
            check_in: Some("2026-07-15".into()),
            check_out: Some("2026-07-18".into()),
            ..d()
        }),
    )
    .await;
    // Detail endpoints need an id from a prior call (data_id / property_token /
    // author_id / patent_id / product_id) or an image URL — not chained here.
    r.skip(
        "google.maps_place/maps_reviews/maps_photos/maps_posts",
        "need a maps data_id from a prior maps_search",
    );
    r.skip("google.hotel_details", "needs a property_token");
    r.skip("google.products_detail", "needs a product_id + q");
    r.skip("google.patent_detail", "needs a patent_id");
    r.skip(
        "google.scholar_author/scholar_author_citation",
        "need an author_id",
    );
    r.skip("google.lens_search", "needs an image url");

    // ---- reddit (chained) ----
    r.run(
        "reddit.get_new_subreddits",
        c.reddit().get_new_subreddits(d()),
    )
    .await;
    r.run(
        "reddit.get_popular_subreddits",
        c.reddit().get_popular_subreddits(d()),
    )
    .await;
    r.run(
        "reddit.get_trending_posts",
        c.reddit().get_trending_posts(d()),
    )
    .await;
    r.run(
        "reddit.get_subreddit",
        c.reddit().get_subreddit("rust", d()),
    )
    .await;
    r.run(
        "reddit.get_subreddit_rules",
        c.reddit().get_subreddit_rules("rust", d()),
    )
    .await;
    r.run("reddit.get_user", c.reddit().get_user("spez", d()))
        .await;
    r.run(
        "reddit.get_user_posts",
        c.reddit().get_user_posts("spez", d()),
    )
    .await;
    r.run(
        "reddit.get_user_comments",
        c.reddit().get_user_comments("spez", d()),
    )
    .await;
    r.run(
        "reddit.get_user_moderated",
        c.reddit().get_user_moderated("spez", d()),
    )
    .await;
    r.run(
        "reddit.get_user_trophies",
        c.reddit().get_user_trophies("spez", d()),
    )
    .await;
    r.run(
        "reddit.get_domain_posts",
        c.reddit().get_domain_posts("github.com", d()),
    )
    .await;
    r.run(
        "reddit.search_posts",
        c.reddit().search_posts(reddit::SearchPostsParams {
            q: Some("rust".into()),
            ..d()
        }),
    )
    .await;
    r.run(
        "reddit.search_subreddits",
        c.reddit()
            .search_subreddits(reddit::SearchSubredditsParams {
                q: Some("rust".into()),
                ..d()
            }),
    )
    .await;
    r.run(
        "reddit.search_users",
        c.reddit().search_users(reddit::SearchUsersParams {
            q: Some("rust".into()),
            ..d()
        }),
    )
    .await;
    let wiki = r
        .run(
            "reddit.get_wiki_pages",
            c.reddit().get_wiki_pages("rust", d()),
        )
        .await;
    if let Some(page) = wiki.and_then(|w| w.pages.into_iter().next()) {
        r.run(
            "reddit.get_wiki_page",
            c.reddit().get_wiki_page("rust", &page, d()),
        )
        .await;
    } else {
        r.skip("reddit.get_wiki_page", "no wiki pages listed");
    }
    let posts = r
        .run(
            "reddit.get_subreddit_posts",
            c.reddit().get_subreddit_posts("rust", d()),
        )
        .await;
    let post_id = posts
        .and_then(|p| p.posts.into_iter().next())
        .and_then(|p| p.id);
    if let Some(pid) = &post_id {
        r.run("reddit.get_post", c.reddit().get_post(pid, d()))
            .await;
        r.run(
            "reddit.get_post_comments",
            c.reddit().get_post_comments(pid, d()),
        )
        .await;
        r.run(
            "reddit.get_post_duplicates",
            c.reddit().get_post_duplicates(pid, d()),
        )
        .await;
    } else {
        r.skip(
            "reddit.get_post[_comments|_duplicates]",
            "no post id from listing",
        );
    }

    // ---- twitter (chained) ----
    let search = r
        .run(
            "twitter.advanced_search_tweets",
            c.twitter()
                .advanced_search_tweets(twitter::AdvancedSearchTweetsParams {
                    query: Some("rust lang".into()),
                    ..d()
                }),
        )
        .await;
    let tweet = search
        .and_then(|s| s.data)
        .and_then(|v| v.into_iter().next());
    let tweet_id = tweet.as_ref().and_then(|t| t.id.clone());
    let user_id = tweet.as_ref().and_then(|t| t.user_id.clone());
    let username = tweet
        .as_ref()
        .and_then(|t| t.username.clone())
        .unwrap_or_else(|| "elonmusk".into());

    r.run(
        "twitter.search_users",
        c.twitter().search_users(twitter::SearchUsersParams {
            query: Some("rust".into()),
            ..d()
        }),
    )
    .await;
    r.run(
        "twitter.search_communities",
        c.twitter()
            .search_communities(twitter::SearchCommunitiesParams {
                query: Some("rust".into()),
                ..d()
            }),
    )
    .await;
    r.run("twitter.get_trends", c.twitter().get_trends(d()))
        .await;
    r.run(
        "twitter.get_trends_by_place",
        c.twitter().get_trends_by_place("1", d()),
    )
    .await;
    r.run(
        "twitter.get_users_by_usernames",
        c.twitter()
            .get_users_by_usernames(twitter::GetUsersByUsernamesParams {
                usernames: Some("elonmusk,jack".into()),
                ..d()
            }),
    )
    .await;
    r.run(
        "twitter.get_user_by_username",
        c.twitter().get_user_by_username(&username, d()),
    )
    .await;
    r.run(
        "twitter.get_user_followers",
        c.twitter().get_user_followers(&username, d()),
    )
    .await;
    r.run(
        "twitter.get_user_followings",
        c.twitter().get_user_followings(&username, d()),
    )
    .await;
    r.run(
        "twitter.get_user_latest_tweets",
        c.twitter().get_user_latest_tweets(&username, d()),
    )
    .await;
    r.run(
        "twitter.get_user_mentions",
        c.twitter().get_user_mentions(&username, d()),
    )
    .await;
    if let Some(uid) = &user_id {
        r.run(
            "twitter.get_user_by_id",
            c.twitter().get_user_by_id(uid, d()),
        )
        .await;
        r.run(
            "twitter.get_users_by_ids",
            c.twitter().get_users_by_ids(twitter::GetUsersByIdsParams {
                user_ids: Some(uid.clone()),
                ..d()
            }),
        )
        .await;
        r.run(
            "twitter.get_user_subscriptions",
            c.twitter().get_user_subscriptions(uid, d()),
        )
        .await;
        r.run(
            "twitter.get_user_articles",
            c.twitter().get_user_articles(uid, d()),
        )
        .await;
    } else {
        r.skip(
            "twitter.get_user_by_id/get_users_by_ids/get_user_subscriptions/get_user_articles",
            "no user id from search",
        );
    }
    if let Some(tid) = &tweet_id {
        r.run(
            "twitter.get_tweet_detail",
            c.twitter().get_tweet_detail(tid, d()),
        )
        .await;
        r.run(
            "twitter.get_tweets_by_ids",
            c.twitter()
                .get_tweets_by_ids(twitter::GetTweetsByIdsParams {
                    tweets: Some(tid.clone()),
                    ..d()
                }),
        )
        .await;
        r.run(
            "twitter.get_tweet_replies",
            c.twitter().get_tweet_replies(tid, d()),
        )
        .await;
        r.run(
            "twitter.get_tweet_quotes",
            c.twitter().get_tweet_quotes(tid, d()),
        )
        .await;
        r.run(
            "twitter.get_tweet_retweeters",
            c.twitter().get_tweet_retweeters(tid, d()),
        )
        .await;
        r.run(
            "twitter.get_tweet_favoriters",
            c.twitter().get_tweet_favoriters(tid, d()),
        )
        .await;
        r.run(
            "twitter.get_tweet_community_notes",
            c.twitter().get_tweet_community_notes(tid, d()),
        )
        .await;
        r.run(
            "twitter.get_tweet_edit_history",
            c.twitter().get_tweet_edit_history(tid, d()),
        )
        .await;
        r.run(
            "twitter.get_similar_tweets",
            c.twitter().get_similar_tweets(tid, d()),
        )
        .await;
    } else {
        r.skip("twitter.get_tweet_*", "no tweet id from search");
    }
    r.run(
        "twitter.get_filter_rule_pricing_tiers",
        c.twitter().get_filter_rule_pricing_tiers(d()),
    )
    .await;
    r.run(
        "twitter.list_filter_rules",
        c.twitter().list_filter_rules(d()),
    )
    .await;
    r.run(
        "twitter.list_stream_monitors",
        c.twitter().list_stream_monitors(d()),
    )
    .await;
    r.run(
        "twitter.list_stream_webhooks",
        c.twitter().list_stream_webhooks(d()),
    )
    .await;
    r.run(
        "twitter.list_stream_delivery_logs",
        c.twitter().list_stream_delivery_logs(d()),
    )
    .await;
    r.run(
        "twitter.list_stream_billing_logs",
        c.twitter().list_stream_billing_logs(d()),
    )
    .await;
    r.run(
        "twitter.validate_filter_rule_query",
        c.twitter()
            .validate_filter_rule_query(twitter::ValidateFilterRuleQueryParams {
                query: Some("rust lang".into()),
                ..d()
            }),
    )
    .await;
    r.skip("twitter.get_community_*/get_list_*/get_space_detail/get_broadcast_detail/get_place_detail/get_article_detail/search_list_tweets/search_places", "need community/list/space/broadcast/place/article ids");
    r.skip(
        "twitter.{create,update,delete}_* + test_stream_webhook",
        "stateful mutations — not run in a conformance sweep",
    );

    // ---- vinted ----
    r.run("vinted.list_markets", c.vinted().list_markets(d()))
        .await;
    r.run("vinted.list_colors", c.vinted().list_colors(d()))
        .await;
    r.run("vinted.list_statuses", c.vinted().list_statuses(d()))
        .await;
    r.run(
        "vinted.search_items",
        c.vinted().search_items(vinted::SearchItemsParams {
            query: Some("nike".into()),
            ..d()
        }),
    )
    .await;
    r.run(
        "vinted.search_brands",
        c.vinted().search_brands(vinted::SearchBrandsParams {
            keyword: Some("nike".into()),
            ..d()
        }),
    )
    .await;
    r.skip(
        "vinted.get_item_detail/get_user_profile/get_user_items",
        "need an item id / user id",
    );

    // ---- web ----
    r.run(
        "web.scrape_url",
        c.web().scrape_url(web::ScrapeUrlParams {
            url: Some("https://example.com".into()),
            ..d()
        }),
    )
    .await;
    r.run(
        "web.detect_protection",
        c.web().detect_protection(web::DetectProtectionParams {
            url: Some("https://example.com".into()),
            ..d()
        }),
    )
    .await;

    // ---- ebay ----
    r.run("ebay.list_markets", c.ebay().list_markets(d())).await;
    r.run("ebay.list_categories", c.ebay().list_categories(d()))
        .await;
    r.run(
        "ebay.search",
        c.ebay().search(ebay::SearchParams {
            query: Some("laptop".into()),
            ..d()
        }),
    )
    .await;
    r.run(
        "ebay.autocomplete",
        c.ebay().autocomplete(ebay::AutocompleteParams {
            query: Some("laptop".into()),
            ..d()
        }),
    )
    .await;
    r.skip(
        "ebay.get_item/get_item_reviews/get_seller/completed/browse_category",
        "need an item id / seller username / category id",
    );

    // ---- tiktok ----
    r.run("tiktok.trending_videos", c.tiktok().trending_videos(d()))
        .await;
    r.run(
        "tiktok.trending_hashtags",
        c.tiktok().trending_hashtags(d()),
    )
    .await;
    r.run("tiktok.trending_songs", c.tiktok().trending_songs(d()))
        .await;
    r.run("tiktok.list_regions", c.tiktok().list_regions(d()))
        .await;
    r.run(
        "tiktok.search_general",
        c.tiktok().search_general(tiktok::SearchGeneralParams {
            query: Some("rust".into()),
            ..d()
        }),
    )
    .await;
    r.skip(
        "tiktok.get_user/get_video/get_hashtag/get_music/*replies",
        "need a username / video id / hashtag / music id",
    );

    // ---- youtube ----
    r.run("youtube.trending", c.youtube().trending(d())).await;
    r.run("youtube.categories", c.youtube().categories(d()))
        .await;
    r.run("youtube.regions", c.youtube().regions(d())).await;
    r.run("youtube.languages", c.youtube().languages(d())).await;
    r.run(
        "youtube.search",
        c.youtube().search(youtube::SearchParams {
            query: Some("rust programming".into()),
            ..d()
        }),
    )
    .await;
    r.skip(
        "youtube.get_video/get_channel/get_playlist/*transcript/*comments",
        "need a video id / channel id / playlist id",
    );

    // ---- depop ----
    r.run("depop.list_markets", c.depop().list_markets(d()))
        .await;
    let depop_search = r
        .run(
            "depop.search",
            c.depop().search(depop::SearchParams {
                query: Some("nike".into()),
                market: Some("gb".into()),
                ..d()
            }),
        )
        .await;
    // Depop's guest search often returns an empty result set (0 products), so
    // the detail/user endpoints usually can't be id-chained from it.
    match pick(&depop_search, &["products", "0", "id"]) {
        Some(id) => {
            r.run(
                "depop.get_product",
                c.depop().get_product(&id, d::<depop::GetProductParams>()),
            )
            .await;
        }
        None => r.skip("depop.get_product", "search returned no products to chain"),
    }
    match pick(&depop_search, &["products", "0", "seller", "username"]) {
        Some(user) => {
            r.run("depop.get_user", c.depop().get_user(&user, d()))
                .await;
            r.run(
                "depop.get_user_products",
                c.depop().get_user_products(&user, d()),
            )
            .await;
        }
        None => r.skip(
            "depop.get_user/get_user_products",
            "no shop username to chain",
        ),
    }

    // ---- linkedin (chained) ----
    let lin_jobs = r
        .run(
            "linkedin.jobs_search",
            c.linkedin().jobs_search(linkedin::JobsSearchParams {
                keywords: Some("rust engineer".into()),
                location: Some("London".into()),
                ..d()
            }),
        )
        .await;
    match pick(&lin_jobs, &["jobs", "0", "job_id"]) {
        Some(job) => {
            r.run("linkedin.get_job", c.linkedin().get_job(&job, d()))
                .await;
        }
        None => r.skip("linkedin.get_job", "no job id in search"),
    }
    let lin_co = r
        .run(
            "linkedin.geo_suggest",
            c.linkedin().geo_suggest(linkedin::GeoSuggestParams {
                query: Some("microsoft".into()),
                type_: Some("company".into()),
                ..d()
            }),
        )
        .await;
    match pick(&lin_co, &["suggestions", "0", "id"]) {
        Some(cid) => {
            r.run(
                "linkedin.company_jobs",
                c.linkedin().company_jobs(&cid, d()),
            )
            .await;
        }
        None => r.skip("linkedin.company_jobs", "no company id from geo_suggest"),
    }
    r.run(
        "linkedin.get_company",
        c.linkedin().get_company("microsoft", d()),
    )
    .await;
    r.run(
        "linkedin.get_school",
        c.linkedin().get_school("stanford-university", d()),
    )
    .await;
    r.run(
        "linkedin.get_profile",
        c.linkedin().get_profile("williamhgates", d()),
    )
    .await;
    r.run(
        "linkedin.get_article",
        c.linkedin().get_article("welcome-2024-bill-gates", d()),
    )
    .await;
    r.run(
        "linkedin.get_course",
        c.linkedin().get_course("learning-python", d()),
    )
    .await;
    r.skip(
        "linkedin.get_post",
        "public post slugs are volatile / anti-bot gated",
    );

    // ---- idealista (chained) ----
    r.run("idealista.list_markets", c.idealista().list_markets(d()))
        .await;
    let ide_suggest = r
        .run(
            "idealista.suggest",
            c.idealista().suggest(idealista::SuggestParams {
                query: Some("Madrid".into()),
                ..d()
            }),
        )
        .await;
    let ide_loc = pick(&ide_suggest, &["locations", "0", "locationId"])
        .unwrap_or_else(|| "0-EU-ES-28".into());
    let ide_search = r
        .run(
            "idealista.search",
            c.idealista().search(idealista::SearchParams {
                location: Some(ide_loc.clone()),
                operation: Some("sale".into()),
                ..d()
            }),
        )
        .await;
    r.run(
        "idealista.search_all",
        c.idealista().search_all(idealista::SearchAllParams {
            location: Some(ide_loc),
            operation: Some("sale".into()),
            max_results: Some(30),
            ..d()
        }),
    )
    .await;
    match pick(&ide_search, &["elementList", "0", "propertyCode"]) {
        Some(pc) => {
            r.run(
                "idealista.property_detail",
                c.idealista().property_detail(&pc, d()),
            )
            .await;
            r.run(
                "idealista.property_stats",
                c.idealista().property_stats(&pc, d()),
            )
            .await;
        }
        None => r.skip(
            "idealista.property_detail/property_stats",
            "no propertyCode in search",
        ),
    }
    r.skip(
        "idealista.agency/agency_by_phone",
        "need a real agency short_name / phone",
    );

    // ---- immobiliare (chained) ----
    r.run(
        "immobiliare.list_markets",
        c.immobiliare().list_markets(d()),
    )
    .await;
    r.run("immobiliare.reference", c.immobiliare().reference(d()))
        .await;
    r.run(
        "immobiliare.autocomplete",
        c.immobiliare()
            .autocomplete(immobiliare::AutocompleteParams {
                query: Some("Milano".into()),
                ..d()
            }),
    )
    .await;
    let imm_search = r
        .run(
            "immobiliare.search",
            c.immobiliare().search(immobiliare::SearchParams {
                location: Some("Milano".into()),
                ..d()
            }),
        )
        .await;
    match pick(&imm_search, &["results", "0", "id"]) {
        Some(lid) => {
            r.run(
                "immobiliare.get_listing",
                c.immobiliare().get_listing(&lid, d()),
            )
            .await;
        }
        None => r.skip("immobiliare.get_listing", "no listing id in search"),
    }
    r.run(
        "immobiliare.price_stats",
        c.immobiliare().price_stats(immobiliare::PriceStatsParams {
            region_id: Some("lom".into()),
            ..d()
        }),
    )
    .await;
    r.skip(
        "immobiliare.get_agency/get_agency_listings",
        "need an agency id",
    );

    // ---- leboncoin (chained) ----
    r.run("leboncoin.list_markets", c.leboncoin().list_markets(d()))
        .await;
    r.run(
        "leboncoin.list_categories",
        c.leboncoin().list_categories(d()),
    )
    .await;
    r.run("leboncoin.list_regions", c.leboncoin().list_regions(d()))
        .await;
    r.run(
        "leboncoin.list_departments",
        c.leboncoin().list_departments(d()),
    )
    .await;
    r.run(
        "leboncoin.search_locations",
        c.leboncoin()
            .search_locations(leboncoin::SearchLocationsParams {
                q: Some("Paris".into()),
                ..d()
            }),
    )
    .await;
    let lbc_search = r
        .run(
            "leboncoin.search_ads",
            c.leboncoin().search_ads(leboncoin::SearchAdsParams {
                text: Some("velo".into()),
                ..d()
            }),
        )
        .await;
    // Leboncoin ads are volatile — a `list_id` from search may 404 seconds later.
    match pick(&lbc_search, &["ads", "0", "list_id"]) {
        Some(ad) => {
            r.run("leboncoin.get_ad", c.leboncoin().get_ad(&ad, d()))
                .await;
            r.run("leboncoin.get_similar", c.leboncoin().get_similar(&ad, d()))
                .await;
        }
        None => r.skip("leboncoin.get_ad/get_similar", "no ad id in search"),
    }
    match pick(&lbc_search, &["ads", "0", "user_id"]) {
        Some(seller) => {
            r.run(
                "leboncoin.get_seller",
                c.leboncoin().get_seller(&seller, d()),
            )
            .await;
            r.run(
                "leboncoin.get_seller_listings",
                c.leboncoin().get_seller_listings(&seller, d()),
            )
            .await;
        }
        None => r.skip(
            "leboncoin.get_seller/get_seller_listings",
            "no seller id in search",
        ),
    }

    // ---- loopnet (chained) ----
    r.run("loopnet.list_markets", c.loopnet().list_markets(d()))
        .await;
    r.run(
        "loopnet.list_property_types",
        c.loopnet().list_property_types(d()),
    )
    .await;
    let loop_search = r
        .run(
            "loopnet.search",
            c.loopnet().search(loopnet::SearchParams {
                location: Some("New York, NY".into()),
                listing_type: Some("for-lease".into()),
                ..d()
            }),
        )
        .await;
    match pick(&loop_search, &["listings", "0", "listing_id"]) {
        Some(lid) => {
            r.run("loopnet.get_listing", c.loopnet().get_listing(&lid, d()))
                .await;
        }
        None => r.skip("loopnet.get_listing", "no listing id in search"),
    }
    r.skip("loopnet.get_broker", "need a broker slug + id");

    // ---- realtor (chained) ----
    r.run("realtor.list_markets", c.realtor().list_markets(d()))
        .await;
    r.run(
        "realtor.autocomplete",
        c.realtor().autocomplete(realtor::AutocompleteParams {
            query: Some("Austin, TX".into()),
            ..d()
        }),
    )
    .await;
    let realtor_search = r
        .run(
            "realtor.search",
            c.realtor().search(realtor::SearchParams {
                location: Some("Austin, TX".into()),
                ..d()
            }),
        )
        .await;
    match pick(&realtor_search, &["results", "0", "property_id"]) {
        Some(pid) => {
            r.run("realtor.get_property", c.realtor().get_property(&pid, d()))
                .await;
        }
        None => r.skip("realtor.get_property", "no property id in search"),
    }

    // ---- redfin (chained) ----
    r.run("redfin.list_markets", c.redfin().list_markets(d()))
        .await;
    r.run(
        "redfin.autocomplete",
        c.redfin().autocomplete(redfin::AutocompleteParams {
            query: Some("Seattle".into()),
            ..d()
        }),
    )
    .await;
    let redfin_search = r
        .run(
            "redfin.search",
            c.redfin().search(redfin::SearchParams {
                location: Some("Seattle, WA".into()),
                ..d()
            }),
        )
        .await;
    match pick(&redfin_search, &["results", "0", "property_id"]) {
        Some(pid) => {
            r.run("redfin.get_property", c.redfin().get_property(&pid, d()))
                .await;
        }
        None => r.skip("redfin.get_property", "no property id in search"),
    }
    // Fetch-by-URL is frequently anti-bot blocked (422 blocking_page_detected).
    match pick(&redfin_search, &["results", "0", "url"]) {
        Some(url) => {
            r.run(
                "redfin.get_property_by_url",
                c.redfin()
                    .get_property_by_url(redfin::GetPropertyByUrlParams {
                        url: Some(url),
                        ..d()
                    }),
            )
            .await;
        }
        None => r.skip("redfin.get_property_by_url", "no url in search"),
    }
    r.skip("redfin.get_agent", "need an agent url / id");

    // ---- zillow (chained) ----
    r.run("zillow.list_markets", c.zillow().list_markets(d()))
        .await;
    r.run(
        "zillow.autocomplete",
        c.zillow().autocomplete(zillow::AutocompleteParams {
            query: Some("Austin".into()),
            ..d()
        }),
    )
    .await;
    let zillow_search = r
        .run(
            "zillow.search",
            c.zillow().search(zillow::SearchParams {
                location: Some("Austin, TX".into()),
                ..d()
            }),
        )
        .await;
    match pick(&zillow_search, &["results", "0", "zpid"]) {
        Some(zpid) => {
            r.run("zillow.get_property", c.zillow().get_property(&zpid, d()))
                .await;
        }
        None => r.skip("zillow.get_property", "no zpid in search"),
    }
    match pick(&zillow_search, &["results", "0", "detail_url"]) {
        Some(url) => {
            r.run(
                "zillow.get_property_by_url",
                c.zillow()
                    .get_property_by_url(zillow::GetPropertyByUrlParams {
                        url: Some(url),
                        ..d()
                    }),
            )
            .await;
        }
        None => r.skip("zillow.get_property_by_url", "no detail_url in search"),
    }
    r.skip("zillow.get_agent", "need an agent username / url");

    // ---- summary ----
    let total = r.pass.len() + r.type_fail.len() + r.api_err.len();
    println!("\n========== conformance summary ==========");
    println!("checked:    {total}");
    println!("✓ pass:     {}", r.pass.len());
    println!("✗ TYPE:     {}", r.type_fail.len());
    println!("· api err:  {}", r.api_err.len());
    println!("— skipped:  {}", r.skip.len());
    if !r.type_fail.is_empty() {
        println!("\nTYPE FAILURES (typed model does not match the live response):");
        for (label, msg) in &r.type_fail {
            println!("  {label}\n    {msg}");
        }
    }
    if !r.api_err.is_empty() {
        println!("\napi errors (not type problems — stale id, rate limit, plan limit, …):");
        for (label, msg) in &r.api_err {
            println!("  {label}: {}", msg.chars().take(120).collect::<String>());
        }
    }
    std::process::exit(if r.type_fail.is_empty() { 0 } else { 1 });
}
