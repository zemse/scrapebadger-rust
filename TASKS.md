# TASKS

Pending backlog for the `scrapebadger` Rust crate (lib + bin). Completed work
(foundation, transport, CLI, all 19 platforms incl. the eBay/TikTok/YouTube and
LinkedIn/real-estate/marketplace additions and the Google path fix) is recorded
in git history, `docs/CLI.md` (CI-generated coverage), and the CHANGELOG. Only
open work remains below.
Current coverage: **269 endpoints across 19 platforms.**

## Release
- [ ] Cut 0.4.0 (the 9 new platforms are additive) — publish to crates.io, tag a
      GitHub release. Requires explicit go-ahead (irreversible); run via the
      `release` skill. Version already bumped to 0.4.0 in `Cargo.toml`.

## Known minor issues
- [ ] CLI flag collision: `google search` exposes a Google API param named
      `output` (json|html SERP) as `--output`, shadowing the global `-o`. Works
      if you omit `-o` (JSON is the default), but worth renaming the API param's
      CLI flag (e.g. `--serp-format`) in the generator to avoid the clash.
