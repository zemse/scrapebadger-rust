#!/usr/bin/env python3
"""Generate sanitized response fixtures for the offline conformance test.

Captures one live response per distinct response *type* and writes a sanitized
copy to `crates/scrapebadger/tests/fixtures/`. Sanitization is **type-preserving**
but content-free: every string becomes "x", every number 0, every bool false,
nulls/objects/arrays keep their structure (arrays trimmed to 2 elements). This
keeps the exact per-field JSON shape the typed models must handle (including
quirks like numbers-encoded-as-strings and object-valued fields) while removing
all real user data, so the fixtures are safe to commit to a public repo.

Reddit fixtures are derived from existing reddit_samples/ (run
collect_reddit_samples.py first); other platforms are fetched live.

Usage:
    SCRAPEBADGER_API_KEY=sb_live_xxx python3 scripts/make_fixtures.py
"""

import json
import os
import time
import urllib.error
import urllib.parse
import urllib.request

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BASE = os.environ.get("SCRAPEBADGER_BASE_URL", "https://scrapebadger.com")
KEY = os.environ.get("SCRAPEBADGER_API_KEY")
OUT = os.path.join(ROOT, "crates/scrapebadger/tests/fixtures")
SAMPLES = os.path.join(ROOT, "reddit_samples")


def sanitize(v):
    """Type-preserving, content-free scrub. Arrays trimmed to 2 elements."""
    if isinstance(v, dict):
        return {k: sanitize(val) for k, val in v.items()}
    if isinstance(v, list):
        return [sanitize(x) for x in v[:2]]
    if isinstance(v, bool):
        return False
    if isinstance(v, (int, float)):
        return 0
    if isinstance(v, str):
        return "x"
    return None  # null stays null


def write(name, data):
    os.makedirs(OUT, exist_ok=True)
    with open(os.path.join(OUT, name + ".json"), "w") as f:
        json.dump(sanitize(data), f, indent=2)
    print(f"  wrote {name}.json")


def get(path, query=None):
    url = BASE.rstrip("/") + path + ("?" + urllib.parse.urlencode(query) if query else "")
    req = urllib.request.Request(url, headers={"x-api-key": KEY, "accept": "application/json"})
    for attempt in range(6):
        try:
            with urllib.request.urlopen(req, timeout=60) as r:
                return json.loads(r.read().decode("utf-8"))
        except urllib.error.HTTPError as e:
            if e.code == 429 and attempt < 5:
                time.sleep(int(e.headers.get("Retry-After") or min(2**attempt, 30)))
                continue
            raise


# Reddit: reuse existing samples (filename -> fixture name = same).
REDDIT = [
    "subreddit_posts", "subreddit", "subreddits_new", "subreddit_rules",
    "user", "user_comments", "user_moderated", "user_trophies",
    "subreddit_wiki_pages", "subreddit_wiki_page", "post", "post_comments",
    "post_duplicates", "search_users",
]

# Other platforms: (fixture_name, path, query) fetched live.
LIVE = [
    ("account_me", "/v1/account/me", None),
    ("amazon_search", "/v1/amazon/search", {"query": "laptop"}),
    ("amazon_markets", "/v1/amazon/markets", None),
    ("vinted_search_items", "/v1/vinted/search", {"query": "nike"}),
    ("vinted_markets", "/v1/vinted/markets", None),
    ("twitter_advanced_search", "/v1/twitter/tweets/advanced_search", {"query": "rust lang"}),
    ("twitter_search_users", "/v1/twitter/users/search_users", {"query": "rust"}),
    ("twitter_trends", "/v1/twitter/trends/", None),
    ("web_scrape", "/v1/web/scrape", "POST"),
]


def main():
    # Reddit from existing samples.
    for name in REDDIT:
        src = os.path.join(SAMPLES, name + ".json")
        if os.path.exists(src):
            write("reddit_" + name, json.load(open(src)))
        else:
            print(f"  !! missing {src} (run collect_reddit_samples.py)")

    if not KEY:
        print("SCRAPEBADGER_API_KEY not set — skipping live-captured fixtures")
        return

    for name, path, q in LIVE:
        try:
            if q == "POST":
                req = urllib.request.Request(
                    BASE.rstrip("/") + path,
                    data=json.dumps({"url": "https://example.com"}).encode(),
                    headers={"x-api-key": KEY, "content-type": "application/json"},
                    method="POST",
                )
                with urllib.request.urlopen(req, timeout=120) as r:
                    write(name, json.loads(r.read().decode("utf-8")))
            else:
                write(name, get(path, q))
        except Exception as e:  # noqa: BLE001
            print(f"  !! {name}: {e}")
        time.sleep(1.0)

    print(f"\nFixtures in {OUT}/")


if __name__ == "__main__":
    main()
