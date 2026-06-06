//! `scrapebadger` command-line interface.
//!
//! The entire command tree is generated from the OpenAPI specs (see
//! [`scrapebadger::cli`]): every endpoint is a nested subcommand
//! (`scrapebadger <platform> <group> <action> [<ids>] [--flags]`). A generic
//! dispatcher maps the selected leaf back to its descriptor and issues the
//! request. `raw` reaches any endpoint directly; `config` manages the stored
//! API key; `commands` and `completions` aid discovery.

use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use clap::ArgMatches;
use scrapebadger::cli::{self, Endpoint, Param, Ty};
use scrapebadger::core::Method;
use scrapebadger::ScrapeBadger;
use serde_json::{Map, Value};

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("error: {err:#}");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let matches = cli::build_command(env!("CARGO_PKG_VERSION")).get_matches();

    // `--help-all`: dump the entire tree (every platform) and exit.
    if matches.get_flag("help_all") {
        return dump_all();
    }

    // Subcommands that don't need a client.
    match matches.subcommand() {
        Some(("config", m)) => return handle_config(m),
        Some(("commands", m)) => return handle_commands(m),
        Some(("completions", m)) => return handle_completions(m),
        Some(("man", m)) => return handle_man(m),
        None => {
            cli::build_command(env!("CARGO_PKG_VERSION")).print_help()?;
            return Ok(());
        }
        _ => {}
    }

    // Resolve the API key: --api-key / env (both via the global arg) > config file.
    let key = matches
        .get_one::<String>("api_key")
        .cloned()
        .or_else(load_config_key);
    let mut builder = ScrapeBadger::builder();
    if let Some(key) = key {
        builder = builder.api_key(key);
    }
    let client = builder.build().context(
        "no API key found: pass --api-key, set SCRAPEBADGER_API_KEY, or run `scrapebadger config set-key <KEY>`",
    )?;

    if let Some(("raw", m)) = matches.subcommand() {
        return run_raw(&client, m).await;
    }

    // Otherwise: walk to the selected endpoint leaf and dispatch.
    let mut path = Vec::new();
    let leaf = cli::find_leaf(&matches, &mut path);
    let ep = cli::lookup(&path).ok_or_else(|| anyhow!("unknown command: {}", path.join(" ")))?;
    run_endpoint(&client, ep, leaf).await
}

/// Bind args to an endpoint descriptor and execute (or explain).
async fn run_endpoint(client: &ScrapeBadger, ep: &Endpoint, m: &ArgMatches) -> Result<()> {
    // Path params → fill the template.
    let mut path = ep.path.to_string();
    for p in ep.path_params {
        let val = m
            .get_one::<String>(p)
            .ok_or_else(|| anyhow!("missing path argument: {p}"))?;
        path = path.replace(&format!("{{{p}}}"), val);
    }

    // Query flags.
    let query = collect_query(ep.query, m);

    // Body: --body / --body-file as the base, typed flags override.
    let body = if ep.has_body {
        let mut map = base_body(m)?;
        overlay_body(ep.body, m, &mut map);
        if map.is_empty() {
            None
        } else {
            Some(Value::Object(map))
        }
    } else {
        None
    };

    let method: Method = ep
        .method
        .parse()
        .with_context(|| format!("invalid method: {}", ep.method))?;

    // --explain / --curl short-circuit before any network call.
    if m.get_flag("explain") || m.get_flag("curl") {
        return explain(client, m, &method, &path, &query, body.as_ref());
    }

    let follow_all = m.get_flag("all");
    let value = if follow_all {
        send_paginated(client, ep, method, &path, query, body).await?
    } else {
        client
            .client()
            .send::<Value>(method, &path, &query, body)
            .await?
    };

    emit(&value, m)
}

/// The `raw` escape hatch — unchanged behaviour from the original CLI.
async fn run_raw(client: &ScrapeBadger, m: &ArgMatches) -> Result<()> {
    let method: Method = m
        .get_one::<String>("method")
        .unwrap()
        .parse()
        .context("invalid HTTP method")?;
    let path = m.get_one::<String>("path").unwrap();
    let query: Vec<(String, String)> = m
        .get_many::<String>("query")
        .map(|vs| {
            vs.map(|kv| {
                let (k, v) = kv
                    .split_once('=')
                    .with_context(|| format!("query must be key=value: {kv}"))?;
                Ok::<_, anyhow::Error>((k.to_string(), v.to_string()))
            })
            .collect::<Result<_>>()
        })
        .transpose()?
        .unwrap_or_default();
    let body = match m.get_one::<String>("body") {
        Some(b) => Some(serde_json::from_str(b).context("--body is not valid JSON")?),
        None => None,
    };
    if m.get_flag("explain") || m.get_flag("curl") {
        return explain(client, m, &method, path, &query, body.as_ref());
    }
    let value: Value = client.client().send(method, path, &query, body).await?;
    emit(&value, m)
}

// ===== Param binding =====

fn collect_query(params: &[Param], m: &ArgMatches) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for p in params {
        match p.ty {
            Ty::Bool => {
                if m.get_flag(p.name) {
                    out.push((p.name.to_string(), "true".to_string()));
                }
            }
            Ty::StrArray => {
                if let Some(vs) = m.get_many::<String>(p.name) {
                    for v in vs {
                        out.push((p.name.to_string(), v.clone()));
                    }
                }
            }
            _ => {
                if let Some(v) = m.get_one::<String>(p.name) {
                    out.push((p.name.to_string(), v.clone()));
                }
            }
        }
    }
    out
}

fn base_body(m: &ArgMatches) -> Result<Map<String, Value>> {
    let raw = if let Some(path) = m.get_one::<String>("body_file") {
        Some(fs::read_to_string(path).with_context(|| format!("reading {path}"))?)
    } else {
        m.get_one::<String>("body").cloned()
    };
    match raw {
        Some(s) => match serde_json::from_str::<Value>(&s).context("--body is not valid JSON")? {
            Value::Object(map) => Ok(map),
            _ => Err(anyhow!("--body must be a JSON object")),
        },
        None => Ok(Map::new()),
    }
}

fn overlay_body(params: &[Param], m: &ArgMatches, map: &mut Map<String, Value>) {
    for p in params {
        match p.ty {
            Ty::Bool => {
                if m.get_flag(p.name) {
                    map.insert(p.name.to_string(), Value::Bool(true));
                }
            }
            Ty::StrArray => {
                if let Some(vs) = m.get_many::<String>(p.name) {
                    let arr: Vec<Value> = vs.map(|v| Value::String(v.clone())).collect();
                    if !arr.is_empty() {
                        map.insert(p.name.to_string(), Value::Array(arr));
                    }
                }
            }
            Ty::Int => {
                if let Some(v) = m.get_one::<String>(p.name) {
                    if let Ok(n) = v.parse::<i64>() {
                        map.insert(p.name.to_string(), Value::from(n));
                    }
                }
            }
            Ty::Num => {
                if let Some(v) = m.get_one::<String>(p.name) {
                    if let Ok(n) = v.parse::<f64>() {
                        map.insert(p.name.to_string(), Value::from(n));
                    }
                }
            }
            Ty::Str => {
                if let Some(v) = m.get_one::<String>(p.name) {
                    map.insert(p.name.to_string(), Value::String(v.clone()));
                }
            }
        }
    }
}

// ===== Pagination (--all, best-effort) =====

/// Follow `after`/`cursor` pagination, merging the primary array across pages.
async fn send_paginated(
    client: &ScrapeBadger,
    ep: &Endpoint,
    method: Method,
    path: &str,
    base_query: Vec<(String, String)>,
    body: Option<Value>,
) -> Result<Value> {
    let cursor_flag = ["after", "cursor"]
        .into_iter()
        .find(|c| ep.query.iter().any(|p| p.name == *c));
    let cursor_flag = match cursor_flag {
        Some(c) => c,
        // No cursor param — nothing to follow; single request.
        None => {
            return Ok(client
                .client()
                .send::<Value>(method, path, &base_query, body)
                .await?)
        }
    };

    let mut merged: Option<Value> = None;
    let mut array_key: Option<String> = None;
    let mut cursor: Option<String> = None;
    for _ in 0..50 {
        let mut query = base_query.clone();
        query.retain(|(k, _)| k != cursor_flag);
        if let Some(c) = &cursor {
            query.push((cursor_flag.to_string(), c.clone()));
        }
        let page: Value = client
            .client()
            .send(method.clone(), path, &query, body.clone())
            .await?;

        if merged.is_none() {
            array_key = primary_array_key(&page);
            merged = Some(page.clone());
        } else if let (Some(key), Some(Value::Object(acc))) = (&array_key, merged.as_mut()) {
            if let (Some(Value::Array(into)), Some(Value::Array(more))) =
                (acc.get_mut(key), page.get(key))
            {
                into.extend(more.iter().cloned());
            }
        }

        cursor = next_cursor(&page);
        if cursor.is_none() {
            break;
        }
    }
    Ok(merged.unwrap_or(Value::Null))
}

/// Name of the first top-level array field (the page's items).
fn primary_array_key(v: &Value) -> Option<String> {
    v.as_object()?
        .iter()
        .find(|(_, val)| val.is_array())
        .map(|(k, _)| k.clone())
}

/// Extract the next-page cursor from common response shapes.
fn next_cursor(v: &Value) -> Option<String> {
    let candidates = [
        v.pointer("/pagination/after"),
        v.get("next_cursor"),
        v.get("cursor"),
    ];
    for c in candidates.into_iter().flatten() {
        match c {
            Value::String(s) if !s.is_empty() => return Some(s.clone()),
            _ => {}
        }
    }
    None
}

// ===== Output =====

/// Apply `--select`, then format per `-o` and print.
fn emit(value: &Value, m: &ArgMatches) -> Result<()> {
    let selected = match m.get_one::<String>("select") {
        Some(path) => {
            let vals = project(vec![value.clone()], path);
            match vals.len() {
                1 => vals.into_iter().next().unwrap(),
                _ => Value::Array(vals),
            }
        }
        None => value.clone(),
    };

    let format = m
        .get_one::<String>("output")
        .map(String::as_str)
        .unwrap_or("json");
    match format {
        "jsonl" => match &selected {
            Value::Array(items) => {
                for it in items {
                    println!("{}", serde_json::to_string(it)?);
                }
            }
            other => println!("{}", serde_json::to_string(other)?),
        },
        "raw" => match &selected {
            Value::String(s) => println!("{s}"),
            other => println!("{}", serde_json::to_string_pretty(other)?),
        },
        _ => println!("{}", serde_json::to_string_pretty(&selected)?),
    }
    Ok(())
}

/// Minimal jq-ish projection: `.a.b`, `.a[].b`, `.items[]`.
fn project(values: Vec<Value>, path: &str) -> Vec<Value> {
    let segs: Vec<&str> = path.split('.').filter(|s| !s.is_empty()).collect();
    let mut current = values;
    for seg in segs {
        let (key, iter) = match seg.strip_suffix("[]") {
            Some(k) => (k, true),
            None => (seg, false),
        };
        let mut next = Vec::new();
        for v in current.drain(..) {
            let got = if key.is_empty() {
                v
            } else {
                v.get(key).cloned().unwrap_or(Value::Null)
            };
            if iter {
                if let Value::Array(a) = got {
                    next.extend(a);
                }
            } else {
                next.push(got);
            }
        }
        current = next;
    }
    current
}

// ===== --explain / --curl =====

fn explain(
    client: &ScrapeBadger,
    m: &ArgMatches,
    method: &Method,
    path: &str,
    query: &[(String, String)],
    body: Option<&Value>,
) -> Result<()> {
    let base = client.client().config().base_url.trim_end_matches('/');
    let qs = if query.is_empty() {
        String::new()
    } else {
        let pairs: Vec<String> = query
            .iter()
            .map(|(k, v)| format!("{}={}", urlencode(k), urlencode(v)))
            .collect();
        format!("?{}", pairs.join("&"))
    };
    let url = format!("{base}{path}{qs}");

    if m.get_flag("curl") {
        let key = if m.get_flag("reveal") {
            client.client().config().api_key.clone()
        } else {
            "$SCRAPEBADGER_API_KEY".to_string()
        };
        let mut s = format!("curl -X {method} '{url}' \\\n  -H 'x-api-key: {key}'");
        if let Some(b) = body {
            s.push_str(&format!(
                " \\\n  -H 'content-type: application/json' \\\n  -d '{}'",
                serde_json::to_string(b)?
            ));
        }
        println!("{s}");
    } else {
        println!("{method} {url}");
        if let Some(b) = body {
            println!("body: {}", serde_json::to_string_pretty(b)?);
        }
    }
    Ok(())
}

fn urlencode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

// ===== `commands` (flat tree dump) =====

fn handle_commands(m: &ArgMatches) -> Result<()> {
    let filter = m.get_one::<String>("platform").map(String::as_str);
    let mut rows: Vec<(String, &str)> = cli::ENDPOINTS
        .iter()
        .filter(|e| filter.map_or(true, |f| e.platform == f))
        .map(|e| (e.signature(), e.summary))
        .collect();
    rows.sort();
    let width = rows.iter().map(|(s, _)| s.len()).max().unwrap_or(0);
    for (sig, summary) in rows {
        println!("{sig:<width$}  {summary}");
    }
    Ok(())
}

/// `--help-all`: print every command across all platforms.
fn dump_all() -> Result<()> {
    let mut rows: Vec<(String, &str)> = cli::ENDPOINTS
        .iter()
        .map(|e| (e.signature(), e.summary))
        .collect();
    rows.sort();
    let width = rows.iter().map(|(s, _)| s.len()).max().unwrap_or(0);
    println!("All {} commands:\n", rows.len());
    for (sig, summary) in rows {
        println!("  {sig:<width$}  {summary}");
    }
    println!("\nRun any command with --help for its arguments and flags.");
    Ok(())
}

// ===== `man` =====

fn handle_man(m: &ArgMatches) -> Result<()> {
    let out_dir = PathBuf::from(m.get_one::<String>("out_dir").unwrap());
    fs::create_dir_all(&out_dir).with_context(|| format!("creating {}", out_dir.display()))?;
    let cmd = cli::build_command(env!("CARGO_PKG_VERSION"));
    let mut count = 0usize;
    render_man(&cmd, "scrapebadger", &out_dir, &mut count)?;
    println!("wrote {count} man pages to {}", out_dir.display());
    Ok(())
}

/// Recursively render a man page per command (`prefix` is the hyphenated name).
fn render_man(cmd: &clap::Command, prefix: &str, dir: &PathBuf, count: &mut usize) -> Result<()> {
    let page = clap_mangen::Man::new(cmd.clone());
    let mut buf = Vec::new();
    page.render(&mut buf)?;
    fs::write(dir.join(format!("{prefix}.1")), buf)?;
    *count += 1;
    for sub in cmd.get_subcommands() {
        if sub.get_name() == "help" {
            continue;
        }
        render_man(sub, &format!("{prefix}-{}", sub.get_name()), dir, count)?;
    }
    Ok(())
}

// ===== `completions` =====

fn handle_completions(m: &ArgMatches) -> Result<()> {
    use clap_complete::{generate, Shell};
    let shell: Shell = m
        .get_one::<String>("shell")
        .unwrap()
        .parse()
        .map_err(|e| anyhow!("{e}"))?;
    let mut cmd = cli::build_command(env!("CARGO_PKG_VERSION"));
    let name = cmd.get_name().to_string();
    generate(shell, &mut cmd, name, &mut std::io::stdout());
    Ok(())
}

// ===== Config file management =====

fn config_path() -> Result<PathBuf> {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))
        .ok_or_else(|| {
            anyhow!("cannot determine config directory (set HOME or XDG_CONFIG_HOME)")
        })?;
    Ok(base.join("scrapebadger").join("config.json"))
}

fn load_config_key() -> Option<String> {
    let text = fs::read_to_string(config_path().ok()?).ok()?;
    let value: Value = serde_json::from_str(&text).ok()?;
    value
        .get("api_key")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn save_config_key(key: &str) -> Result<PathBuf> {
    let path = config_path()?;
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))?;
    }
    let mut value: Value = fs::read_to_string(&path)
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok())
        .unwrap_or_else(|| serde_json::json!({}));
    value["api_key"] = Value::String(key.to_string());
    fs::write(&path, serde_json::to_string_pretty(&value)?)
        .with_context(|| format!("writing {}", path.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600))?;
    }
    Ok(path)
}

fn mask(key: &str) -> String {
    match key.len() {
        0 => "(empty)".to_string(),
        1..=12 => "*".repeat(key.len()),
        n => format!("{}…{} ({n} chars)", &key[..8], &key[n - 4..]),
    }
}

fn handle_config(m: &ArgMatches) -> Result<()> {
    match m.subcommand() {
        Some(("set-key", sm)) => {
            let key = match sm.get_one::<String>("key") {
                Some(k) => k.trim().to_string(),
                None => {
                    use std::io::Read;
                    let mut buf = String::new();
                    std::io::stdin().read_to_string(&mut buf)?;
                    buf.trim().to_string()
                }
            };
            if key.is_empty() {
                return Err(anyhow!("no key provided"));
            }
            let path = save_config_key(&key)?;
            println!("Stored API key ({}) in {}", mask(&key), path.display());
        }
        Some(("path", _)) => println!("{}", config_path()?.display()),
        Some(("show", _)) => match load_config_key() {
            Some(k) => println!("api_key = {}", mask(&k)),
            None => println!("no API key stored (run `scrapebadger config set-key <KEY>`)"),
        },
        _ => unreachable!("config requires a subcommand"),
    }
    Ok(())
}
