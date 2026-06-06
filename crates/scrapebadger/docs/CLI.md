# scrapebadger CLI — command reference

All **137** endpoints as nested subcommands: `scrapebadger <platform> <group> [<subgroup>] <action> [<ids>] [--flags]`. Generated from `specs/*.json` — the same source as the SDK.

`*` marks a required flag. `<arg>` is a required positional. Run `scrapebadger <command> --help` for full per-flag help, or `scrapebadger <platform> --help` to list every command for that platform at once.

**Platforms:** [Account](#account) · [Amazon](#amazon) · [Google](#google) · [Reddit](#reddit) · [Twitter / X](#twitterxx) · [Vinted](#vinted) · [Web Scraping](#web-scraping)


## Account

`scrapebadger account` — 1 endpoints

```text
account me  Get Account Info
```

## Amazon

`scrapebadger amazon` — 14 endpoints

```text
amazon autocomplete                  Keyword Suggestions
amazon bestsellers                   Bestsellers
amazon categories                    List Categories
amazon category                      Browse Category
amazon deals                         Today's Deals
amazon markets                       List Markets
amazon new-releases                  New Releases
amazon products get <asin>           Get Product Detail
amazon products offers <asin>        Get Offers
amazon products reviews <asin>       Get Reviews
amazon search                        Search Products
amazon sellers feedback <seller_id>  Seller Feedback
amazon sellers get <seller_id>       Get Seller
amazon sellers products <seller_id>  Seller Products
```

<details><summary>flags & HTTP mapping</summary>

| Command | Args | Flags | Method & path |
|---|---|---|---|
| `amazon autocomplete` | — | `--query`* `--domain` | `GET /v1/amazon/autocomplete` |
| `amazon bestsellers` | — | `--domain` `--category` `--page` | `GET /v1/amazon/bestsellers` |
| `amazon categories` | — | `--domain` | `GET /v1/amazon/categories` |
| `amazon category` | — | `--node`* `--domain` `--page` `--sort-by` | `GET /v1/amazon/category` |
| `amazon deals` | — | `--domain` `--category` `--page` | `GET /v1/amazon/deals` |
| `amazon markets` | — | — | `GET /v1/amazon/markets` |
| `amazon new-releases` | — | `--domain` `--category` `--page` | `GET /v1/amazon/new-releases` |
| `amazon products get` | `<asin>` | `--domain` `--zip` `--language` | `GET /v1/amazon/products/{asin}` |
| `amazon products offers` | `<asin>` | `--domain` `--zip` | `GET /v1/amazon/products/{asin}/offers` |
| `amazon products reviews` | `<asin>` | `--domain` `--page` `--sort-by` `--star` `--verified-only` `--media-only` | `GET /v1/amazon/products/{asin}/reviews` |
| `amazon search` | — | `--query`* `--domain` `--page` `--sort-by` `--category` `--min-price` `--max-price` `--zip` `--language` | `GET /v1/amazon/search` |
| `amazon sellers feedback` | `<seller_id>` | `--domain` `--page` | `GET /v1/amazon/sellers/{seller_id}/feedback` |
| `amazon sellers get` | `<seller_id>` | `--domain` | `GET /v1/amazon/sellers/{seller_id}` |
| `amazon sellers products` | `<seller_id>` | `--domain` `--page` | `GET /v1/amazon/sellers/{seller_id}/products` |

</details>

## Google

`scrapebadger google` — 39 endpoints

```text
google ai-mode search           Google AI Mode search
google autocomplete             Google search suggestions
google finance quote            Get Google Finance stock quote
google flights search           Google Flights search
google hotels details           Hotel details
google hotels search            Search Google Hotels
google images search            Search Google Images
google jobs search              Search Google Jobs
google lens search              Google Lens visual search
google local search             Google Local Pack search
google maps photos              Get place photos
google maps place               Get place details
google maps posts               Get business posts
google maps reviews             Get place reviews
google maps search              Search Google Maps places
google news search              Search Google News
google news topics              News by topic
google news trending            Trending news
google patents detail           Get patent details
google patents search           Search Google Patents
google products detail          Google immersive product detail
google scholar author citation  Get citation chart data for a Scholar author
google scholar author get       Get Google Scholar author profile
google scholar cite             Get citation formats for a Scholar paper
google scholar profiles         Search Google Scholar author profiles
google scholar search           Search Google Scholar
google search                   Google web search
google shopping product click   Resolve merchant URL for a Google Shopping product
google shopping product get     Product details
google shopping search          Search Google Shopping
google shorts search            Google Shorts search
google trends autocomplete      Trends topic autocomplete
google trends interest          Interest over time
google trends regions           Interest by region
google trends related           Related topics & queries
google trends search            Google Trends — unified search
google trends trending          Trending searches
google trends trending-now      Google Trends — current trending searches
google videos search            Search Google Videos
```

<details><summary>flags & HTTP mapping</summary>

| Command | Args | Flags | Method & path |
|---|---|---|---|
| `google ai-mode search` | — | `--q`* `--gl` `--hl` | `GET /api/v1/ai-mode/search` |
| `google autocomplete` | — | `--q`* `--hl` `--gl` | `GET /api/v1/autocomplete/` |
| `google finance quote` | — | `--q`* `--hl` | `GET /api/v1/finance/quote` |
| `google flights search` | — | `--departure-id`* `--arrival-id`* `--outbound-date`* `--return-date` `--trip-type` `--adults` `--children` `--infants-in-seat` `--infants-on-lap` `--travel-class` `--currency` `--gl` `--hl` `--stops` `--max-price` | `GET /api/v1/flights/search` |
| `google hotels details` | — | `--property-token`* `--check-in`* `--check-out`* `--adults` `--currency` `--gl` `--hl` | `GET /api/v1/hotels/details` |
| `google hotels search` | — | `--q`* `--check-in`* `--check-out`* `--adults` `--children` `--currency` `--gl` `--hl` `--sort-by` `--min-price` `--max-price` `--hotel-class` `--next-page-token` | `GET /api/v1/hotels/search` |
| `google images search` | — | `--q`* `--gl` `--hl` `--tbs` `--imgsz` `--imgcolor` `--imgtype` `--safe` `--page` | `GET /api/v1/images/search` |
| `google jobs search` | — | `--q`* `--location` `--gl` `--country` `--hl` `--language` `--domain` `--job-type` `--date-posted` `--ltype` `--chips` `--uds` `--uule` `--lrad` `--next-page-token` `--mode` | `GET /api/v1/jobs/search` |
| `google lens search` | — | `--url`* `--gl` `--hl` | `GET /api/v1/lens/search` |
| `google local search` | — | `--q`* `--gl` `--hl` `--domain` `--location` `--uule` `--num` `--start` | `GET /api/v1/local/search` |
| `google maps photos` | — | `--data-id`* `--hl` `--gl` | `GET /api/v1/maps/photos` |
| `google maps place` | — | `--place-id` `--data-id` `--hl` `--gl` | `GET /api/v1/maps/place` |
| `google maps posts` | — | `--data-id` `--place-id` `--hl` `--gl` | `GET /api/v1/maps/posts` |
| `google maps reviews` | — | `--data-id`* `--sort-by` `--hl` `--gl` `--next-page-token` `--offset` `--results` | `GET /api/v1/maps/reviews` |
| `google maps search` | — | `--q`* `--ll` `--gl` `--hl` `--start` | `GET /api/v1/maps/search` |
| `google news search` | — | `--q`* `--hl` `--gl` `--max-results` | `GET /api/v1/news/search` |
| `google news topics` | — | `--topic`* `--hl` `--gl` `--max-results` | `GET /api/v1/news/topics` |
| `google news trending` | — | `--hl` `--gl` `--max-results` | `GET /api/v1/news/trending` |
| `google patents detail` | — | `--patent-id`* | `GET /api/v1/patents/detail` |
| `google patents search` | — | `--q`* `--page` `--num` `--sort` `--inventor` `--assignee` `--country` `--language` `--status` `--patent-type` `--before` `--after` | `GET /api/v1/patents/search` |
| `google products detail` | — | `--product-id`* `--q`* `--gl` `--hl` `--domain` `--include-offers` `--include-variants` `--resolve-deep-urls` | `GET /api/v1/products/detail` |
| `google scholar author citation` | — | `--author-id`* `--hl` | `GET /api/v1/scholar/author/citation` |
| `google scholar author get` | — | `--author-id`* `--hl` `--cstart` `--pagesize` | `GET /api/v1/scholar/author` |
| `google scholar cite` | — | `--q`* `--hl` | `GET /api/v1/scholar/cite` |
| `google scholar profiles` | — | `--mauthors`* `--hl` `--after-author` `--before-author` | `GET /api/v1/scholar/profiles` |
| `google scholar search` | — | `--q`* `--hl` `--as-ylo` `--as-yhi` `--as-sdt` `--page` `--num` | `GET /api/v1/scholar/search` |
| `google search` | — | `--q`* `--gl` `--hl` `--num` `--start` `--domain` `--device` `--location` `--lr` `--tbs` `--safe` `--uule` `--filter` `--nfpr` `--cr` `--ludocid` `--lsig` `--kgmid` `--si` `--ibp` `--uds` `--ai-overview` `--mode` | `GET /api/v1/search` |
| `google shopping product click` | — | `--title`* `--source` `--q` `--product-id` `--gl` `--hl` | `GET /api/v1/shopping/product/click` |
| `google shopping product get` | — | `--product-id`* `--gl` `--hl` | `GET /api/v1/shopping/product` |
| `google shopping search` | — | `--q`* `--gl` `--hl` `--min-price` `--max-price` `--sort-by` `--free-shipping` `--on-sale` `--start` | `GET /api/v1/shopping/search` |
| `google shorts search` | — | `--q`* `--gl` `--hl` `--domain` `--num` `--start` | `GET /api/v1/shorts/search` |
| `google trends autocomplete` | — | `--q`* `--hl` `--tz` | `GET /api/v1/trends/autocomplete` |
| `google trends interest` | — | `--q`* `--geo` `--date` `--category` `--gprop` | `GET /api/v1/trends/interest` |
| `google trends regions` | — | `--q`* `--geo` `--date` `--resolution` | `GET /api/v1/trends/regions` |
| `google trends related` | — | `--q`* `--geo` `--date` | `GET /api/v1/trends/related` |
| `google trends search` | — | `--q`* `--data-type` `--geo` `--date` `--cat` `--gprop` `--region` `--language` `--tz` | `GET /api/v1/trends/search` |
| `google trends trending` | — | `--geo` `--hl` `--hours` | `GET /api/v1/trends/trending` |
| `google trends trending-now` | — | `--geo` `--hours` `--category` `--status` `--sort` `--hl` | `GET /api/v1/trends/trending-now` |
| `google videos search` | — | `--q`* `--gl` `--hl` `--tbs` `--safe` `--page` | `GET /api/v1/videos/search` |

</details>

## Reddit

`scrapebadger reddit` — 20 endpoints

```text
reddit domains posts <domain>                  Get posts by domain
reddit posts comments <post_id>                Get post comments
reddit posts duplicates <post_id>              Get cross-posts
reddit posts get <post_id>                     Get post detail
reddit posts trending                          Get trending posts
reddit search posts                            Search Reddit posts
reddit search subreddits                       Search subreddits
reddit search users                            Search users
reddit subreddits get <subreddit>              Get subreddit info
reddit subreddits new                          New subreddits
reddit subreddits popular                      Popular subreddits
reddit subreddits posts <subreddit>            Get subreddit posts
reddit subreddits rules <subreddit>            Get subreddit rules
reddit subreddits wiki get <subreddit> <page>  Get wiki page content
reddit subreddits wiki list <subreddit>        List wiki pages
reddit users comments <username>               Get user's comments
reddit users get <username>                    Get user profile
reddit users moderated <username>              Get user's moderated subreddits
reddit users posts <username>                  Get user's posts
reddit users trophies <username>               Get user's trophies
```

<details><summary>flags & HTTP mapping</summary>

| Command | Args | Flags | Method & path |
|---|---|---|---|
| `reddit domains posts` | `<domain>` | `--sort` `--t` `--limit` `--after` | `GET /v1/reddit/domains/{domain}/posts` |
| `reddit posts comments` | `<post_id>` | `--sort` `--limit` `--depth` | `GET /v1/reddit/posts/{post_id}/comments` |
| `reddit posts duplicates` | `<post_id>` | `--limit` `--after` | `GET /v1/reddit/posts/{post_id}/duplicates` |
| `reddit posts get` | `<post_id>` | — | `GET /v1/reddit/posts/{post_id}` |
| `reddit posts trending` | — | `--sort` `--t` `--limit` `--after` | `GET /v1/reddit/posts/trending` |
| `reddit search posts` | — | `--q`* `--subreddit` `--sort` `--t` `--limit` `--after` | `GET /v1/reddit/search/posts` |
| `reddit search subreddits` | — | `--q`* `--limit` `--after` | `GET /v1/reddit/search/subreddits` |
| `reddit search users` | — | `--q`* `--limit` `--after` | `GET /v1/reddit/search/users` |
| `reddit subreddits get` | `<subreddit>` | — | `GET /v1/reddit/subreddits/{subreddit}` |
| `reddit subreddits new` | — | `--limit` `--after` | `GET /v1/reddit/subreddits/new` |
| `reddit subreddits popular` | — | `--limit` `--after` | `GET /v1/reddit/subreddits/popular` |
| `reddit subreddits posts` | `<subreddit>` | `--sort` `--t` `--limit` `--after` | `GET /v1/reddit/subreddits/{subreddit}/posts` |
| `reddit subreddits rules` | `<subreddit>` | — | `GET /v1/reddit/subreddits/{subreddit}/rules` |
| `reddit subreddits wiki get` | `<subreddit> <page>` | — | `GET /v1/reddit/subreddits/{subreddit}/wiki/{page}` |
| `reddit subreddits wiki list` | `<subreddit>` | — | `GET /v1/reddit/subreddits/{subreddit}/wiki` |
| `reddit users comments` | `<username>` | `--sort` `--t` `--limit` `--after` | `GET /v1/reddit/users/{username}/comments` |
| `reddit users get` | `<username>` | — | `GET /v1/reddit/users/{username}` |
| `reddit users moderated` | `<username>` | — | `GET /v1/reddit/users/{username}/moderated` |
| `reddit users posts` | `<username>` | `--sort` `--t` `--limit` `--after` | `GET /v1/reddit/users/{username}/posts` |
| `reddit users trophies` | `<username>` | — | `GET /v1/reddit/users/{username}/trophies` |

</details>

## Twitter / X

`scrapebadger twitter` — 53 endpoints

```text
twitter communities get <community_id>           Get community details
twitter communities search                       Search communities
twitter communities tweets <community_id>        Get community tweets
twitter geo places get <place_id>                Get place details
twitter geo search                               Search places
twitter lists detail <list_id>                   Get list details
twitter lists search_tweets <list_id>            Search list tweets
twitter lists tweets <list_id>                   Get list tweets
twitter spaces broadcast get <broadcast_id>      Get broadcast details
twitter spaces get <space_id>                    Get Space details
twitter stream billing-logs                      List billing logs
twitter stream filter-rules create               Create filter rule
twitter stream filter-rules delete <rule_id>     Delete filter rule
twitter stream filter-rules get <rule_id>        Get filter rule
twitter stream filter-rules list                 List filter rules
twitter stream filter-rules logs <rule_id>       Get filter rule delivery logs
twitter stream filter-rules update <rule_id>     Update filter rule
twitter stream filter-rules validate             Validate filter rule query
twitter stream filter-rules-pricing              Get filter rule pricing tiers
twitter stream logs                              List delivery logs
twitter stream monitors create                   Create stream monitor
twitter stream monitors delete <monitor_id>      Delete stream monitor
twitter stream monitors get <monitor_id>         Get stream monitor
twitter stream monitors list                     List stream monitors
twitter stream monitors update <monitor_id>      Update stream monitor
twitter stream webhooks create                   Create webhook
twitter stream webhooks delete <webhook_id>      Delete webhook
twitter stream webhooks list                     List webhooks
twitter stream webhooks test                     Test webhook
twitter trends get                               Get trending topics
twitter trends place get <woeid>                 Get trends by location
twitter tweets advanced_search                   Advanced tweet search
twitter tweets article get <article_id>          Get article by ID
twitter tweets get                               Get tweets by IDs
twitter tweets tweet community_notes <tweet_id>  Get community notes
twitter tweets tweet edit_history <tweet_id>     Get tweet edit history
twitter tweets tweet favoriters <tweet_id>       Get tweet favoriters
twitter tweets tweet get <tweet_id>              Get tweet details
twitter tweets tweet quotes <tweet_id>           Get tweet quotes
twitter tweets tweet replies <tweet_id>          Get tweet replies
twitter tweets tweet retweeters <tweet_id>       Get tweet retweeters
twitter tweets tweet similar <tweet_id>          Get similar tweets
twitter users articles <user_id>                 Get user articles
twitter users batch_by_ids                       Batch get users by IDs
twitter users batch_by_usernames                 Batch get users by usernames
twitter users by_id <user_id>                    Get user by ID
twitter users by_username <username>             Get user by username
twitter users followers <username>               Get user followers
twitter users followings <username>              Get user following
twitter users latest_tweets <username>           Get user tweets
twitter users mentions <username>                Get user mentions
twitter users search_users                       Search users
twitter users subscriptions <user_id>            Get user subscriptions
```

<details><summary>flags & HTTP mapping</summary>

| Command | Args | Flags | Method & path |
|---|---|---|---|
| `twitter communities get` | `<community_id>` | — | `GET /v1/twitter/communities/{community_id}` |
| `twitter communities search` | — | `--query`* `--cursor` | `GET /v1/twitter/communities/search` |
| `twitter communities tweets` | `<community_id>` | `--tweet-type` `--cursor` | `GET /v1/twitter/communities/{community_id}/tweets` |
| `twitter geo places get` | `<place_id>` | — | `GET /v1/twitter/geo/places/{place_id}` |
| `twitter geo search` | — | `--query` `--lat` `--long` | `GET /v1/twitter/geo/search` |
| `twitter lists detail` | `<list_id>` | — | `GET /v1/twitter/lists/{list_id}/detail` |
| `twitter lists search_tweets` | `<list_id>` | `--query`* `--cursor` | `GET /v1/twitter/lists/{list_id}/search_tweets` |
| `twitter lists tweets` | `<list_id>` | `--cursor` | `GET /v1/twitter/lists/{list_id}/tweets` |
| `twitter spaces broadcast get` | `<broadcast_id>` | — | `GET /v1/twitter/spaces/broadcast/{broadcast_id}` |
| `twitter spaces get` | `<space_id>` | — | `GET /v1/twitter/spaces/{space_id}` |
| `twitter stream billing-logs` | — | `--monitor-id` `--page` `--page-size` | `GET /v1/twitter/stream/billing-logs` |
| `twitter stream filter-rules create` | — | `--body` | `POST /v1/twitter/stream/filter-rules` |
| `twitter stream filter-rules delete` | `<rule_id>` | — | `DELETE /v1/twitter/stream/filter-rules/{rule_id}` |
| `twitter stream filter-rules get` | `<rule_id>` | — | `GET /v1/twitter/stream/filter-rules/{rule_id}` |
| `twitter stream filter-rules list` | — | `--page` `--page-size` `--status` | `GET /v1/twitter/stream/filter-rules` |
| `twitter stream filter-rules logs` | `<rule_id>` | `--page` `--page-size` | `GET /v1/twitter/stream/filter-rules/{rule_id}/logs` |
| `twitter stream filter-rules update` | `<rule_id>` | `--body` | `PATCH /v1/twitter/stream/filter-rules/{rule_id}` |
| `twitter stream filter-rules validate` | — | `--body` | `POST /v1/twitter/stream/filter-rules/validate` |
| `twitter stream filter-rules-pricing` | — | — | `GET /v1/twitter/stream/filter-rules-pricing` |
| `twitter stream logs` | — | `--monitor-id` `--page` `--page-size` | `GET /v1/twitter/stream/logs` |
| `twitter stream monitors create` | — | `--body` | `POST /v1/twitter/stream/monitors` |
| `twitter stream monitors delete` | `<monitor_id>` | — | `DELETE /v1/twitter/stream/monitors/{monitor_id}` |
| `twitter stream monitors get` | `<monitor_id>` | — | `GET /v1/twitter/stream/monitors/{monitor_id}` |
| `twitter stream monitors list` | — | `--page` `--page-size` `--status` | `GET /v1/twitter/stream/monitors` |
| `twitter stream monitors update` | `<monitor_id>` | `--body` | `PATCH /v1/twitter/stream/monitors/{monitor_id}` |
| `twitter stream webhooks create` | — | `--body` | `POST /v1/twitter/stream/webhooks` |
| `twitter stream webhooks delete` | `<webhook_id>` | — | `DELETE /v1/twitter/stream/webhooks/{webhook_id}` |
| `twitter stream webhooks list` | — | `--monitor-id` | `GET /v1/twitter/stream/webhooks` |
| `twitter stream webhooks test` | — | `--body` | `POST /v1/twitter/stream/webhooks/test` |
| `twitter trends get` | — | `--category` `--count` | `GET /v1/twitter/trends/` |
| `twitter trends place get` | `<woeid>` | — | `GET /v1/twitter/trends/place/{woeid}` |
| `twitter tweets advanced_search` | — | `--query`* `--query-type` `--count` `--cursor` | `GET /v1/twitter/tweets/advanced_search` |
| `twitter tweets article get` | `<article_id>` | — | `GET /v1/twitter/tweets/article/{article_id}` |
| `twitter tweets get` | — | `--tweets`* | `GET /v1/twitter/tweets/` |
| `twitter tweets tweet community_notes` | `<tweet_id>` | — | `GET /v1/twitter/tweets/tweet/{tweet_id}/community_notes` |
| `twitter tweets tweet edit_history` | `<tweet_id>` | — | `GET /v1/twitter/tweets/tweet/{tweet_id}/edit_history` |
| `twitter tweets tweet favoriters` | `<tweet_id>` | `--cursor` | `GET /v1/twitter/tweets/tweet/{tweet_id}/favoriters` |
| `twitter tweets tweet get` | `<tweet_id>` | `--cursor` | `GET /v1/twitter/tweets/tweet/{tweet_id}` |
| `twitter tweets tweet quotes` | `<tweet_id>` | `--cursor` | `GET /v1/twitter/tweets/tweet/{tweet_id}/quotes` |
| `twitter tweets tweet replies` | `<tweet_id>` | `--cursor` | `GET /v1/twitter/tweets/tweet/{tweet_id}/replies` |
| `twitter tweets tweet retweeters` | `<tweet_id>` | `--cursor` | `GET /v1/twitter/tweets/tweet/{tweet_id}/retweeters` |
| `twitter tweets tweet similar` | `<tweet_id>` | — | `GET /v1/twitter/tweets/tweet/{tweet_id}/similar` |
| `twitter users articles` | `<user_id>` | `--cursor` | `GET /v1/twitter/users/{user_id}/articles` |
| `twitter users batch_by_ids` | — | `--user-ids`* | `GET /v1/twitter/users/batch_by_ids` |
| `twitter users batch_by_usernames` | — | `--usernames`* | `GET /v1/twitter/users/batch_by_usernames` |
| `twitter users by_id` | `<user_id>` | — | `GET /v1/twitter/users/{user_id}/by_id` |
| `twitter users by_username` | `<username>` | — | `GET /v1/twitter/users/{username}/by_username` |
| `twitter users followers` | `<username>` | `--cursor` | `GET /v1/twitter/users/{username}/followers` |
| `twitter users followings` | `<username>` | `--cursor` | `GET /v1/twitter/users/{username}/followings` |
| `twitter users latest_tweets` | `<username>` | `--cursor` | `GET /v1/twitter/users/{username}/latest_tweets` |
| `twitter users mentions` | `<username>` | `--count` `--cursor` | `GET /v1/twitter/users/{username}/mentions` |
| `twitter users search_users` | — | `--query`* `--cursor` | `GET /v1/twitter/users/search_users` |
| `twitter users subscriptions` | `<user_id>` | `--cursor` | `GET /v1/twitter/users/{user_id}/subscriptions` |

</details>

## Vinted

`scrapebadger vinted` — 8 endpoints

```text
vinted brands                 Search Brands
vinted colors                 List Colors
vinted items get <item_id>    Get Item Details
vinted markets                List Markets
vinted search                 Search Items
vinted statuses               List Conditions
vinted users get <user_id>    Get User Profile
vinted users items <user_id>  Get User Items
```

<details><summary>flags & HTTP mapping</summary>

| Command | Args | Flags | Method & path |
|---|---|---|---|
| `vinted brands` | — | `--query`* `--market` | `GET /v1/vinted/brands` |
| `vinted colors` | — | `--market` | `GET /v1/vinted/colors` |
| `vinted items get` | `<item_id>` | `--market` | `GET /v1/vinted/items/{item_id}` |
| `vinted markets` | — | — | `GET /v1/vinted/markets` |
| `vinted search` | — | `--query`* `--market` `--page` `--per-page` `--price-from` `--price-to` `--brand-ids` `--color-ids` `--status-ids` `--order` | `GET /v1/vinted/search` |
| `vinted statuses` | — | `--market` | `GET /v1/vinted/statuses` |
| `vinted users get` | `<user_id>` | `--market` | `GET /v1/vinted/users/{user_id}` |
| `vinted users items` | `<user_id>` | `--market` `--page` `--per-page` | `GET /v1/vinted/users/{user_id}/items` |

</details>

## Web Scraping

`scrapebadger web` — 2 endpoints

```text
web detect  Detect Protection
web scrape  Scrape URL
```

<details><summary>flags & HTTP mapping</summary>

| Command | Args | Flags | Method & path |
|---|---|---|---|
| `web detect` | — | `--body` | `POST /v1/web/detect` |
| `web scrape` | — | `--body` | `POST /v1/web/scrape` |

</details>
