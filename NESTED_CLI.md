# Nested CLI — design & implementation plan

Revamp the `scrapebadger` CLI from ~8 hand-picked subcommands into a **fully nested**, 
spec-generated command tree covering **all 137 endpoints**. Every endpoint becomes a 
discoverable subcommand: `scrapebadger <platform> <group> [<subgroup>] <action> [<ids>] [--flags]`.

> Source of truth: `specs/*.json` (the same specs that generate the SDK). The CLI tree, 
arg names, and help text are all derived from them — nothing hand-maintained per endpoint.

## Conventions

- **Static path segments → subcommands** (`subreddits`, `posts`). Finite, listable in `--help`, tab-completable.

- **`{path params}` → positional args**, reordered to the end (`subreddits posts <subreddit>`).

- **Query params → `--flags`** (kebab-cased: `render_js` → `--render-js`). `*` marks required.

- **Leaf verbs** disambiguate fetch-one / CRUD / runnable-parents: `get` (fetch by id), `list` (collection), `create` / `update` / `delete` (POST / PATCH / DELETE).

- **POST/PATCH bodies** → either typed `--field` flags or a raw `--body '<json>'`.

- `raw <METHOD> <path>` stays as the escape hatch for anything unmodeled.

## Coverage checklist

All **137** data endpoints (the 3 `/health` endpoints are excluded, matching the SDK).


### Account — `scrapebadger account` (1 endpoints)

| ✓ | Command | Args | Method & path | Query flags | Summary |
|---|---------|------|---------------|-------------|---------|
| [ ] | `account me` | — | `GET /v1/account/me` | — | Get Account Info |

### Amazon — `scrapebadger amazon` (14 endpoints)

| ✓ | Command | Args | Method & path | Query flags | Summary |
|---|---------|------|---------------|-------------|---------|
| [ ] | `amazon autocomplete` | — | `GET /v1/amazon/autocomplete` | --query* --domain | Keyword Suggestions |
| [ ] | `amazon bestsellers` | — | `GET /v1/amazon/bestsellers` | --domain --category --page | Bestsellers |
| [ ] | `amazon categories` | — | `GET /v1/amazon/categories` | --domain | List Categories |
| [ ] | `amazon category` | — | `GET /v1/amazon/category` | --node* --domain --page --sort-by | Browse Category |
| [ ] | `amazon deals` | — | `GET /v1/amazon/deals` | --domain --category --page | Today's Deals |
| [ ] | `amazon markets` | — | `GET /v1/amazon/markets` | — | List Markets |
| [ ] | `amazon new-releases` | — | `GET /v1/amazon/new-releases` | --domain --category --page | New Releases |
| [ ] | `amazon products get` | `<asin>` | `GET /v1/amazon/products/{asin}` | --domain --zip --language | Get Product Detail |
| [ ] | `amazon products offers` | `<asin>` | `GET /v1/amazon/products/{asin}/offers` | --domain --zip | Get Offers |
| [ ] | `amazon products reviews` | `<asin>` | `GET /v1/amazon/products/{asin}/reviews` | --domain --page --sort-by --star --verified-only --media-only | Get Reviews |
| [ ] | `amazon search` | — | `GET /v1/amazon/search` | --query* --domain --page --sort-by --category --min-price --max-price --zip --language | Search Products |
| [ ] | `amazon sellers feedback` | `<seller_id>` | `GET /v1/amazon/sellers/{seller_id}/feedback` | --domain --page | Seller Feedback |
| [ ] | `amazon sellers get` | `<seller_id>` | `GET /v1/amazon/sellers/{seller_id}` | --domain | Get Seller |
| [ ] | `amazon sellers products` | `<seller_id>` | `GET /v1/amazon/sellers/{seller_id}/products` | --domain --page | Seller Products |

### Google — `scrapebadger google` (39 endpoints)

| ✓ | Command | Args | Method & path | Query flags | Summary |
|---|---------|------|---------------|-------------|---------|
| [ ] | `google ai-mode search` | — | `GET /api/v1/ai-mode/search` | --q* --gl --hl | Google AI Mode search |
| [ ] | `google autocomplete` | — | `GET /api/v1/autocomplete/` | --q* --hl --gl | Google search suggestions |
| [ ] | `google finance quote` | — | `GET /api/v1/finance/quote` | --q* --hl | Get Google Finance stock quote |
| [ ] | `google flights search` | — | `GET /api/v1/flights/search` | --departure-id* --arrival-id* --outbound-date* --return-date --trip-type --adults --children --infants-in-seat --infants-on-lap --travel-class --currency --gl --hl --stops --max-price | Google Flights search |
| [ ] | `google hotels details` | — | `GET /api/v1/hotels/details` | --property-token* --check-in* --check-out* --adults --currency --gl --hl | Hotel details |
| [ ] | `google hotels search` | — | `GET /api/v1/hotels/search` | --q* --check-in* --check-out* --adults --children --currency --gl --hl --sort-by --min-price --max-price --hotel-class --next-page-token | Search Google Hotels |
| [ ] | `google images search` | — | `GET /api/v1/images/search` | --q* --gl --hl --tbs --imgsz --imgcolor --imgtype --safe --page | Search Google Images |
| [ ] | `google jobs search` | — | `GET /api/v1/jobs/search` | --q* --location --gl --country --hl --language --domain --job-type --date-posted --ltype --chips --uds --uule --lrad --next-page-token --mode | Search Google Jobs |
| [ ] | `google lens search` | — | `GET /api/v1/lens/search` | --url* --gl --hl | Google Lens visual search |
| [ ] | `google local search` | — | `GET /api/v1/local/search` | --q* --gl --hl --domain --location --uule --num --start | Google Local Pack search |
| [ ] | `google maps photos` | — | `GET /api/v1/maps/photos` | --data-id* --hl --gl | Get place photos |
| [ ] | `google maps place` | — | `GET /api/v1/maps/place` | --place-id --data-id --hl --gl | Get place details |
| [ ] | `google maps posts` | — | `GET /api/v1/maps/posts` | --data-id --place-id --hl --gl | Get business posts |
| [ ] | `google maps reviews` | — | `GET /api/v1/maps/reviews` | --data-id* --sort-by --hl --gl --next-page-token --offset --results | Get place reviews |
| [ ] | `google maps search` | — | `GET /api/v1/maps/search` | --q* --ll --gl --hl --start | Search Google Maps places |
| [ ] | `google news search` | — | `GET /api/v1/news/search` | --q* --hl --gl --max-results | Search Google News |
| [ ] | `google news topics` | — | `GET /api/v1/news/topics` | --topic* --hl --gl --max-results | News by topic |
| [ ] | `google news trending` | — | `GET /api/v1/news/trending` | --hl --gl --max-results | Trending news |
| [ ] | `google patents detail` | — | `GET /api/v1/patents/detail` | --patent-id* | Get patent details |
| [ ] | `google patents search` | — | `GET /api/v1/patents/search` | --q* --page --num --sort --inventor --assignee --country --language --status --patent-type --before --after | Search Google Patents |
| [ ] | `google products detail` | — | `GET /api/v1/products/detail` | --product-id* --q* --gl --hl --domain --include-offers --include-variants --resolve-deep-urls | Google immersive product detail |
| [ ] | `google scholar author citation` | — | `GET /api/v1/scholar/author/citation` | --author-id* --hl | Get citation chart data for a Scholar author |
| [ ] | `google scholar author get` | — | `GET /api/v1/scholar/author` | --author-id* --hl --cstart --pagesize | Get Google Scholar author profile |
| [ ] | `google scholar cite` | — | `GET /api/v1/scholar/cite` | --q* --hl | Get citation formats for a Scholar paper |
| [ ] | `google scholar profiles` | — | `GET /api/v1/scholar/profiles` | --mauthors* --hl --after-author --before-author | Search Google Scholar author profiles |
| [ ] | `google scholar search` | — | `GET /api/v1/scholar/search` | --q* --hl --as-ylo --as-yhi --as-sdt --page --num | Search Google Scholar |
| [ ] | `google search` | — | `GET /api/v1/search` | --q* --gl --hl --num --start --domain --device --location --lr --tbs --safe --uule --filter --nfpr --cr --ludocid --lsig --kgmid --si --ibp --uds --ai-overview --mode | Google web search |
| [ ] | `google shopping product click` | — | `GET /api/v1/shopping/product/click` | --title* --source --q --product-id --gl --hl | Resolve merchant URL for a Google Shopping product |
| [ ] | `google shopping product get` | — | `GET /api/v1/shopping/product` | --product-id* --gl --hl | Product details |
| [ ] | `google shopping search` | — | `GET /api/v1/shopping/search` | --q* --gl --hl --min-price --max-price --sort-by --free-shipping --on-sale --start | Search Google Shopping |
| [ ] | `google shorts search` | — | `GET /api/v1/shorts/search` | --q* --gl --hl --domain --num --start | Google Shorts search |
| [ ] | `google trends autocomplete` | — | `GET /api/v1/trends/autocomplete` | --q* --hl --tz | Trends topic autocomplete |
| [ ] | `google trends interest` | — | `GET /api/v1/trends/interest` | --q* --geo --date --category --gprop | Interest over time |
| [ ] | `google trends regions` | — | `GET /api/v1/trends/regions` | --q* --geo --date --resolution | Interest by region |
| [ ] | `google trends related` | — | `GET /api/v1/trends/related` | --q* --geo --date | Related topics & queries |
| [ ] | `google trends search` | — | `GET /api/v1/trends/search` | --q* --data-type --geo --date --cat --gprop --region --language --tz | Google Trends — unified search |
| [ ] | `google trends trending` | — | `GET /api/v1/trends/trending` | --geo --hl --hours | Trending searches |
| [ ] | `google trends trending-now` | — | `GET /api/v1/trends/trending-now` | --geo --hours --category --status --sort --hl | Google Trends — current trending searches |
| [ ] | `google videos search` | — | `GET /api/v1/videos/search` | --q* --gl --hl --tbs --safe --page | Search Google Videos |

### Reddit — `scrapebadger reddit` (20 endpoints)

| ✓ | Command | Args | Method & path | Query flags | Summary |
|---|---------|------|---------------|-------------|---------|
| [ ] | `reddit domains posts` | `<domain>` | `GET /v1/reddit/domains/{domain}/posts` | --sort --t --limit --after | Get posts by domain |
| [ ] | `reddit posts comments` | `<post_id>` | `GET /v1/reddit/posts/{post_id}/comments` | --sort --limit --depth | Get post comments |
| [ ] | `reddit posts duplicates` | `<post_id>` | `GET /v1/reddit/posts/{post_id}/duplicates` | --limit --after | Get cross-posts |
| [ ] | `reddit posts get` | `<post_id>` | `GET /v1/reddit/posts/{post_id}` | — | Get post detail |
| [ ] | `reddit posts trending` | — | `GET /v1/reddit/posts/trending` | --sort --t --limit --after | Get trending posts |
| [ ] | `reddit search posts` | — | `GET /v1/reddit/search/posts` | --q* --subreddit --sort --t --limit --after | Search Reddit posts |
| [ ] | `reddit search subreddits` | — | `GET /v1/reddit/search/subreddits` | --q* --limit --after | Search subreddits |
| [ ] | `reddit search users` | — | `GET /v1/reddit/search/users` | --q* --limit --after | Search users |
| [ ] | `reddit subreddits get` | `<subreddit>` | `GET /v1/reddit/subreddits/{subreddit}` | — | Get subreddit info |
| [ ] | `reddit subreddits new` | — | `GET /v1/reddit/subreddits/new` | --limit --after | New subreddits |
| [ ] | `reddit subreddits popular` | — | `GET /v1/reddit/subreddits/popular` | --limit --after | Popular subreddits |
| [ ] | `reddit subreddits posts` | `<subreddit>` | `GET /v1/reddit/subreddits/{subreddit}/posts` | --sort --t --limit --after | Get subreddit posts |
| [ ] | `reddit subreddits rules` | `<subreddit>` | `GET /v1/reddit/subreddits/{subreddit}/rules` | — | Get subreddit rules |
| [ ] | `reddit subreddits wiki get` | `<subreddit> <page>` | `GET /v1/reddit/subreddits/{subreddit}/wiki/{page}` | — | Get wiki page content |
| [ ] | `reddit subreddits wiki list` | `<subreddit>` | `GET /v1/reddit/subreddits/{subreddit}/wiki` | — | List wiki pages |
| [ ] | `reddit users comments` | `<username>` | `GET /v1/reddit/users/{username}/comments` | --sort --t --limit --after | Get user's comments |
| [ ] | `reddit users get` | `<username>` | `GET /v1/reddit/users/{username}` | — | Get user profile |
| [ ] | `reddit users moderated` | `<username>` | `GET /v1/reddit/users/{username}/moderated` | — | Get user's moderated subreddits |
| [ ] | `reddit users posts` | `<username>` | `GET /v1/reddit/users/{username}/posts` | --sort --t --limit --after | Get user's posts |
| [ ] | `reddit users trophies` | `<username>` | `GET /v1/reddit/users/{username}/trophies` | — | Get user's trophies |

### Twitter/X — `scrapebadger twitter` (53 endpoints)

| ✓ | Command | Args | Method & path | Query flags | Summary |
|---|---------|------|---------------|-------------|---------|
| [ ] | `twitter communities get` | `<community_id>` | `GET /v1/twitter/communities/{community_id}` | — | Get community details |
| [ ] | `twitter communities search` | — | `GET /v1/twitter/communities/search` | --query* --cursor | Search communities |
| [ ] | `twitter communities tweets` | `<community_id>` | `GET /v1/twitter/communities/{community_id}/tweets` | --tweet-type --cursor | Get community tweets |
| [ ] | `twitter geo places get` | `<place_id>` | `GET /v1/twitter/geo/places/{place_id}` | — | Get place details |
| [ ] | `twitter geo search` | — | `GET /v1/twitter/geo/search` | --query --lat --long | Search places |
| [ ] | `twitter lists detail` | `<list_id>` | `GET /v1/twitter/lists/{list_id}/detail` | — | Get list details |
| [ ] | `twitter lists search_tweets` | `<list_id>` | `GET /v1/twitter/lists/{list_id}/search_tweets` | --query* --cursor | Search list tweets |
| [ ] | `twitter lists tweets` | `<list_id>` | `GET /v1/twitter/lists/{list_id}/tweets` | --cursor | Get list tweets |
| [ ] | `twitter spaces broadcast get` | `<broadcast_id>` | `GET /v1/twitter/spaces/broadcast/{broadcast_id}` | — | Get broadcast details |
| [ ] | `twitter spaces get` | `<space_id>` | `GET /v1/twitter/spaces/{space_id}` | — | Get Space details |
| [ ] | `twitter stream billing-logs` | — | `GET /v1/twitter/stream/billing-logs` | --monitor-id --page --page-size | List billing logs |
| [ ] | `twitter stream filter-rules create` | — | `POST /v1/twitter/stream/filter-rules` | — `--body` | Create filter rule |
| [ ] | `twitter stream filter-rules delete` | `<rule_id>` | `DELETE /v1/twitter/stream/filter-rules/{rule_id}` | — | Delete filter rule |
| [ ] | `twitter stream filter-rules get` | `<rule_id>` | `GET /v1/twitter/stream/filter-rules/{rule_id}` | — | Get filter rule |
| [ ] | `twitter stream filter-rules list` | — | `GET /v1/twitter/stream/filter-rules` | --page --page-size --status | List filter rules |
| [ ] | `twitter stream filter-rules logs` | `<rule_id>` | `GET /v1/twitter/stream/filter-rules/{rule_id}/logs` | --page --page-size | Get filter rule delivery logs |
| [ ] | `twitter stream filter-rules update` | `<rule_id>` | `PATCH /v1/twitter/stream/filter-rules/{rule_id}` | — `--body` | Update filter rule |
| [ ] | `twitter stream filter-rules validate` | — | `POST /v1/twitter/stream/filter-rules/validate` | — `--body` | Validate filter rule query |
| [ ] | `twitter stream filter-rules-pricing` | — | `GET /v1/twitter/stream/filter-rules-pricing` | — | Get filter rule pricing tiers |
| [ ] | `twitter stream logs` | — | `GET /v1/twitter/stream/logs` | --monitor-id --page --page-size | List delivery logs |
| [ ] | `twitter stream monitors create` | — | `POST /v1/twitter/stream/monitors` | — `--body` | Create stream monitor |
| [ ] | `twitter stream monitors delete` | `<monitor_id>` | `DELETE /v1/twitter/stream/monitors/{monitor_id}` | — | Delete stream monitor |
| [ ] | `twitter stream monitors get` | `<monitor_id>` | `GET /v1/twitter/stream/monitors/{monitor_id}` | — | Get stream monitor |
| [ ] | `twitter stream monitors list` | — | `GET /v1/twitter/stream/monitors` | --page --page-size --status | List stream monitors |
| [ ] | `twitter stream monitors update` | `<monitor_id>` | `PATCH /v1/twitter/stream/monitors/{monitor_id}` | — `--body` | Update stream monitor |
| [ ] | `twitter stream webhooks create` | — | `POST /v1/twitter/stream/webhooks` | — `--body` | Create webhook |
| [ ] | `twitter stream webhooks delete` | `<webhook_id>` | `DELETE /v1/twitter/stream/webhooks/{webhook_id}` | — | Delete webhook |
| [ ] | `twitter stream webhooks list` | — | `GET /v1/twitter/stream/webhooks` | --monitor-id | List webhooks |
| [ ] | `twitter stream webhooks test` | — | `POST /v1/twitter/stream/webhooks/test` | — `--body` | Test webhook |
| [ ] | `twitter trends get` | — | `GET /v1/twitter/trends/` | --category --count | Get trending topics |
| [ ] | `twitter trends place get` | `<woeid>` | `GET /v1/twitter/trends/place/{woeid}` | — | Get trends by location |
| [ ] | `twitter tweets advanced_search` | — | `GET /v1/twitter/tweets/advanced_search` | --query* --query-type --count --cursor | Advanced tweet search |
| [ ] | `twitter tweets article get` | `<article_id>` | `GET /v1/twitter/tweets/article/{article_id}` | — | Get article by ID |
| [ ] | `twitter tweets get` | — | `GET /v1/twitter/tweets/` | --tweets* | Get tweets by IDs |
| [ ] | `twitter tweets tweet community_notes` | `<tweet_id>` | `GET /v1/twitter/tweets/tweet/{tweet_id}/community_notes` | — | Get community notes |
| [ ] | `twitter tweets tweet edit_history` | `<tweet_id>` | `GET /v1/twitter/tweets/tweet/{tweet_id}/edit_history` | — | Get tweet edit history |
| [ ] | `twitter tweets tweet favoriters` | `<tweet_id>` | `GET /v1/twitter/tweets/tweet/{tweet_id}/favoriters` | --cursor | Get tweet favoriters |
| [ ] | `twitter tweets tweet get` | `<tweet_id>` | `GET /v1/twitter/tweets/tweet/{tweet_id}` | --cursor | Get tweet details |
| [ ] | `twitter tweets tweet quotes` | `<tweet_id>` | `GET /v1/twitter/tweets/tweet/{tweet_id}/quotes` | --cursor | Get tweet quotes |
| [ ] | `twitter tweets tweet replies` | `<tweet_id>` | `GET /v1/twitter/tweets/tweet/{tweet_id}/replies` | --cursor | Get tweet replies |
| [ ] | `twitter tweets tweet retweeters` | `<tweet_id>` | `GET /v1/twitter/tweets/tweet/{tweet_id}/retweeters` | --cursor | Get tweet retweeters |
| [ ] | `twitter tweets tweet similar` | `<tweet_id>` | `GET /v1/twitter/tweets/tweet/{tweet_id}/similar` | — | Get similar tweets |
| [ ] | `twitter users articles` | `<user_id>` | `GET /v1/twitter/users/{user_id}/articles` | --cursor | Get user articles |
| [ ] | `twitter users batch_by_ids` | — | `GET /v1/twitter/users/batch_by_ids` | --user-ids* | Batch get users by IDs |
| [ ] | `twitter users batch_by_usernames` | — | `GET /v1/twitter/users/batch_by_usernames` | --usernames* | Batch get users by usernames |
| [ ] | `twitter users by_id` | `<user_id>` | `GET /v1/twitter/users/{user_id}/by_id` | — | Get user by ID |
| [ ] | `twitter users by_username` | `<username>` | `GET /v1/twitter/users/{username}/by_username` | — | Get user by username |
| [ ] | `twitter users followers` | `<username>` | `GET /v1/twitter/users/{username}/followers` | --cursor | Get user followers |
| [ ] | `twitter users followings` | `<username>` | `GET /v1/twitter/users/{username}/followings` | --cursor | Get user following |
| [ ] | `twitter users latest_tweets` | `<username>` | `GET /v1/twitter/users/{username}/latest_tweets` | --cursor | Get user tweets |
| [ ] | `twitter users mentions` | `<username>` | `GET /v1/twitter/users/{username}/mentions` | --count --cursor | Get user mentions |
| [ ] | `twitter users search_users` | — | `GET /v1/twitter/users/search_users` | --query* --cursor | Search users |
| [ ] | `twitter users subscriptions` | `<user_id>` | `GET /v1/twitter/users/{user_id}/subscriptions` | --cursor | Get user subscriptions |

### Vinted — `scrapebadger vinted` (8 endpoints)

| ✓ | Command | Args | Method & path | Query flags | Summary |
|---|---------|------|---------------|-------------|---------|
| [ ] | `vinted brands` | — | `GET /v1/vinted/brands` | --query* --market | Search Brands |
| [ ] | `vinted colors` | — | `GET /v1/vinted/colors` | --market | List Colors |
| [ ] | `vinted items get` | `<item_id>` | `GET /v1/vinted/items/{item_id}` | --market | Get Item Details |
| [ ] | `vinted markets` | — | `GET /v1/vinted/markets` | — | List Markets |
| [ ] | `vinted search` | — | `GET /v1/vinted/search` | --query* --market --page --per-page --price-from --price-to --brand-ids --color-ids --status-ids --order | Search Items |
| [ ] | `vinted statuses` | — | `GET /v1/vinted/statuses` | --market | List Conditions |
| [ ] | `vinted users get` | `<user_id>` | `GET /v1/vinted/users/{user_id}` | --market | Get User Profile |
| [ ] | `vinted users items` | `<user_id>` | `GET /v1/vinted/users/{user_id}/items` | --market --page --per-page | Get User Items |

### Web Scraping — `scrapebadger web` (2 endpoints)

| ✓ | Command | Args | Method & path | Query flags | Summary |
|---|---------|------|---------------|-------------|---------|
| [ ] | `web detect` | — | `POST /v1/web/detect` | — `--body` | Detect Protection |
| [ ] | `web scrape` | — | `POST /v1/web/scrape` | — `--body` | Scrape URL |
## Documentation: how the CLI is self-documenting

Five layers, all derived from the same specs — nothing written twice.

### 1. Built-in `--help` at every level — and **the whole subtree at once**
The nested tree means `--help` works at each depth, narrowing as you go. But by
default a parent only lists its *immediate* children, forcing you to drill
`reddit --help` → `reddit subreddits --help` → … . We don't want that.

**The fix: a generated `after_help` block that lists every descendant leaf
command on the parent's help page.** So `scrapebadger reddit --help` shows both
the group navigation *and* a flat listing of all 20 reddit commands with
summaries — no iteration needed:
```
$ scrapebadger reddit --help
Reddit posts, comments, subreddits, users, and wiki content

Usage: scrapebadger reddit [GROUP]

Commands:                       # clap's normal section — for navigation
  domains     Posts by domain
  posts       Posts, comments, duplicates, trending
  search      Search posts / subreddits / users
  subreddits  Subreddit info, posts, rules, wiki
  users       User profiles, posts, comments, trophies

All reddit commands:            # GENERATED after_help — every leaf at a glance
  domains posts <domain>           Get posts by domain
  posts get <post_id>              Get post detail
  posts comments <post_id>         Get post comments
  ...
  subreddits posts <subreddit>     Get subreddit posts
  subreddits wiki get <sub> <page> Get wiki page content
  users posts <username>           Get user's posts
  ...
Run `scrapebadger reddit <command> --help` for arguments and flags.
```
The leaf-level help still narrows to one endpoint with its args/flags:
```
$ scrapebadger reddit subreddits posts --help
Get subreddit posts
Usage: scrapebadger reddit subreddits posts <SUBREDDIT> [OPTIONS]
Arguments:  <SUBREDDIT>
Options:    --sort <SORT>    Sort: hot, new, top, rising, controversial
            --limit <LIMIT>  ...
```
All text comes straight from each operation's `summary`/`description` and each
parameter's `description` in the OpenAPI spec (the SDK already carries these as
doc-comments). `*`-required params become required clap args.

> **Why not clap's `flatten_help(true)`?** It only flattens **one** level — it
> reveals a group's direct subcommands but not the leaves beneath a 3-level path
> (`reddit subreddits get`). Verified against clap 4.6. The generated
> `after_help` listing is the reliable way to show the full subtree, and gives
> nicer one-line-per-command output.

- [ ] Generate, per platform/group command, an `after_help` listing all descendant leaves (`cmd <args>  summary`), from the descriptor table
- [ ] Every command has a one-line `about` (from `summary`) and long `about` (from `description`)
- [ ] Every flag has help (from parameter `description`), shows default + allowed values where the spec lists them
- [ ] Root `scrapebadger --help` lists platforms + a one-line endpoint count each; `--help-all` dumps the entire tree

### 1b. Does docs.rs cover this? (the SDK yes, the CLI partially)
docs.rs builds **library** API docs from doc-comments — it documents
`ScrapeBadger`, `client.reddit().get_subreddit_posts(...)`, the `*Params`
structs, etc. It does **not** render CLI `--help`, man pages, or binary
behaviour. So:
- **Library**: docs.rs is the right home and already works for free on publish
  (the generated methods carry doc-comments). Add `#![doc = include_str!("../../README.md")]` at the crate root for a good landing page.
- **CLI**: docs.rs won't show the command tree on its own — but we can surface it
  there by embedding the generated CLI reference into rustdoc:
  ```rust
  /// Full CLI command reference (generated from specs).
  #[doc = include_str!("../../docs/CLI.md")]
  pub mod cli_reference {}
  ```
  Then the CLI reference appears as a page on docs.rs alongside the SDK API, and
  can never drift (it's the same generated `docs/CLI.md` from layer 5).

- [x] docs.rs metadata in `Cargo.toml` (`all-features`, `--cfg docsrs`); `docs/CLI.md` added to packaged `include`
- [x] `#[doc = include_str!("../docs/CLI.md")]` on a `cli_reference` module so the CLI tree shows on docs.rs (generated, 137 commands; builds clean on nightly)
- [ ] Crate-root README landing page polish (optional `#![doc = include_str!("README.md")]`)
- [ ] Move `docs/CLI.md` generation into `xtask gen-docs` (currently generated by script — see Phase 4)

### 2. `commands` — the full flat tree (grep-able)
A `scrapebadger commands [--platform reddit]` subcommand prints every leaf command
on one line each (this file's tables, generated at runtime) so users can `grep`:
```
scrapebadger commands | grep subreddit
```
- [ ] `commands` subcommand walks the descriptor table and prints `cmd <args>  # summary`

### 3. `--explain` / `--curl` — show the underlying request
Any command accepts `--explain` to print the resolved HTTP request instead of
sending it (great for learning the API and for docs):
```
scrapebadger reddit subreddits posts sneakers --sort new --explain
#   GET https://api.scrapebadger.com/v1/reddit/subreddits/sneakers/posts?sort=new
scrapebadger reddit subreddits posts sneakers --sort new --curl   # emits a runnable curl
```
- [ ] `--explain` (resolved method+URL+query+body, no network call)
- [ ] `--curl` (equivalent `curl` command, API key redacted unless `--reveal`)

### 4. Shell completions
`clap_complete` generates completions for the whole tree for free:
```
scrapebadger completions zsh > ~/.zfunc/_scrapebadger
```
- [ ] `completions <bash|zsh|fish|powershell|elvish>` subcommand (`clap_complete`)

### 5. Generated reference docs (man pages + markdown)
An `xtask gen-docs` target regenerates, from the same descriptor table:
- man pages via `clap_mangen` (`man scrapebadger-reddit-subreddits-posts`)
- a `docs/CLI.md` (this file's tables) so the repo docs never drift from the specs
- [ ] `xtask gen-docs` emits `docs/CLI.md` + `man/*.1`
- [ ] CI check fails if `docs/CLI.md` is stale vs specs (same pattern as the SDK codegen check)

## Implementation plan

The CLI tree is **generated from the specs**, not hand-written — same source of
truth as the SDK, so it can never drift to 137 hand-maintained subcommands.

### Phase 1 — descriptor table (in `xtask`)
- [ ] Extend `xtask` to emit `crates/scrapebadger/src/cli/generated.rs` with a `const ENDPOINTS: &[Endpoint]`
- [ ] `Endpoint { platform, command_path: &[&str], method, path_template, path_params: &[&str], query: &[QueryParam], has_body, summary, description }`
- [ ] `QueryParam { name, flag, ty, required, default, help, allowed: Option<&[&str]> }` (allowed parsed from `description` enums where present)
- [ ] Reuse the existing `naming.rs` rules; add the command-path derivation (static→subcommand, param→positional, leaf verbs, CRUD verbs, runnable-parent `get`)
- [ ] Unit test: 137 endpoints, 0 duplicate command paths, 0 runnable-parents (asserts this plan's invariants)

### Phase 2 — runtime command tree (in `main.rs`)
- [ ] Build the clap `Command` tree from `ENDPOINTS` at startup (clap builder API)
- [ ] Generic dispatcher: match the selected leaf back to its `Endpoint`, bind positionals→path template, flags→query, `--body`→JSON body
- [ ] Execute via the existing `client.client().send(method, &path, &query, body)`
- [ ] Map booleans/ints per `QueryParam.ty`; required-but-missing → clap error
- [ ] Keep `raw` and `config`; keep typed shortcuts? (see open questions)

### Phase 3 — output & UX
- [ ] `-o, --output <json|jsonl|raw>` (pretty JSON default)
- [ ] `--select <path>` field projection (e.g. `.posts[].title`)
- [ ] `--all` auto-follows the `pagination.after` cursor until exhausted
- [ ] `--explain` / `--curl` (doc layer 3)
- [ ] Non-zero exit + clean stderr on API errors (already partly done)

### Phase 4 — docs surface
- [ ] Generated `after_help` full-subtree listing on every platform/group command (doc layer 1) — the "see everything at `reddit --help`" feature
- [ ] `--help-all` root flag dumps the entire tree (doc layer 1)
- [ ] `commands` subcommand (doc layer 2)
- [ ] `completions` subcommand (doc layer 4)
- [ ] `xtask gen-docs` → `docs/CLI.md` + man pages (doc layer 5)
- [ ] `#[doc = include_str!]` `docs/CLI.md` into a `cli_reference` module + crate-root README so the CLI tree shows on docs.rs (doc layer 1b)
- [ ] Rich `--help` wiring verified at all depths (doc layer 1)

### Phase 5 — migration & back-compat
- [ ] Remove the 8 current flat shortcuts (clean break — decision 2) and add the old→new mapping table to the changelog
- [ ] Update root `README.md` + `crates/scrapebadger/README.md` examples to the nested form
- [ ] Bump version; note CLI change in changelog

## Decisions (resolved)

1. **Body handling for POST/PATCH (10 endpoints): support _both_ methods.**
   Every body endpoint accepts `--body '<json>'` **and** generated typed scalar
   `--flags`; the two are merged with **flags overriding `--body`**. Non-scalar
   body fields (arrays/objects like `js_scenario`, `custom_headers`) are reachable
   via `--body` (or repeatable `--field k=v` for scalar arrays such as `usernames`).
   PATCH endpoints get the same flags, all optional, sent only when set.
   - [ ] Generate typed scalar body flags into the descriptor table (`body: &[BodyField]`)
   - [ ] Dispatcher merges `--body` JSON with typed flags (flags win) before send
   - [ ] `--body-file <path>` convenience for large bodies (e.g. `web scrape`)

2. **Back-compat: clean break.** Drop the 8 flat shortcuts (`account`, `scrape`,
   `flights`, `google-search`, `amazon-search`, `amazon-product`, `twitter-search`,
   `reddit-post`). Pre-1.0, no alias baggage. Ship an old→new mapping table.
   - [ ] Remove the 8 shortcuts; add the migration mapping table to changelog + README

3. **Action naming: kebab-case, with hidden underscore aliases.** Commands use
   kebab (`twitter tweets advanced-search`) to match the already-kebab `--flags`;
   the underscore form (`advanced_search`) is registered as a **hidden alias** so
   copy-paste from the API/SDK still resolves. Affects 10 Twitter commands.
   - [ ] Kebab-case action segments in the descriptor table
   - [ ] Register underscore originals as `hide`-d clap aliases

4. **`web scrape` (and `web detect`): typed convenience flags.** The headline
   endpoint gets typed scalar flags (`--url`, `--render-js`, `--format`,
   `--screenshot`, `--country`, …) as the first application of decision 1 —
   replacing today's `scrape` shortcut. `--url` may also be a positional.
   - [ ] `web scrape <url|--url> [scalar flags]` + `--body` for `js_scenario`/`custom_headers`
   - [ ] `web detect <url|--url> [--timeout] [--country]`
