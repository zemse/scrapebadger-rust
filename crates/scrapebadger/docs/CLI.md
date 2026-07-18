# scrapebadger CLI — command reference

All **269** endpoints as nested subcommands, generated from `specs/*.json` — the same source as the SDK. The general shape is:

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

## Depop

`scrapebadger depop` — 5 endpoints

```text
depop markets                    List markets
depop products get <product_id>  Get product detail
depop search                     Search Depop products
depop users get <username>       Get shop/user profile
depop users products <username>  Get a user's products
```

## eBay

`scrapebadger ebay` — 11 endpoints

```text
ebay autocomplete                    Keyword suggestions
ebay categories get                  List categories
ebay categories items <category_id>  Browse a category
ebay completed                       Completed / sold listings
ebay items get <item_id>             Get item detail
ebay items reviews <item_id>         Get item reviews
ebay markets                         List markets
ebay search                          Search listings
ebay sellers feedback <username>     Get seller feedback
ebay sellers get <username>          Get seller profile
ebay sellers items <username>        Get seller listings
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

## Idealista

`scrapebadger idealista` — 8 endpoints

```text
idealista agency by-phone get <phone>       Agency by phone
idealista agency get <short_name>           Agency profile + listings
idealista markets                           List markets
idealista properties get <property_code>    Get property detail
idealista properties stats <property_code>  Get listing engagement stats
idealista search all                        Search all (beats result cap)
idealista search get                        Search listings
idealista suggest                           Resolve locations
```

## Immobiliare

`scrapebadger immobiliare` — 8 endpoints

```text
immobiliare agencies get <agency_id>       Get agency profile
immobiliare agencies listings <agency_id>  Get an agency's listings
immobiliare autocomplete                   Location autocomplete
immobiliare listings get <listing_id>      Get listing detail
immobiliare market-insights prices         Price €/m² time series
immobiliare markets                        List markets
immobiliare reference                      List filter enums
immobiliare search                         Search listings
```

## Leboncoin

`scrapebadger leboncoin` — 10 endpoints

```text
leboncoin ads get <list_id>           Get ad detail
leboncoin ads similar <list_id>       Get similar ads
leboncoin categories                  List categories
leboncoin departments                 List departments
leboncoin locations search            Location autocomplete
leboncoin markets                     List markets
leboncoin regions                     List regions
leboncoin search                      Search Leboncoin ads
leboncoin sellers get <user_id>       Get seller profile
leboncoin sellers listings <user_id>  Get a seller's ads
```

## LinkedIn

`scrapebadger linkedin` — 10 endpoints

```text
linkedin articles get <article_slug>     Get a public article
linkedin companies get <universal_name>  Get company
linkedin companies jobs <company_id>     Get a company's job postings
linkedin geo suggest                     Suggest location geo ids
linkedin jobs get <job_id>               Get job detail
linkedin jobs search                     Search LinkedIn jobs
linkedin learning get <course_slug>      Get a course
linkedin posts get <post_slug>           Get a public post
linkedin profiles get <public_id>        Get public profile
linkedin schools get <universal_name>    Get school
```

## LoopNet

`scrapebadger loopnet` — 5 endpoints

```text
loopnet brokers get <slug> <broker_id>  Get broker profile
loopnet listings get <listing_id>       Get listing detail
loopnet markets                         List coverage markets
loopnet property-types                  List property types
loopnet search                          Search commercial real estate
```

## Realtor

`scrapebadger realtor` — 4 endpoints

```text
realtor autocomplete                  Location autocomplete
realtor markets                       List markets
realtor properties get <property_id>  Get full property detail
realtor search                        Search property listings
```

## Redfin

`scrapebadger redfin` — 6 endpoints

```text
redfin agent                       Get agent profile + listings
redfin autocomplete                Region/address suggestions
redfin markets                     List coverage markets
redfin property get <property_id>  Get property detail
redfin property list               Get property detail by URL
redfin search                      Search properties
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

## TikTok

`scrapebadger tiktok` — 25 endpoints

```text
tiktok ads search                     Search the TikTok Ad Library
tiktok comments replies <comment_id>  Get comment replies
tiktok hashtags get <name>            Get hashtag detail
tiktok hashtags videos <name>         Get hashtag videos
tiktok music get <music_id>           Get music/sound detail
tiktok music videos <music_id>        Get music videos
tiktok oembed                         Get oEmbed metadata
tiktok regions                        List regions
tiktok search get                     General search
tiktok search hashtags                Search hashtags
tiktok search users                   Search users
tiktok search videos                  Search videos
tiktok trending hashtags              Trending hashtags
tiktok trending songs                 Trending songs
tiktok trending videos                Trending videos
tiktok users followers <username>     Get followers (deprecated)
tiktok users following <username>     Get following (deprecated)
tiktok users get <username>           Get user profile
tiktok users liked <username>         Get liked videos (deprecated)
tiktok users reposts <username>       Get reposts
tiktok users videos <username>        Get user videos
tiktok videos comments <video_id>     Get comments
tiktok videos get <video_id>          Get video detail
tiktok videos related <video_id>      Get related videos
tiktok videos transcript <video_id>   Get transcript
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

## YouTube

`scrapebadger youtube` — 38 endpoints

```text
youtube autocomplete                                     Keyword suggestions
youtube categories                                       Video categories
youtube channels about <channel_id>                      Channel about
youtube channels community <channel_id>                  Community posts
youtube channels get <channel_id>                        Get channel detail
youtube channels playlists <channel_id>                  Channel playlists
youtube channels resolve                                 Resolve handle/URL to id
youtube channels search <channel_id>                     Search within a channel
youtube channels shorts <channel_id>                     Channel shorts
youtube channels streams <channel_id>                    Channel streams
youtube channels subscriber-count <channel_id>           Subscriber count (fast)
youtube channels videos <channel_id>                     Channel videos
youtube hashtags get <tag>                               Videos under a hashtag
youtube home                                             Guest home feed
youtube languages                                        UI languages
youtube markets                                          Supported markets
youtube mixes get <playlist_id>                          Get a mix / radio queue
youtube music search                                     Search YouTube Music
youtube oembed                                           oEmbed metadata
youtube playlists get <playlist_id>                      Get playlist detail
youtube playlists items <playlist_id>                    Playlist items page
youtube posts comments <post_id>                         Community post comments
youtube posts get <post_id>                              Get a community post
youtube regions                                          Content regions
youtube search                                           Search YouTube
youtube shorts by-sound get <sound_id>                   Shorts by sound
youtube shorts get <video_id>                            Get a Short
youtube trending get                                     Trending videos
youtube trending shorts                                  Trending shorts
youtube videos batch                                     Batch video detail
youtube videos captions <video_id>                       List caption tracks
youtube videos comments get <video_id>                   Video comments
youtube videos comments replies <video_id> <comment_id>  Comment replies
youtube videos get <video_id>                            Get video detail
youtube videos live-chat <video_id>                      Live chat messages
youtube videos related <video_id>                        Related videos
youtube videos streams <video_id>                        Stream formats
youtube videos transcript <video_id>                     Video transcript
```

## Zillow

`scrapebadger zillow` — 6 endpoints

```text
zillow agent                Get agent profile + listings
zillow autocomplete         Region/address suggestions
zillow markets              List coverage markets
zillow property get <zpid>  Get property detail
zillow property list        Get property detail by URL
zillow search               Search properties
```
