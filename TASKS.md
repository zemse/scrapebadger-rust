# TASKS

Pending backlog for the `scrapebadger` Rust crate (lib + bin). Completed
foundation, transport, CLI, and the 7 shipped platforms (account, amazon,
google, reddit, twitter, vinted, web) are no longer listed here — see the
git history, `docs/CLI.md` (CI-generated coverage), and the v0.2.0 crates.io
release. Only open work remains below.

## New platforms (live Portal API as of 2026-06-26)

Three platforms added — eBay, TikTok, YouTube — vendored, generated, wired into
the SDK + CLI, and live-verified. Total CLI/SDK coverage is now **211
endpoints** (health endpoints excluded, matching the other platforms). Each was
extracted from the live `/api/openapi.json` into `specs/<plat>.json` and built
through the standard pipeline: `cargo run -p xtask -- gen` → CLI tree → live
conformance. Responses are `serde_json::Value` (the live spec leaves all 200
bodies untyped); typed models can follow later as a separate enhancement.

### eBay (11 endpoints) — `/v1/ebay/*`
- [x] Vendored `specs/ebay.json`; regenerated; `ebay` subcommand tree + lib
      methods compile
- [x] Endpoints: search, items/{id}, items/{id}/reviews, sellers/{username}
      (+ feedback, items), categories, categories/{id}/items, completed,
      autocomplete, markets
- [x] search param is `query` (not `keyword`) — confirmed live
- [x] Added to conformance suite (4 checks pass live: markets, categories,
      search, autocomplete; id-dependent detail endpoints skipped)

### TikTok (25 endpoints) — `/v1/tiktok/*`
- [x] Vendored `specs/tiktok.json`; regenerated; compiles
- [x] Endpoints: search (+ videos/users/hashtags), users/{username}
      (+ followers, following, videos, liked, reposts), videos/{id}
      (+ comments, related, transcript), comments/{id}/replies,
      hashtags/{name} (+ videos), music/{id} (+ videos), trending
      (videos/hashtags/songs), ads/search, oembed, regions
- [x] Added to conformance suite (5 checks pass live: trending
      videos/hashtags/songs, regions, search)

### YouTube (38 endpoints) — `/v1/youtube/*`
- [x] Vendored `specs/youtube.json`; regenerated; compiles
- [x] Endpoints: search, videos/{id} (+ comments, replies, captions,
      transcript, related, streams, live_chat), channels/{id} (+ about,
      videos, shorts, streams, playlists, community, search,
      subscriber_count), channels/resolve, playlists/{id} (+ items),
      posts/{id} (+ comments), shorts, mixes, music/search, trending
      (+ shorts), home, hashtags/{tag}, autocomplete, oembed, videos/batch,
      categories, languages, markets, regions
- [x] Overlap with `yt` CLI: covered all endpoints — `yt` owns transcripts, but
      the SDK's value-add is search, channels, comments, playlists, trending
- [x] Added to conformance suite (5 checks pass live: trending, categories,
      regions, languages, search)

### Release
- [ ] Cut a minor version (3 new platforms is additive) — bump to 0.3.0,
      publish to crates.io, tag a GitHub release. Requires explicit go-ahead
      (irreversible); run via the `release` skill.

## Blocked

- [ ] **Google conformance** can't run until the Google Scraper service is
      enabled on the account (currently tier `basic`; all `/api/v1/*` →
      404 "not configured"). Revisit if/when the Google add-on is provisioned;
      run `CONFORMANCE_FILTER=google cargo run --example conformance` then.
