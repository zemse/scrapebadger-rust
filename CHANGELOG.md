# Changelog

## Unreleased

### Added
- Three new platforms, generated from the live Portal OpenAPI and exposed in
  both the SDK and the nested CLI (total coverage now 211 endpoints):
  - **eBay** (`scrapebadger ebay`, 11 endpoints): search, item details and
    reviews, sellers (with feedback and items), categories (with items),
    completed listings, autocomplete, markets.
  - **TikTok** (`scrapebadger tiktok`, 25 endpoints): videos, users, hashtags,
    music, comments, search, trending (videos/hashtags/songs), ads, regions.
  - **YouTube** (`scrapebadger youtube`, 38 endpoints): videos (with comments,
    captions, transcripts, related, streams, live chat), channels, playlists,
    posts, shorts, music search, search, trending, and reference data.
  - All responses are `serde_json::Value` (the live spec leaves 200 bodies
    untyped); all three are live-verified via `examples/conformance.rs`.

## 0.2.0 — 2026-06-09

First release published to crates.io. (0.1.0 was developed but never published,
so this is the initial public version; the entries below record the full set of
changes since the internal 0.1.0.)

### SDK

**Fixed**
- Vinted `search_brands` now sends the `keyword` query param (was `query`). The
  live API requires `keyword`; the vendored spec was stale. **Breaking**:
  `vinted::SearchBrandsParams::query` is now `::keyword`.

**Added**
- Twitter stream delivery-log endpoints (`list_stream_delivery_logs`,
  `get_filter_rule_delivery_logs`) gained the optional `author_username`,
  `delivery_status`, and `sort` (`asc`/`desc`, typed enum) query params that the
  live API supports.
- Typed Reddit response models (`reddit::models`): the 20 Reddit endpoints now
  return typed structs (`PostsResponse`, `RedditPost`, `RedditComment`,
  `RedditSubreddit`, `RedditUser`, …) instead of `serde_json::Value`. Reverse-
  engineered from live samples; tolerant of missing/null/unknown fields. **Breaking**
  for code that treated Reddit responses as `Value`. Reddit's mixed `edited`
  field (bool or timestamp) is modeled as an untagged `Edited` enum.
- Typed enums for fixed-value parameters — both query and request-body inputs
  (32 across the API, e.g. `FlightsSearchStops`, `JobsSearchJobType`,
  `ScrapeUrlFormat`, `UpdateStreamMonitorStatus`). Each `Display`s/serializes to
  its wire value, so `*Params` fields like `stops: Option<FlightsSearchStops>`
  replace the old `Option<String>`. **Breaking** for code that passed these as
  strings. Response/model fields stay `String` so unknown future values still
  deserialize.
- Pagination `*_stream` adapters across platforms — Twitter (`next_cursor`),
  Reddit (`after`), and Amazon/Vinted (page numbers) — that follow pagination
  automatically and yield individual items as a `Stream` (e.g.
  `advanced_search_tweets_stream`, `get_subreddit_posts_stream`,
  `search_products_stream`, `search_items_stream`).
- `Twitter::stream_events_reconnecting()` — an endless WebSocket event stream
  that reconnects with exponential backoff (1s→30s, reset on success).
- Live integration tests behind `SCRAPEBADGER_API_KEY` (ignored by default);
  run with `cargo test --test integration -- --ignored`.
- Live type-conformance sweep (`examples/conformance.rs`): calls read endpoints
  and verifies each response deserializes into its typed model, classifying
  Pass / TYPE-failure / api-error and chaining real ids. Run with
  `cargo run --example conformance` (needs `SCRAPEBADGER_API_KEY`).
- Offline conformance test (`tests/conformance_offline.rs`) over 20 committed,
  type-preserving-sanitized response fixtures — runs in CI with no key as a
  permanent regression guard for the typed models.

### Robustness & forward-compat

- The client now retries `429 Too Many Requests`, honoring the server's
  `Retry-After` (capped at 60s) and otherwise using exponential backoff.
- Retry/reconnect backoff now includes "equal jitter" (client retries and the
  WebSocket auto-reconnect) so concurrent clients don't retry in lockstep.
- Generated response structs capture unknown fields in a `#[serde(flatten)]`
  catch-all (`extra`) instead of silently dropping them — responses are now
  lossless against spec drift.
- Lenient scalar decoding: every generated scalar field routes through
  `core::flex` deserializers that accept numbers-as-strings, bools-as-strings,
  and JSON-stringify an unexpected object — so the scalar/shape drift common in
  scraped data no longer fails a whole response. Found and fixed real mismatches
  in Twitter (string-encoded counts typed `i64`) and Vinted (`price` object
  typed `String`) via the conformance sweep below.
- Generated query-param enums and the `Error` enum are `#[non_exhaustive]`, so
  the API adding a value/variant later is not a breaking change. **Breaking** for
  exhaustive `match` on `Error` (add a `_ =>` arm).

### Tooling

- `cargo run -p xtask -- gen` now runs `rustfmt` over its output, so generated
  code is format-clean.
- GitHub Actions CI: fmt / clippy / build / test / feature-matrix, plus a
  `codegen-fresh` job that regenerates from `specs/` and fails on any drift in
  the checked-in generated code or `docs/CLI.md`.

### CLI: full nested command tree (breaking)

The CLI is now **fully nested** — every one of the 137 endpoints is a
discoverable subcommand (`scrapebadger <platform> <group> <action> [<ids>]
[--flags]`), generated from the OpenAPI specs (the same source as the SDK).
See [`crates/scrapebadger/docs/CLI.md`](crates/scrapebadger/docs/CLI.md).

**Added**
- Nested subcommands for all 137 endpoints, with rich `--help` at every level.
- `scrapebadger <platform> --help` lists every command for that platform at once
  (generated `after_help`), and `--help-all` dumps the entire tree.
- Output controls: `-o json|jsonl|raw`, `--select '<path>'` field projection,
  `--all` to auto-follow pagination cursors (best-effort).
- Request inspection: `--explain` (resolved HTTP request) and `--curl`
  (equivalent curl, API key redacted unless `--reveal`).
- Discovery: `commands` (flat grep-able listing), `completions <shell>`,
  `man <dir>` (roff man pages, one per command).
- POST/PATCH endpoints accept both a raw `--body '<json>'` / `--body-file` and
  typed scalar `--flags` (flags override the body).
- CLI command reference rendered on docs.rs via the `cli_reference` module.

**Changed / removed (breaking)** — the 8 hand-picked flat shortcuts are replaced
by their nested equivalents:

| Removed shortcut                     | New nested command                                                  |
|--------------------------------------|---------------------------------------------------------------------|
| `account`                            | `account me`                                                        |
| `scrape <url>`                       | `web scrape --url <url>`                                             |
| `flights --from --to --date`         | `google flights search --departure-id … --arrival-id … --outbound-date …` |
| `google-search <q>`                  | `google search --q <q>`                                             |
| `amazon-search <q>`                  | `amazon search --query <q>`                                          |
| `amazon-product <asin>`              | `amazon products get <asin>`                                        |
| `twitter-search <q>`                 | `twitter tweets advanced-search --query <q>`                        |
| `reddit-post <id>`                   | `reddit posts get <id>`                                             |

Underscore action names (e.g. `advanced_search`) are accepted as hidden aliases
of their kebab-cased forms (`advanced-search`). The `raw` and `config`
subcommands are unchanged.

## 0.1.0

Initial release — async SDK + CLI covering 137 endpoints across Amazon, Google,
Twitter/X, Reddit, Vinted, Web Scraping, and Account, plus Twitter Streams.
