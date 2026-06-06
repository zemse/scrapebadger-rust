# scrapebadger

Async Rust SDK **and** CLI for the [ScrapeBadger](https://scrapebadger.com)
web-scraping API — **137 endpoints** across Amazon, Google (16 product APIs),
Twitter/X, Reddit, Vinted, Web Scraping, and Account, plus real-time Twitter
Streams (WebSocket + HMAC webhooks).

One crate ships a library and a binary, both named `scrapebadger`.

## Install

```toml
[dependencies]
scrapebadger = "0.1"
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

`account()` · `amazon()` · `google()` · `reddit()` · `twitter()` · `vinted()` · `web()`

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

Verify webhook callbacks with
`scrapebadger::twitter::stream::verify_webhook_signature(secret, body, header)`.

## CLI

```bash
# Store the key once in the global config (~/.config/scrapebadger/config.json, chmod 600):
scrapebadger config set-key sb_live_xxx
# (or `export SCRAPEBADGER_API_KEY=sb_live_xxx`, or pass `--api-key`)

scrapebadger account
scrapebadger scrape https://example.com --format markdown --render-js
scrapebadger flights --from DEL --to BOM --date 2026-07-01
scrapebadger amazon-product B08N5WRWNW
scrapebadger twitter-search "rust lang"

# raw reaches any endpoint:
scrapebadger raw /v1/amazon/products/B08N5WRWNW
scrapebadger raw --method POST /v1/web/scrape -d '{"url":"https://example.com"}'
```

## How it works

Typed models and per-endpoint methods are **generated from the vendored OpenAPI
specs** (`specs/*.json`) by `cargo run -p xtask -- gen`; the ergonomic namespace
layer and transport core are hand-written. See [`ARCHITECTURE.md`](../../ARCHITECTURE.md)
for the full design, codegen notes, and the complete endpoint reference, and
[`TASKS.md`](../../TASKS.md) for build status.

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
