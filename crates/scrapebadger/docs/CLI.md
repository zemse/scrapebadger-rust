# scrapebadger CLI — command reference

All **207** endpoints as nested subcommands, generated from `specs/*.json` — the same source as the SDK. The general shape is:

```text
scrapebadger PLATFORM GROUP [SUBGROUP] ACTION [IDS...] [--flags]
```

An `<arg>` below is a required positional; run `scrapebadger <command> --help` for full per-flag help, or `scrapebadger <platform> --help` to list every command for that platform at once.

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

## Google

`scrapebadger google` — 35 endpoints

```text
google ai-mode search           Google AI Mode search
google ai-overview              Google AI Overview (inline SERP block)
google autocomplete             Google search suggestions
google finance quote            Get stock/index quote
google flights search           Google Flights search
google hotels details           Hotel details
google hotels search            Search hotels
google images search            Search Google Images
google jobs search              Search Google Jobs
google lens search              Google Lens visual search
google maps photos              Get place photos
google maps place               Get place details
google maps posts               Get business posts
google maps reviews             Get place reviews
google maps search              Search Google Maps places
google news search              Search Google News
google news topics              News by topic
google news trending            Trending news
google patents detail           Patent details
google patents search           Search patents
google products detail          Immersive product detail
google scholar author citation  Get author citations-per-year chart
google scholar author get       Get Scholar author profile
google scholar cite             Get citation formats for a Scholar paper
google scholar profiles         Search Scholar author profiles
google scholar search           Search Google Scholar
google search                   Google web search
google shopping search          Search products
google shorts search            Google Shorts search
google trends autocomplete      Trends topic autocomplete
google trends interest          Interest over time
google trends regions           Interest by region
google trends related           Related topics & queries
google trends trending          Trending searches
google videos search            Search Google Videos
```

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

## Twitter / X

`scrapebadger twitter` — 53 endpoints

```text
twitter communities get <community_id>           Get community details
twitter communities search                       Search communities
twitter communities tweets <community_id>        Get community tweets
twitter geo places get <place_id>                Get place details
twitter geo search                               Search places
twitter lists detail <list_id>                   Get list details
twitter lists search-tweets <list_id>            Search list tweets
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
twitter tweets advanced-search                   Advanced tweet search
twitter tweets article get <article_id>          Get article by ID
twitter tweets get                               Get tweets by IDs
twitter tweets tweet community-notes <tweet_id>  Get community notes
twitter tweets tweet edit-history <tweet_id>     Get tweet edit history
twitter tweets tweet favoriters <tweet_id>       Get tweet favoriters
twitter tweets tweet get <tweet_id>              Get tweet details
twitter tweets tweet quotes <tweet_id>           Get tweet quotes
twitter tweets tweet replies <tweet_id>          Get tweet replies
twitter tweets tweet retweeters <tweet_id>       Get tweet retweeters
twitter tweets tweet similar <tweet_id>          Get similar tweets
twitter users articles <user_id>                 Get user articles
twitter users batch-by-ids                       Batch get users by IDs
twitter users batch-by-usernames                 Batch get users by usernames
twitter users by-id <user_id>                    Get user by ID
twitter users by-username <username>             Get user by username
twitter users followers <username>               Get user followers
twitter users followings <username>              Get user following
twitter users latest-tweets <username>           Get user tweets
twitter users mentions <username>                Get user mentions
twitter users search-users                       Search users
twitter users subscriptions <user_id>            Get user subscriptions
```

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

## Web Scraping

`scrapebadger web` — 2 endpoints

```text
web detect  Detect Protection
web scrape  Scrape URL
```
