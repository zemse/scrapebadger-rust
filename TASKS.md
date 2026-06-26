# TASKS

Pending backlog for the `scrapebadger` Rust crate (lib + bin). Completed work
(foundation, transport, CLI, all 10 platforms incl. the eBay/TikTok/YouTube
additions and the Google path fix) is recorded in git history, `docs/CLI.md`
(CI-generated coverage), and the CHANGELOG. Only open work remains below.
Current coverage: **207 endpoints across 10 platforms.**

## Release
- [ ] Cut a minor version (the 3 new platforms + Google fix are additive) — bump
      to 0.3.0, publish to crates.io, tag a GitHub release. Requires explicit
      go-ahead (irreversible); run via the `release` skill.

## Known minor issues
- [ ] CLI flag collision: `google search` exposes a Google API param named
      `output` (json|html SERP) as `--output`, shadowing the global `-o`. Works
      if you omit `-o` (JSON is the default), but worth renaming the API param's
      CLI flag (e.g. `--serp-format`) in the generator to avoid the clash.
