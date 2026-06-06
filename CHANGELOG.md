# Changelog

## 0.2.0 — unreleased

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
