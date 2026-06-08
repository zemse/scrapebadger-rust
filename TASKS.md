# TASKS

Build status for the `scrapebadger` Rust crate (lib + bin). All 137
endpoints are generated and compile; boxes below track implementation coverage
and remaining polish.

## Foundation
- [x] Cargo workspace (`crates/scrapebadger` lib+bin of same name, `xtask`)
- [x] Vendor 7 OpenAPI specs into `specs/`
- [x] Core transport: `Client`, `Config`, builder, `x-api-key`, base-URL handling
- [x] Retries (502/503/504 + transient) with exponential backoff
- [x] Typed `Error` enum (401/402/422/429 + `Api` fallback) + `Result`
- [x] `QueryParams` builder; `cursor_stream` / `page_stream` pagination helpers
- [x] OpenAPI → Rust generator (`cargo run -p xtask -- gen`), deterministic output
- [x] `cargo build` / `cargo clippy` / `cargo test` clean

## Twitter Streams (`feature = "stream"`)
- [x] WebSocket consumer → `Stream<Item = Result<TweetEvent>>`
- [x] HMAC-SHA256 webhook signature verify (`X-Signature-256`)
- [x] Monitors / Filter Rules / Webhooks CRUD (generated)

## CLI (`feature = "cli"`)
- [x] `raw` command (covers all endpoints)
- [x] Global config file for API key (`config set-key|path|show`, chmod 600)
- [x] Fully-generated nested subcommand tree for every endpoint (137) — see
      [NESTED_CLI.md](NESTED_CLI.md); table in `cli/generated.rs`, built by `xtask gen`
- [x] `<platform> --help` lists all commands at once; `--help-all` dumps the tree
- [x] Output: `-o json|jsonl|raw`, `--select`, `--all` (best-effort pagination)
- [x] `--explain` / `--curl`; typed `--flags` + `--body` for POST/PATCH
- [x] `commands`, `completions <shell>`, `man <dir>` discovery aids
- [x] Removed the 8 flat shortcuts (clean break, v0.2.0 — mapping in CHANGELOG.md)
- [x] CI `docs/CLI.md` + generated-code staleness check (`.github/workflows/ci.yml`,
      `codegen-fresh` job); `xtask gen` now rustfmt's its output so it stays clean

## Endpoints by platform

### Account (1)
- [x] `account.get_account_info()` — GET `/v1/account/me`

### Amazon (14)
- [x] `amazon.autocomplete()` — GET `/v1/amazon/autocomplete`
- [x] `amazon.browse_category()` — GET `/v1/amazon/category`
- [x] `amazon.get_bestsellers()` — GET `/v1/amazon/bestsellers`
- [x] `amazon.get_deals()` — GET `/v1/amazon/deals`
- [x] `amazon.get_new_releases()` — GET `/v1/amazon/new-releases`
- [x] `amazon.get_offers()` — GET `/v1/amazon/products/{asin}/offers`
- [x] `amazon.get_product()` — GET `/v1/amazon/products/{asin}`
- [x] `amazon.get_reviews()` — GET `/v1/amazon/products/{asin}/reviews`
- [x] `amazon.get_seller()` — GET `/v1/amazon/sellers/{seller_id}`
- [x] `amazon.get_seller_feedback()` — GET `/v1/amazon/sellers/{seller_id}/feedback`
- [x] `amazon.get_seller_products()` — GET `/v1/amazon/sellers/{seller_id}/products`
- [x] `amazon.list_categories()` — GET `/v1/amazon/categories`
- [x] `amazon.list_markets()` — GET `/v1/amazon/markets`
- [x] `amazon.search_products()` — GET `/v1/amazon/search`

### Google (39)
- [x] `google.flights_search()` — GET `/api/v1/flights/search`
- [x] `google.get_autocomplete()` — GET `/api/v1/autocomplete/`
- [x] `google.get_finance_quote()` — GET `/api/v1/finance/quote`
- [x] `google.get_patent_detail()` — GET `/api/v1/patents/detail`
- [x] `google.get_product_detail()` — GET `/api/v1/products/detail`
- [x] `google.hotel_details()` — GET `/api/v1/hotels/details`
- [x] `google.hotels_search()` — GET `/api/v1/hotels/search`
- [x] `google.jobs_search()` — GET `/api/v1/jobs/search`
- [x] `google.local_search()` — GET `/api/v1/local/search`
- [x] `google.maps_photos()` — GET `/api/v1/maps/photos`
- [x] `google.maps_place()` — GET `/api/v1/maps/place`
- [x] `google.maps_posts()` — GET `/api/v1/maps/posts`
- [x] `google.maps_reviews()` — GET `/api/v1/maps/reviews`
- [x] `google.maps_search()` — GET `/api/v1/maps/search`
- [x] `google.news_by_topic()` — GET `/api/v1/news/topics`
- [x] `google.scholar_author()` — GET `/api/v1/scholar/author`
- [x] `google.scholar_author_citation()` — GET `/api/v1/scholar/author/citation`
- [x] `google.scholar_cite()` — GET `/api/v1/scholar/cite`
- [x] `google.scholar_profiles()` — GET `/api/v1/scholar/profiles`
- [x] `google.search()` — GET `/api/v1/search`
- [x] `google.search_ai_mode()` — GET `/api/v1/ai-mode/search`
- [x] `google.search_images()` — GET `/api/v1/images/search`
- [x] `google.search_lens()` — GET `/api/v1/lens/search`
- [x] `google.search_news()` — GET `/api/v1/news/search`
- [x] `google.search_patents()` — GET `/api/v1/patents/search`
- [x] `google.search_scholar()` — GET `/api/v1/scholar/search`
- [x] `google.search_videos()` — GET `/api/v1/videos/search`
- [x] `google.shopping_product()` — GET `/api/v1/shopping/product`
- [x] `google.shopping_product_click()` — GET `/api/v1/shopping/product/click`
- [x] `google.shopping_search()` — GET `/api/v1/shopping/search`
- [x] `google.shorts_search()` — GET `/api/v1/shorts/search`
- [x] `google.trending_news()` — GET `/api/v1/news/trending`
- [x] `google.trends_autocomplete()` — GET `/api/v1/trends/autocomplete`
- [x] `google.trends_interest()` — GET `/api/v1/trends/interest`
- [x] `google.trends_regions()` — GET `/api/v1/trends/regions`
- [x] `google.trends_related()` — GET `/api/v1/trends/related`
- [x] `google.trends_search()` — GET `/api/v1/trends/search`
- [x] `google.trends_trending()` — GET `/api/v1/trends/trending`
- [x] `google.trends_trending_now()` — GET `/api/v1/trends/trending-now`

### Reddit (20)
- [x] `reddit.get_domain_posts()` — GET `/v1/reddit/domains/{domain}/posts`
- [x] `reddit.get_new_subreddits()` — GET `/v1/reddit/subreddits/new`
- [x] `reddit.get_popular_subreddits()` — GET `/v1/reddit/subreddits/popular`
- [x] `reddit.get_post()` — GET `/v1/reddit/posts/{post_id}`
- [x] `reddit.get_post_comments()` — GET `/v1/reddit/posts/{post_id}/comments`
- [x] `reddit.get_post_duplicates()` — GET `/v1/reddit/posts/{post_id}/duplicates`
- [x] `reddit.get_subreddit()` — GET `/v1/reddit/subreddits/{subreddit}`
- [x] `reddit.get_subreddit_posts()` — GET `/v1/reddit/subreddits/{subreddit}/posts`
- [x] `reddit.get_subreddit_rules()` — GET `/v1/reddit/subreddits/{subreddit}/rules`
- [x] `reddit.get_trending_posts()` — GET `/v1/reddit/posts/trending`
- [x] `reddit.get_user()` — GET `/v1/reddit/users/{username}`
- [x] `reddit.get_user_comments()` — GET `/v1/reddit/users/{username}/comments`
- [x] `reddit.get_user_moderated()` — GET `/v1/reddit/users/{username}/moderated`
- [x] `reddit.get_user_posts()` — GET `/v1/reddit/users/{username}/posts`
- [x] `reddit.get_user_trophies()` — GET `/v1/reddit/users/{username}/trophies`
- [x] `reddit.get_wiki_page()` — GET `/v1/reddit/subreddits/{subreddit}/wiki/{page}`
- [x] `reddit.get_wiki_pages()` — GET `/v1/reddit/subreddits/{subreddit}/wiki`
- [x] `reddit.search_posts()` — GET `/v1/reddit/search/posts`
- [x] `reddit.search_subreddits()` — GET `/v1/reddit/search/subreddits`
- [x] `reddit.search_users()` — GET `/v1/reddit/search/users`

### Twitter (53)
- [x] `twitter.advanced_search_tweets()` — GET `/v1/twitter/tweets/advanced_search`
- [x] `twitter.create_filter_rule()` — POST `/v1/twitter/stream/filter-rules`
- [x] `twitter.create_stream_monitor()` — POST `/v1/twitter/stream/monitors`
- [x] `twitter.create_stream_webhook()` — POST `/v1/twitter/stream/webhooks`
- [x] `twitter.delete_filter_rule()` — DELETE `/v1/twitter/stream/filter-rules/{rule_id}`
- [x] `twitter.delete_stream_monitor()` — DELETE `/v1/twitter/stream/monitors/{monitor_id}`
- [x] `twitter.delete_stream_webhook()` — DELETE `/v1/twitter/stream/webhooks/{webhook_id}`
- [x] `twitter.get_article_detail()` — GET `/v1/twitter/tweets/article/{article_id}`
- [x] `twitter.get_broadcast_detail()` — GET `/v1/twitter/spaces/broadcast/{broadcast_id}`
- [x] `twitter.get_community_detail()` — GET `/v1/twitter/communities/{community_id}`
- [x] `twitter.get_community_tweets()` — GET `/v1/twitter/communities/{community_id}/tweets`
- [x] `twitter.get_filter_rule()` — GET `/v1/twitter/stream/filter-rules/{rule_id}`
- [x] `twitter.get_filter_rule_delivery_logs()` — GET `/v1/twitter/stream/filter-rules/{rule_id}/logs`
- [x] `twitter.get_filter_rule_pricing_tiers()` — GET `/v1/twitter/stream/filter-rules-pricing`
- [x] `twitter.get_list_detail()` — GET `/v1/twitter/lists/{list_id}/detail`
- [x] `twitter.get_list_tweets()` — GET `/v1/twitter/lists/{list_id}/tweets`
- [x] `twitter.get_place_detail()` — GET `/v1/twitter/geo/places/{place_id}`
- [x] `twitter.get_similar_tweets()` — GET `/v1/twitter/tweets/tweet/{tweet_id}/similar`
- [x] `twitter.get_space_detail()` — GET `/v1/twitter/spaces/{space_id}`
- [x] `twitter.get_stream_monitor()` — GET `/v1/twitter/stream/monitors/{monitor_id}`
- [x] `twitter.get_trends()` — GET `/v1/twitter/trends/`
- [x] `twitter.get_trends_by_place()` — GET `/v1/twitter/trends/place/{woeid}`
- [x] `twitter.get_tweet_community_notes()` — GET `/v1/twitter/tweets/tweet/{tweet_id}/community_notes`
- [x] `twitter.get_tweet_detail()` — GET `/v1/twitter/tweets/tweet/{tweet_id}`
- [x] `twitter.get_tweet_edit_history()` — GET `/v1/twitter/tweets/tweet/{tweet_id}/edit_history`
- [x] `twitter.get_tweet_favoriters()` — GET `/v1/twitter/tweets/tweet/{tweet_id}/favoriters`
- [x] `twitter.get_tweet_quotes()` — GET `/v1/twitter/tweets/tweet/{tweet_id}/quotes`
- [x] `twitter.get_tweet_replies()` — GET `/v1/twitter/tweets/tweet/{tweet_id}/replies`
- [x] `twitter.get_tweet_retweeters()` — GET `/v1/twitter/tweets/tweet/{tweet_id}/retweeters`
- [x] `twitter.get_tweets_by_ids()` — GET `/v1/twitter/tweets/`
- [x] `twitter.get_user_articles()` — GET `/v1/twitter/users/{user_id}/articles`
- [x] `twitter.get_user_by_id()` — GET `/v1/twitter/users/{user_id}/by_id`
- [x] `twitter.get_user_by_username()` — GET `/v1/twitter/users/{username}/by_username`
- [x] `twitter.get_user_followers()` — GET `/v1/twitter/users/{username}/followers`
- [x] `twitter.get_user_followings()` — GET `/v1/twitter/users/{username}/followings`
- [x] `twitter.get_user_latest_tweets()` — GET `/v1/twitter/users/{username}/latest_tweets`
- [x] `twitter.get_user_mentions()` — GET `/v1/twitter/users/{username}/mentions`
- [x] `twitter.get_user_subscriptions()` — GET `/v1/twitter/users/{user_id}/subscriptions`
- [x] `twitter.get_users_by_ids()` — GET `/v1/twitter/users/batch_by_ids`
- [x] `twitter.get_users_by_usernames()` — GET `/v1/twitter/users/batch_by_usernames`
- [x] `twitter.list_filter_rules()` — GET `/v1/twitter/stream/filter-rules`
- [x] `twitter.list_stream_billing_logs()` — GET `/v1/twitter/stream/billing-logs`
- [x] `twitter.list_stream_delivery_logs()` — GET `/v1/twitter/stream/logs`
- [x] `twitter.list_stream_monitors()` — GET `/v1/twitter/stream/monitors`
- [x] `twitter.list_stream_webhooks()` — GET `/v1/twitter/stream/webhooks`
- [x] `twitter.search_communities()` — GET `/v1/twitter/communities/search`
- [x] `twitter.search_list_tweets()` — GET `/v1/twitter/lists/{list_id}/search_tweets`
- [x] `twitter.search_places()` — GET `/v1/twitter/geo/search`
- [x] `twitter.search_users()` — GET `/v1/twitter/users/search_users`
- [x] `twitter.test_stream_webhook()` — POST `/v1/twitter/stream/webhooks/test`
- [x] `twitter.update_filter_rule()` — PATCH `/v1/twitter/stream/filter-rules/{rule_id}`
- [x] `twitter.update_stream_monitor()` — PATCH `/v1/twitter/stream/monitors/{monitor_id}`
- [x] `twitter.validate_filter_rule_query()` — POST `/v1/twitter/stream/filter-rules/validate`

### Vinted (8)
- [x] `vinted.get_item_detail()` — GET `/v1/vinted/items/{item_id}`
- [x] `vinted.get_user_items()` — GET `/v1/vinted/users/{user_id}/items`
- [x] `vinted.get_user_profile()` — GET `/v1/vinted/users/{user_id}`
- [x] `vinted.list_colors()` — GET `/v1/vinted/colors`
- [x] `vinted.list_markets()` — GET `/v1/vinted/markets`
- [x] `vinted.list_statuses()` — GET `/v1/vinted/statuses`
- [x] `vinted.search_brands()` — GET `/v1/vinted/brands`
- [x] `vinted.search_items()` — GET `/v1/vinted/search`

### Web Scraping (2)
- [x] `web.detect_protection()` — POST `/v1/web/detect`
- [x] `web.scrape_url()` — POST `/v1/web/scrape`

## Future enhancements
- [x] Live integration tests behind `SCRAPEBADGER_API_KEY` (ignored by default)
      — `crates/scrapebadger/tests/integration.rs`
- [x] Auto-reconnect helper for WebSocket streams
      — `Twitter::stream_events_reconnecting()`
- [x] Per-platform pagination convenience methods (`*_stream`)
      — Twitter cursor endpoints in `twitter/pagination.rs`
- [x] Generate enums for fixed-value query params (28 enums; inputs only — model
      fields stay `String` for forward-compat). See `xtask/src/schema.rs`.
- [x] Typed Reddit response models (`reddit/models.rs`, reverse-engineered from
      live samples via `scripts/collect_reddit_samples.py`; wired through the
      generator's `response_override`)
- [ ] Publish to crates.io (`scrapebadger` — name is available) — **on hold**

## Type correctness (live validation)
- [x] **Live type-conformance suite** (`examples/conformance.rs`): calls read
      endpoints, deserializes each live response into its typed model, classifies
      Pass / TYPE-fail / api-err, and chains real ids. Found & fixed 4 real type
      bugs (Twitter string-encoded counts typed `i64`; Vinted `price` object
      typed `String`) via lenient `core::flex` decoding. Last run: 64 pass, 0
      type failures.
      - Coverage gaps (environmental, not type issues): Google scrapers return
        404 "not configured" on the test key; Amazon detail hits anti-bot;
        several Twitter detail endpoints (community/list/space/broadcast/place/
        article) and all stream mutations are skipped — extend when ids/state
        are available.
      - Note: `vinted.search_brands` wants a `keyword` query param but the spec
        declares `query` — a spec/param mismatch to investigate.
- [x] Commit **sanitized response fixtures** (`tests/fixtures/`, 20 files via
      `scripts/make_fixtures.py`) + offline test `tests/conformance_offline.rs`
      — runs in CI with no key. (Google/some endpoints unavailable on test key;
      extend fixtures when reachable.)

## Robustness & ergonomics
- [x] Retry `429` honoring `Retry-After` (capped 60s) + exponential backoff.
- [x] Lenient scalar decoding (`core::flex`) — generated scalar fields accept
      number/bool-as-string and JSON-stringify unexpected objects.
- [x] Cross-platform pagination `*_stream` — Reddit (`after`) in
      `reddit/pagination.rs`; Amazon/Vinted (page) in `*/pagination.rs`.
- [x] `#[non_exhaustive]` on generated enums (and `Error`) so adding a
      variant/field later isn't a breaking change.
- [x] Enums for fixed-value **body** params (web scrape `format`/`engine`,
      stream `status`) — query + body now both typed (32 enums).
- [x] Backoff **jitter** (equal jitter) on client retries + WS reconnect.
- [x] Investigate `vinted.search_brands` `keyword` vs spec's `query` param —
      **resolved**: the live OpenAPI (`/api/openapi.json`) is authoritative and
      declares the param `keyword` (required); our vendored `specs/vinted.json`
      was stale (`query`). Fixed the spec, regenerated (field + wire name now
      `keyword`), updated `examples/conformance.rs`. Verified live:
      `?keyword=nike` → 200 with brand data; `?query=nike` → 422
      `{loc:["query","keyword"],msg:"Field required"}`.
- [x] **Spec drift audit** — diffed all 7 vendored specs against the live
      Portal OpenAPI (`/api/openapi.json`). Findings:
      - Twitter delivery-logs (`/stream/logs` + `/stream/filter-rules/{rule_id}/logs`):
        live exposes 3 optional query params we lacked — `author_username`,
        `delivery_status`, `sort` (enum asc/desc). **Added** to the spec +
        regenerated (additive, non-breaking; `sort` is a typed enum).
      - Google (39 paths): the Portal OpenAPI does **not** cover the separate
        "ScrapeBadger Google Scraper" service — see the resolved item below.
- [x] Resolve vendored `/v1/vinted/search` `color_ids` + `status_ids` — **keep
      them**. Verified live: both are real, working filters (the live Portal spec
      just under-documents them). `status_ids=6` returns only "Neuf avec
      étiquette" items, `status_ids=4` only "Satisfaisant"; `color_ids=1` vs `=7`
      return different item sets and a bogus `color_ids=999` returns 0. Our
      vendored spec is *more complete* than live here — no change needed.
- [x] Locate a Google Scraper OpenAPI — **none is published**. The live service
      is a dynamic dispatcher (`GET /api/v1/<scraper>` → "Scraper 'X' is not
      configured" for unknowns); `/api/v1/{openapi.json,docs,redoc}` all 404. The
      docs site only has prose pages (`docs.scrapebadger.com/google/overview.md`,
      `…/shopping-click-enrichment.md`) — unlike the other 6 platforms, Google has
      **no** structured `/api-reference/endpoint/google/*` (OpenAPI-backed) pages.
      So `specs/google.json` is hand-authored/reverse-engineered with no upstream
      to diff against; the only Google drift-check is the live conformance suite.
- [ ] (follow-up) Extend the live conformance suite to exercise more Google
      endpoints — it's the *only* drift-check Google has. Many currently 404
      "not configured" on the key; revisit now that credits are funded.
