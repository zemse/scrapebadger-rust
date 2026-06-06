# scrapebadger (Rust)

Async Rust SDK and CLI for the [ScrapeBadger](https://scrapebadger.com)
web-scraping API — **137 endpoints** across Amazon, Google, Twitter/X, Reddit,
Vinted, Web Scraping, and Account, plus real-time Twitter Streams.

One crate, a library and a binary both named `scrapebadger`. Typed models are
generated from ScrapeBadger's OpenAPI specs; the namespace layer is hand-written.

- 📦 **Crate usage & quickstart:** [`crates/scrapebadger/README.md`](crates/scrapebadger/README.md)
- 🏗️ **Design + full endpoint reference:** [`ARCHITECTURE.md`](ARCHITECTURE.md)
- ✅ **Build status:** [`TASKS.md`](TASKS.md)

## Layout

```
crates/scrapebadger/   the published crate (lib + bin)
specs/                 vendored OpenAPI specs (codegen source of truth)
xtask/                 the OpenAPI → Rust generator
```

## Quick start

```bash
# regenerate the typed bindings + CLI command tree from specs/
cargo run -p xtask -- gen

# build & test
cargo build --workspace
cargo test  -p scrapebadger

# use the CLI — every endpoint is a nested subcommand
export SCRAPEBADGER_API_KEY=sb_live_xxx
cargo run -p scrapebadger -- account me
cargo run -p scrapebadger -- reddit subreddits posts sneakers --sort new
cargo run -p scrapebadger -- reddit --help    # lists every reddit command at once
```

See [`crates/scrapebadger/docs/CLI.md`](crates/scrapebadger/docs/CLI.md) for the
full 137-command reference.

## License

MIT — see [LICENSE](LICENSE).
