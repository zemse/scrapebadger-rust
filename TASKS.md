# TASKS

Pending backlog for the `scrapebadger` Rust crate (lib + bin). Completed
foundation, transport, CLI, and the 7 shipped platforms (account, amazon,
google, reddit, twitter, vinted, web) are no longer listed here — see the
git history, `docs/CLI.md` (CI-generated coverage), and the v0.2.0 crates.io
release. Only open work remains below.

## New platforms (live Portal API as of 2026-06-26)

The live `/api/openapi.json` now exposes 3 platforms the crate doesn't cover.
All 3 are **provisioned on the funded account** (verified live, unlike Google):
eBay search returned real listings, TikTok `trending/videos` and YouTube
`trending` returned 200. Per-platform work follows the existing pattern:
vendor spec → `cargo run -p xtask -- gen` → verify CLI tree → add to
`examples/conformance.rs` → fixtures.

### eBay (12 endpoints) — `/v1/ebay/*`
- [ ] Vendor `specs/ebay.json` from the live OpenAPI (`/api/openapi.json`)
- [ ] Regenerate; confirm `ebay` subcommand tree + lib methods compile
- [ ] Endpoints: search, items/{id}, items/{id}/reviews, sellers/{username}
      (+ feedback, items), categories, categories/{id}/items, completed,
      autocomplete, markets
- [ ] Note: search param is `query` (not `keyword`) — a 422 confirmed this
- [ ] Add to conformance suite (provisioned, so testable here)

### TikTok (26 endpoints) — `/v1/tiktok/*`
- [ ] Vendor `specs/tiktok.json`; regenerate; confirm compile
- [ ] Endpoints: search (+ videos/users/hashtags), users/{username}
      (+ followers, following, videos, liked, reposts), videos/{id}
      (+ comments, related, transcript), comments/{id}/replies,
      hashtags/{name} (+ videos), music/{id} (+ videos), trending
      (videos/hashtags/songs), ads/search, oembed, regions
- [ ] Add to conformance suite

### YouTube (39 endpoints) — `/v1/youtube/*`
- [ ] Vendor `specs/youtube.json`; regenerate; confirm compile
- [ ] Endpoints: search, videos/{id} (+ comments, replies, captions,
      transcript, related, streams, live_chat), channels/{id} (+ about,
      videos, shorts, streams, playlists, community, search,
      subscriber_count), channels/resolve, playlists/{id} (+ items),
      posts/{id} (+ comments), shorts, mixes, music/search, trending
      (+ shorts), home, hashtags/{tag}, autocomplete, oembed, videos/batch,
      categories, languages, markets, regions
- [ ] Decide overlap with the existing `yt` CLI (transcripts/metadata) before
      committing to full coverage — the value-add over `yt` is the non-transcript
      endpoints (search, channels, comments, trending)
- [ ] Add to conformance suite

### Release
- [ ] Cut a minor version (3 new platforms is additive) once the above land;
      publish to crates.io and tag a GitHub release

## Blocked

- [ ] **Google conformance** can't run until the Google Scraper service is
      enabled on the account (currently tier `basic`; all `/api/v1/*` →
      404 "not configured"). Revisit if/when the Google add-on is provisioned;
      run `CONFORMANCE_FILTER=google cargo run --example conformance` then.
