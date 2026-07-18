#!/usr/bin/env python3
"""Vendor per-platform OpenAPI specs from the live ScrapeBadger Portal spec.

The live "Portal API" spec at ``https://scrapebadger.com/api/openapi.json`` is the
source of truth for every platform. This script slices out one platform's paths
into ``specs/<stem>.json`` in the exact shape the codegen (``xtask``) expects:

  * only ``/v1/<platform>/...`` paths, ``health`` endpoints excluded;
  * each operation keeps just ``operationId``, ``summary``, ``description``,
    ``parameters``, and its ``200`` response (drops ``tags`` and error responses);
  * a curated ``info`` (title/version/description), the production ``servers``
    block, ``security``, and the ``apiKeyAuth`` security scheme;
  * ``components.schemas`` left empty — the Portal spec leaves 200 bodies untyped
    (title only), so the codegen maps them onto ``serde_json::Value``.

Only the platforms named on the command line (default: the 9 that were added in
the LinkedIn/real-estate expansion) are written. The 10 original specs are
hand-curated (vinted in particular deliberately keeps filters the Portal spec
under-documents) and are never touched.

Usage:
    python3 scripts/vendor_specs.py                 # all 9 new platforms
    python3 scripts/vendor_specs.py linkedin depop  # a subset
    SPEC_URL=file:///tmp/portal.json python3 scripts/vendor_specs.py
"""

import json
import os
import sys
import urllib.request
from pathlib import Path

SPEC_URL = os.environ.get("SPEC_URL", "https://scrapebadger.com/api/openapi.json")

# platform stem -> (info title, info description). The stem doubles as the module
# name and the `/v1/<stem>/` path prefix for every platform added in this batch.
PLATFORMS = {
    "linkedin": (
        "ScrapeBadger LinkedIn API",
        "LinkedIn public-data scraping: job search & detail, company/school/"
        "profile pages, posts, articles, Learning courses, and geo-id resolution.",
    ),
    "depop": (
        "ScrapeBadger Depop API",
        "Depop marketplace scraping: product search & detail, shop/user profiles, "
        "and a user's product listings.",
    ),
    "idealista": (
        "ScrapeBadger Idealista API",
        "Idealista (Spain) real-estate scraping: listing search & detail, "
        "engagement stats, agency profiles, and location resolution.",
    ),
    "immobiliare": (
        "ScrapeBadger Immobiliare API",
        "Immobiliare (Italy) real-estate scraping: listing search & detail, agency "
        "profiles & listings, price insights, and location autocomplete.",
    ),
    "leboncoin": (
        "ScrapeBadger Leboncoin API",
        "Leboncoin (France) classifieds scraping: ad search & detail, similar ads, "
        "seller profiles & listings, and category/region reference data.",
    ),
    "loopnet": (
        "ScrapeBadger LoopNet API",
        "LoopNet commercial real-estate scraping: listing search & detail, broker "
        "profiles, and property-type/market reference data.",
    ),
    "realtor": (
        "ScrapeBadger Realtor API",
        "Realtor.com (US) real-estate scraping: property search & detail and "
        "location autocomplete.",
    ),
    "redfin": (
        "ScrapeBadger Redfin API",
        "Redfin (US) real-estate scraping: property search & detail, agent "
        "profiles, and region/address autocomplete.",
    ),
    "zillow": (
        "ScrapeBadger Zillow API",
        "Zillow (US) real-estate scraping: property search & detail, agent "
        "profiles, and region/address autocomplete.",
    ),
}

# Fields kept on each operation (everything else, e.g. `tags`, is dropped).
OP_KEYS = ("operationId", "summary", "description", "parameters")

SERVERS = [{"url": "https://scrapebadger.com", "description": "Production"}]
SECURITY = [{"apiKeyAuth": []}]
SECURITY_SCHEMES = {
    "apiKeyAuth": {"type": "apiKey", "in": "header", "name": "x-api-key"}
}


def slice_platform(portal: dict, stem: str, title: str, desc: str) -> dict:
    prefix = f"/v1/{stem}/"
    paths: dict = {}
    for path, item in portal.get("paths", {}).items():
        if not path.startswith(prefix):
            continue
        if any(seg == "health" for seg in path.split("/")):
            continue
        new_item: dict = {}
        for verb, op in item.items():
            if verb not in ("get", "post", "put", "patch", "delete"):
                continue
            trimmed = {k: op[k] for k in OP_KEYS if k in op}
            responses = op.get("responses", {})
            if "200" in responses:
                trimmed["responses"] = {"200": responses["200"]}
            else:
                trimmed["responses"] = {}
            new_item[verb] = trimmed
        if new_item:
            paths[path] = new_item

    if not paths:
        raise SystemExit(f"no paths found for platform {stem!r} (prefix {prefix})")

    return {
        "openapi": portal.get("openapi", "3.1.0"),
        "info": {"title": title, "version": "1.0.0", "description": desc},
        "servers": SERVERS,
        "security": SECURITY,
        "paths": dict(sorted(paths.items())),
        "components": {"securitySchemes": SECURITY_SCHEMES, "schemas": {}},
    }


def main() -> None:
    wanted = sys.argv[1:] or list(PLATFORMS)
    unknown = [p for p in wanted if p not in PLATFORMS]
    if unknown:
        raise SystemExit(f"unknown platform(s): {', '.join(unknown)}")

    with urllib.request.urlopen(SPEC_URL) as resp:
        portal = json.load(resp)

    specs_dir = Path(__file__).resolve().parent.parent / "specs"
    for stem in wanted:
        title, desc = PLATFORMS[stem]
        spec = slice_platform(portal, stem, title, desc)
        dest = specs_dir / f"{stem}.json"
        dest.write_text(json.dumps(spec, indent=2) + "\n")
        print(f"{stem:>12}: {len(spec['paths']):>2} paths -> {dest.relative_to(specs_dir.parent)}")


if __name__ == "__main__":
    main()
