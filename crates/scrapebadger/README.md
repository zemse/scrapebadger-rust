# scrapebadger

Async Rust SDK **and** CLI for the [ScrapeBadger](https://scrapebadger.com)
web-scraping API — **207 endpoints** across Amazon, eBay, Google (35 endpoints),
Reddit, TikTok, Twitter/X, Vinted, Web Scraping, YouTube, and Account,
plus real-time Twitter Streams (WebSocket + HMAC webhooks).

One crate ships a library and a binary, both named `scrapebadger`.

## Install

```toml
[dependencies]
scrapebadger = "0.2"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Library

```rust
use scrapebadger::ScrapeBadger;

#[tokio::main]
async fn main() -> scrapebadger::Result<()> {
    // Reads SCRAPEBADGER_API_KEY (or use ScrapeBadger::new("sb_live_…")).
    let client = ScrapeBadger::from_env()?;

    let me = client.account().get_account_info(Default::default()).await?;
    println!("plan: {:?}", me);

    let flights = client
        .google()
        .flights_search(scrapebadger::google::FlightsSearchParams {
            departure_id: Some("DEL".into()),
            arrival_id: Some("BOM".into()),
            outbound_date: Some("2026-07-01".into()),
            trip_type: Some("one_way".into()), // round_trip (default) also needs return_date
            ..Default::default()
        })
        .await?;

    let product = client
        .amazon()
        .get_product("B08N5WRWNW", Default::default())
        .await?;

    let _ = (flights, product);
    Ok(())
}
```

Every endpoint is `client.<platform>().<method>(<path args…>, params)`. Inputs are
`Default`-able `*Params` structs — set what you need, spread the rest with
`..Default::default()`.

### Namespaces

`account()` · `amazon()` · `ebay()` · `google()` · `reddit()` · `tiktok()` · `twitter()` · `vinted()` · `web()` · `youtube()`

### Pagination

Generic helpers in `core::pagination` turn any "fetch one page" closure into a
flat `Stream`. Cursor- and page-paginated endpoints also have ready-made
`*_stream` adapters that follow the pagination for you — Twitter (`next_cursor`),
Reddit (`after`), and Amazon/Vinted (page numbers):

```rust
use futures_util::StreamExt;

# async fn demo(client: scrapebadger::ScrapeBadger) -> scrapebadger::Result<()> {
let stream = client.twitter().get_user_followers_stream("elonmusk", Default::default());
futures_util::pin_mut!(stream);
while let Some(user) = stream.next().await {
    let _user = user?; // one UserData per follower, across all pages
}
# Ok(()) }
```

### Real-time Twitter Streams (`feature = "stream"`, on by default)

```rust
use futures_util::StreamExt;

# async fn demo(client: scrapebadger::ScrapeBadger) -> scrapebadger::Result<()> {
let mut events = Box::pin(client.twitter().stream_events().await?);
while let Some(event) = events.next().await {
    let event = event?;
    println!("@{:?}: {:?}", event.author_username, event.tweet_url);
}
# Ok(()) }
```

For long-lived consumers, `client.twitter().stream_events_reconnecting()` returns
an endless stream that reconnects with exponential backoff on drop/error.

Verify webhook callbacks with
`scrapebadger::twitter::stream::verify_webhook_signature(secret, body, header)`.

## CLI

Every one of the 207 endpoints is a **nested subcommand**
(`scrapebadger <platform> <group> <action> [<ids>] [--flags]`), generated from
the same specs as the SDK. The full tree is in [`docs/CLI.md`](docs/CLI.md) and
on [docs.rs](https://docs.rs/scrapebadger) under `cli_reference`.

```bash
# Store the key once in the global config (~/.config/scrapebadger/config.json, chmod 600):
scrapebadger config set-key sb_live_xxx
# (or `export SCRAPEBADGER_API_KEY=sb_live_xxx`, or pass `--api-key`)

scrapebadger account me
scrapebadger web scrape --url https://example.com --format markdown --render-js
scrapebadger google flights search --departure-id DEL --arrival-id BOM --outbound-date 2026-07-01 --trip-type one_way
scrapebadger amazon products get B08N5WRWNW
scrapebadger reddit subreddits posts sneakers --sort new --limit 10 --select '.posts[].title'
```

### Discovering commands

```bash
scrapebadger reddit --help        # lists every reddit command at once (no drilling)
scrapebadger --help-all           # the entire tree, all platforms
scrapebadger commands | grep wiki # grep-able flat listing
scrapebadger completions zsh      # shell completion script
```

### Output, inspection & escape hatch

```bash
-o json|jsonl|raw                 # output format (pretty JSON default)
--select '.posts[].title'         # project a field path
--all                             # auto-follow pagination cursors (best-effort)
--explain                         # print the resolved HTTP request, don't send
--curl                            # print an equivalent curl command

# raw reaches any endpoint directly:
scrapebadger raw /v1/amazon/products/B08N5WRWNW
scrapebadger raw --method POST /v1/web/scrape -d '{"url":"https://example.com"}'
```

## How it works

Typed models and per-endpoint methods are **generated from the vendored OpenAPI
specs** (`specs/*.json`) by `cargo run -p xtask -- gen`; the ergonomic namespace
layer and transport core are hand-written. See
[`docs/CLI.md`](https://github.com/zemse/scrapebadger-rust/blob/main/crates/scrapebadger/docs/CLI.md)
for the complete endpoint reference, and
[`TASKS.md`](https://github.com/zemse/scrapebadger-rust/blob/main/TASKS.md) for
the backlog.

## Configuration

```rust
use std::time::Duration;
let client = scrapebadger::ScrapeBadger::builder()
    .api_key("sb_live_xxx")
    .timeout(Duration::from_secs(120))
    .max_retries(5)
    .build()?;
# Ok::<(), scrapebadger::Error>(())
```

## Features

- `cli` *(default)* — the `scrapebadger` binary.
- `stream` *(default)* — Twitter Streams WebSocket + webhook verification.

## License

MIT
