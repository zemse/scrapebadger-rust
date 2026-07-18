# scrapebadger (Rust)

Async Rust SDK and CLI for the [ScrapeBadger](https://scrapebadger.com)
web-scraping API with **269 endpoints** across 19 platforms: Amazon, Depop,
eBay, Google, Idealista, Immobiliare, Leboncoin, LinkedIn, LoopNet, Realtor,
Redfin, Reddit, TikTok, Twitter/X, Vinted, Web Scraping, YouTube, Zillow, and
Account, plus real-time Twitter Streams.

One crate, a library and a binary both named `scrapebadger`.

## Install

```bash
cargo install scrapebadger
```

Or as a library:

```toml
[dependencies]
scrapebadger = "0.4"
```

## Setup

Store your API key once (saved to `~/.config/scrapebadger/config.json`, chmod 600):

```bash
scrapebadger config set-key sb_live_xxx
```

Or `export SCRAPEBADGER_API_KEY=sb_live_xxx`, or pass `--api-key` per call.

## Supported platforms

| Platform | Command | Endpoints | What you get |
|---|---|---|---|
| **Amazon** | `scrapebadger amazon` | 14 | products, offers, reviews, sellers, search, bestsellers, deals, categories |
| **Depop** | `scrapebadger depop` | 5 | product search & detail, shop/user profiles, user listings |
| **eBay** | `scrapebadger ebay` | 11 | search, item details & reviews, sellers, categories, completed listings |
| **Google** | `scrapebadger google` | 35 | search, maps, shopping, flights, hotels, news, jobs, trends, scholar, patents, finance, images, lens |
| **Idealista** | `scrapebadger idealista` | 8 | Spain property search & detail, engagement stats, agencies, location suggest |
| **Immobiliare** | `scrapebadger immobiliare` | 8 | Italy property search & detail, agencies & listings, price insights, autocomplete |
| **Leboncoin** | `scrapebadger leboncoin` | 10 | France classifieds search & detail, similar ads, sellers, category/region reference |
| **LinkedIn** | `scrapebadger linkedin` | 10 | job search & detail, company/school/profile pages, posts, articles, Learning, geo suggest |
| **LoopNet** | `scrapebadger loopnet` | 5 | commercial real-estate search & detail, brokers, property-type/market reference |
| **Realtor** | `scrapebadger realtor` | 4 | Realtor.com (US) property search & detail, location autocomplete |
| **Redfin** | `scrapebadger redfin` | 6 | Redfin (US) property search & detail, agents, region autocomplete |
| **Reddit** | `scrapebadger reddit` | 20 | posts, comments, subreddits, users, wikis, search |
| **TikTok** | `scrapebadger tiktok` | 25 | videos, users, hashtags, music, comments, search, trending, ads |
| **Twitter / X** | `scrapebadger twitter` | 53 | tweets, users, followers, search, trends, lists, communities, spaces + real-time streams |
| **Vinted** | `scrapebadger vinted` | 8 | item & user listings, search with brand/color/condition filters |
| **Web** | `scrapebadger web` | 2 | scrape any URL (markdown/html, JS rendering), bot-protection detection |
| **YouTube** | `scrapebadger youtube` | 38 | videos, channels, playlists, comments, transcripts, captions, search, trending |
| **Zillow** | `scrapebadger zillow` | 6 | Zillow (US) property search & detail, agents, region autocomplete |
| **Account** | `scrapebadger account` | 1 | credits & usage |

## CLI

Every endpoint is a nested subcommand:
`scrapebadger <platform> <group> <action> [<ids>] [--flags]`.

```bash
# Account — check credits
scrapebadger account me

# Amazon — product detail, reviews, search
scrapebadger amazon products get B08N5WRWNW
scrapebadger amazon products reviews B08N5WRWNW
scrapebadger amazon search --query "mechanical keyboard"

# Google — web search, maps, flights, trends (note: `google search` has no -o, JSON is default)
scrapebadger google search --q "rust async runtime"
scrapebadger google maps search --q "coffee near bandra mumbai"
scrapebadger google flights search --departure-id DEL --arrival-id BOM --outbound-date 2026-07-01 --trip-type one_way
scrapebadger google jobs search --q "rust developer"
scrapebadger google trends interest --q "rustlang"

# eBay — search, item detail, sellers
scrapebadger ebay search --query "vintage camera"

# Twitter/X — users, tweets, search
scrapebadger twitter users by-username elonmusk
scrapebadger twitter users latest-tweets elonmusk
scrapebadger twitter tweets advanced-search --query "scrapebadger"

# Reddit — subreddit posts, comments, search
scrapebadger reddit subreddits posts sneakers --sort new --limit 10
scrapebadger reddit posts comments 1abc234
scrapebadger reddit search posts --q "best espresso machine"

# TikTok — trending, search, user videos
scrapebadger tiktok trending videos --region US
scrapebadger tiktok search videos --query "rustlang"

# YouTube — search, channels, trending
scrapebadger youtube search --query "rust tutorial"
scrapebadger youtube trending get

# Vinted / Depop — second-hand fashion search
scrapebadger vinted search --query "leather jacket" --order newest_first
scrapebadger depop search --query "carhartt jacket" --market gb

# LinkedIn — jobs, companies, public profiles
scrapebadger linkedin jobs search --keywords "rust engineer" --location London
scrapebadger linkedin companies get microsoft
scrapebadger linkedin profiles get williamhgates

# Real estate — US (Zillow/Redfin/Realtor), EU (Idealista/Immobiliare), commercial (LoopNet)
scrapebadger zillow search --location "Austin, TX"
scrapebadger redfin search --location "Seattle, WA"
scrapebadger loopnet search --location "New York, NY" --listing-type for-lease

# Leboncoin — French classifieds
scrapebadger leboncoin search --text "velo route"

# Web — scrape any URL
scrapebadger web scrape --url https://example.com --format markdown --render-js
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

Full 269-command reference: [`crates/scrapebadger/docs/CLI.md`](crates/scrapebadger/docs/CLI.md).

## Library

```rust
use scrapebadger::ScrapeBadger;

#[tokio::main]
async fn main() -> scrapebadger::Result<()> {
    let client = ScrapeBadger::from_env()?;
    let product = client.amazon().get_product("B08N5WRWNW", Default::default()).await?;
    println!("{product:?}");
    Ok(())
}
```

See [`crates/scrapebadger/README.md`](crates/scrapebadger/README.md) for the
SDK quickstart — pagination streams, real-time Twitter Streams, builder config.

## Development

```
crates/scrapebadger/   the published crate (lib + bin)
specs/                 vendored OpenAPI specs (codegen source of truth)
xtask/                 the OpenAPI → Rust generator
```

```bash
cargo run -p xtask -- gen   # regenerate typed bindings + CLI tree from specs/
cargo build --workspace
cargo test  -p scrapebadger
```

Endpoint reference: [`docs/CLI.md`](crates/scrapebadger/docs/CLI.md) · Backlog: [`TASKS.md`](TASKS.md)

## License

MIT — see [LICENSE](LICENSE).
