#!/usr/bin/env python3
"""Collect live Reddit response samples for typed-model generation.

The Reddit OpenAPI spec does not type its 200 bodies, so the SDK returns
`serde_json::Value`. This script hits every Reddit endpoint once and saves the
JSON to `reddit_samples/` so typed structs can be reverse-engineered from real
data.

Usage:
    SCRAPEBADGER_API_KEY=sb_live_xxx python3 scripts/collect_reddit_samples.py

No third-party dependencies (urllib only). Safe to re-run; files are overwritten.
The API key is read from the environment and never written to disk.
"""

import json
import os
import sys
import time
import urllib.error
import urllib.parse
import urllib.request

BASE = os.environ.get("SCRAPEBADGER_BASE_URL", "https://scrapebadger.com")
KEY = os.environ.get("SCRAPEBADGER_API_KEY")
OUT = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "reddit_samples")

# Representative inputs. Tweak if any return empty/!200.
SUBREDDIT = "rust"
USERNAME = "spez"
DOMAIN = "github.com"
WIKI_PAGE = "config/sidebar"
QUERY = "rust"


REQUEST_DELAY = float(os.environ.get("REDDIT_SAMPLE_DELAY", "2.0"))
MAX_429_RETRIES = 6


def get(path, query=None):
    url = BASE.rstrip("/") + path
    if query:
        url += "?" + urllib.parse.urlencode(query)
    req = urllib.request.Request(url, headers={"x-api-key": KEY, "accept": "application/json"})
    attempt = 0
    while True:
        try:
            with urllib.request.urlopen(req, timeout=60) as resp:
                return json.loads(resp.read().decode("utf-8"))
        except urllib.error.HTTPError as e:
            if e.code == 429 and attempt < MAX_429_RETRIES:
                ra = e.headers.get("Retry-After")
                wait = int(ra) if (ra and ra.isdigit()) else min(2 ** attempt, 30)
                print(f"     429; waiting {wait}s (retry {attempt + 1}/{MAX_429_RETRIES})")
                time.sleep(wait)
                attempt += 1
                continue
            raise


def exists(name):
    return os.path.exists(os.path.join(OUT, name + ".json"))


def save(name, data):
    os.makedirs(OUT, exist_ok=True)
    with open(os.path.join(OUT, name + ".json"), "w") as f:
        json.dump(data, f, indent=2, ensure_ascii=False)
    print(f"  saved {name}.json")


def first_post_id(listing):
    """Best-effort dig a post id out of a listing response of unknown shape."""
    def walk(v):
        if isinstance(v, dict):
            for k in ("id", "post_id", "name", "fullname"):
                if isinstance(v.get(k), str) and v[k]:
                    return v[k]
            for vv in v.values():
                r = walk(vv)
                if r:
                    return r
        elif isinstance(v, list):
            for vv in v:
                r = walk(vv)
                if r:
                    return r
        return None
    return walk(listing)


def main():
    if not KEY:
        sys.exit("error: set SCRAPEBADGER_API_KEY in your environment")

    # Listing endpoints first; one of them gives us a real post id.
    sub_posts = None
    jobs = [
        ("subreddits_new", "/v1/reddit/subreddits/new", None),
        ("subreddits_popular", "/v1/reddit/subreddits/popular", None),
        ("posts_trending", "/v1/reddit/posts/trending", None),
        ("subreddit", f"/v1/reddit/subreddits/{SUBREDDIT}", None),
        ("subreddit_posts", f"/v1/reddit/subreddits/{SUBREDDIT}/posts", {"sort": "hot"}),
        ("subreddit_rules", f"/v1/reddit/subreddits/{SUBREDDIT}/rules", None),
        ("subreddit_wiki_pages", f"/v1/reddit/subreddits/{SUBREDDIT}/wiki", None),
        ("subreddit_wiki_page", f"/v1/reddit/subreddits/{SUBREDDIT}/wiki/{WIKI_PAGE}", None),
        ("user", f"/v1/reddit/users/{USERNAME}", None),
        ("user_posts", f"/v1/reddit/users/{USERNAME}/posts", None),
        ("user_comments", f"/v1/reddit/users/{USERNAME}/comments", None),
        ("user_moderated", f"/v1/reddit/users/{USERNAME}/moderated", None),
        ("user_trophies", f"/v1/reddit/users/{USERNAME}/trophies", None),
        ("domain_posts", f"/v1/reddit/domains/{DOMAIN}/posts", None),
        ("search_posts", "/v1/reddit/search/posts", {"q": QUERY}),
        ("search_subreddits", "/v1/reddit/search/subreddits", {"q": QUERY}),
        ("search_users", "/v1/reddit/search/users", {"q": QUERY}),
    ]

    for name, path, query in jobs:
        if name == "subreddit_posts" and exists(name):
            sub_posts = json.load(open(os.path.join(OUT, name + ".json")))
        if exists(name):
            print(f"  skip {name} (exists)")
            continue
        try:
            data = get(path, query)
            save(name, data)
            if name == "subreddit_posts":
                sub_posts = data
        except urllib.error.HTTPError as e:
            print(f"  !! {name}: HTTP {e.code} {e.reason}")
        except Exception as e:  # noqa: BLE001
            print(f"  !! {name}: {e}")
        time.sleep(REQUEST_DELAY)

    # Post-scoped endpoints need a real post id.
    pid = first_post_id(sub_posts) if sub_posts else None
    if not pid:
        print("  !! could not derive a post id; skipping post-scoped endpoints")
        return
    print(f"  using post id: {pid}")
    for name, path in [
        ("post", f"/v1/reddit/posts/{pid}"),
        ("post_comments", f"/v1/reddit/posts/{pid}/comments"),
        ("post_duplicates", f"/v1/reddit/posts/{pid}/duplicates"),
    ]:
        if exists(name):
            print(f"  skip {name} (exists)")
            continue
        try:
            save(name, get(path))
        except urllib.error.HTTPError as e:
            print(f"  !! {name}: HTTP {e.code} {e.reason}")
        except Exception as e:  # noqa: BLE001
            print(f"  !! {name}: {e}")
        time.sleep(REQUEST_DELAY)

    print(f"\nDone. Samples in {OUT}/  — re-run safe.")


if __name__ == "__main__":
    main()
