//! CLI descriptor-table + docs generator.
//!
//! Derives the nested-command tree from the OpenAPI specs and emits:
//!   * `crates/scrapebadger/src/cli/generated.rs` — the `ENDPOINTS` table that
//!     drives the runtime clap tree, and
//!   * `crates/scrapebadger/docs/CLI.md` — the human reference (also rendered on
//!     docs.rs via a `cli_reference` module).
//!
//! Command-path rules:
//!   * static path segments → subcommands, `{params}` → trailing positionals;
//!   * a unique chain ending in a param gets a `get` leaf; a chain shared by
//!     several methods/paths (CRUD) gets `list`/`get`/`create`/`update`/`delete`;
//!   * a runnable node that is also a parent gets a `get` leaf;
//!   * action segments are kebab-cased, with the underscore spelling kept as a
//!     hidden alias.

use std::collections::BTreeMap;
use std::path::Path;

use heck::ToKebabCase;
use serde_json::Value;

use crate::PLATFORMS;

#[derive(Clone)]
struct Param {
    name: String,
    flag: String,
    ty: &'static str, // "Str" | "Bool" | "Int" | "Num" | "StrArray"
    required: bool,
    help: String,
}

#[derive(Clone)]
struct Ep {
    platform: String,
    verb: String, // GET/POST/...
    path: String,
    summary: String,
    path_params: Vec<String>,
    query: Vec<Param>,
    body: Vec<Param>,
    has_body: bool,
    // Derived:
    cmd: Vec<String>, // command path incl. platform (kebab)
    leaf_alias: Option<String>,
}

/// Entry point: generate the CLI table + docs. Returns the endpoint count.
pub fn generate(root: &Path) {
    let specs_dir = root.join("specs");
    let mut eps: Vec<Ep> = Vec::new();
    for (stem, module, _handle) in PLATFORMS {
        let spec: Value = serde_json::from_slice(
            &std::fs::read(specs_dir.join(format!("{stem}.json"))).expect("read spec"),
        )
        .expect("parse spec");
        collect_endpoints(module, &spec, &mut eps);
    }

    derive_commands(&mut eps);
    assert_invariants(&eps);

    let table = render_table(&eps);
    let dest = root.join("crates/scrapebadger/src/cli/generated.rs");
    std::fs::write(&dest, table).expect("write generated.rs");
    println!(
        "     cli: {:>3} commands -> {}",
        eps.len(),
        rel(&dest, root)
    );

    let docs = render_docs(&eps);
    let docs_dest = root.join("crates/scrapebadger/docs/CLI.md");
    std::fs::create_dir_all(docs_dest.parent().unwrap()).ok();
    std::fs::write(&docs_dest, docs).expect("write CLI.md");
    println!(
        "    docs: {:>3} commands -> {}",
        eps.len(),
        rel(&docs_dest, root)
    );
}

/// Parse a spec's operations into raw endpoints (no command derivation yet).
fn collect_endpoints(module: &str, spec: &Value, out: &mut Vec<Ep>) {
    let paths = match spec.get("paths").and_then(Value::as_object) {
        Some(p) => p,
        None => return,
    };
    for (path, item) in paths {
        // Skip health endpoints — not part of the data surface (matches SDK).
        if path.split('/').any(|s| s == "health") {
            continue;
        }
        let item = match item.as_object() {
            Some(o) => o,
            None => continue,
        };
        for verb in ["get", "post", "put", "patch", "delete"] {
            let op = match item.get(verb).and_then(Value::as_object) {
                Some(o) => o,
                None => continue,
            };
            let path_params = path_params_of(path);
            let mut query = Vec::new();
            if let Some(arr) = op.get("parameters").and_then(Value::as_array) {
                for p in arr {
                    if p.get("in").and_then(Value::as_str) != Some("query") {
                        continue;
                    }
                    let name = p
                        .get("name")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_string();
                    let schema = p.get("schema").cloned().unwrap_or(Value::Null);
                    let required = p.get("required").and_then(Value::as_bool).unwrap_or(false);
                    let help = p
                        .get("description")
                        .and_then(Value::as_str)
                        .or_else(|| schema.get("description").and_then(Value::as_str))
                        .unwrap_or("")
                        .to_string();
                    query.push(Param {
                        flag: name.to_kebab_case(),
                        ty: ty_of(&schema),
                        required,
                        help,
                        name,
                    });
                }
            }

            // Body (POST/PATCH) → scalar typed flags; non-scalars stay JSON-only.
            let mut body = Vec::new();
            let has_body = op.get("requestBody").is_some();
            if let Some(schema) = op
                .get("requestBody")
                .and_then(|v| v.pointer("/content/application~1json/schema"))
            {
                let resolved = resolve_ref(spec, schema);
                let required: Vec<String> = resolved
                    .get("required")
                    .and_then(Value::as_array)
                    .map(|a| {
                        a.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                if let Some(props) = resolved.get("properties").and_then(Value::as_object) {
                    for (name, psch) in props {
                        let ty = ty_of(psch);
                        // Only scalars + scalar arrays become typed flags.
                        if ty == "Str"
                            || ty == "Bool"
                            || ty == "Int"
                            || ty == "Num"
                            || ty == "StrArray"
                        {
                            // skip nested objects/arrays-of-objects
                            if is_complex(psch) {
                                continue;
                            }
                            let help = psch
                                .get("description")
                                .and_then(Value::as_str)
                                .unwrap_or("")
                                .to_string();
                            body.push(Param {
                                flag: name.to_kebab_case(),
                                ty,
                                required: required.contains(name),
                                help,
                                name: name.clone(),
                            });
                        }
                    }
                }
            }

            out.push(Ep {
                platform: module.to_string(),
                verb: verb.to_uppercase(),
                path: path.clone(),
                summary: op
                    .get("summary")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string(),
                path_params,
                query,
                body,
                has_body,
                cmd: Vec::new(),
                leaf_alias: None,
            });
        }
    }
}

/// Derive each endpoint's nested command path + hidden alias.
fn derive_commands(eps: &mut [Ep]) {
    // 1. Group by (platform, static-chain) to detect CRUD collisions.
    let mut share: BTreeMap<(String, Vec<String>), usize> = BTreeMap::new();
    for e in eps.iter() {
        *share
            .entry((e.platform.clone(), static_chain(e)))
            .or_insert(0) += 1;
    }

    // 2. Base command path per endpoint (raw segments, pre-kebab).
    for e in eps.iter_mut() {
        let chain = static_chain(e);
        let crud = share[&(e.platform.clone(), chain.clone())] > 1;
        let ends_param = e
            .path
            .trim_end_matches('/')
            .rsplit('/')
            .next()
            .map(|s| s.starts_with('{'))
            .unwrap_or(false);
        let leaf: Option<&str> = if crud {
            Some(match e.verb.as_str() {
                "GET" => {
                    if ends_param {
                        "get"
                    } else {
                        "list"
                    }
                }
                "POST" => "create",
                "PATCH" | "PUT" => "update",
                "DELETE" => "delete",
                _ => "get",
            })
        } else if ends_param {
            Some("get")
        } else {
            None
        };
        let mut cmd = vec![e.platform.clone()];
        cmd.extend(chain.iter().cloned());
        if let Some(l) = leaf {
            cmd.push(l.to_string());
        }
        e.cmd = cmd;
    }

    // 3. Runnable-parent fix: if a command path is a strict prefix of another,
    //    append `get` so the parent becomes a pure group.
    let all: Vec<Vec<String>> = eps.iter().map(|e| e.cmd.clone()).collect();
    for e in eps.iter_mut() {
        let is_prefix = all
            .iter()
            .any(|o| o.len() > e.cmd.len() && o[..e.cmd.len()] == e.cmd[..]);
        if is_prefix {
            e.cmd.push("get".to_string());
        }
    }

    // 4. Kebab-case each segment; keep the underscore leaf as a hidden alias.
    for e in eps.iter_mut() {
        let orig_leaf = e.cmd.last().cloned().unwrap_or_default();
        let kebab: Vec<String> = e.cmd.iter().map(|s| s.to_kebab_case()).collect();
        let kebab_leaf = kebab.last().cloned().unwrap_or_default();
        if kebab_leaf != orig_leaf {
            e.leaf_alias = Some(orig_leaf);
        }
        e.cmd = kebab;
    }
}

/// Static (non-param) path segments after the platform name.
fn static_chain(e: &Ep) -> Vec<String> {
    let mut segs: Vec<&str> = e
        .path
        .split('/')
        .filter(|s| !s.is_empty() && *s != "v1" && *s != "api" && *s != "_")
        .collect();
    if segs.first() == Some(&e.platform.as_str()) {
        segs.remove(0);
    }
    segs.into_iter()
        .filter(|s| !s.starts_with('{'))
        .map(String::from)
        .collect()
}

fn path_params_of(path: &str) -> Vec<String> {
    path.split('/')
        .filter(|s| s.starts_with('{') && s.ends_with('}'))
        .map(|s| s[1..s.len() - 1].to_string())
        .collect()
}

fn ty_of(schema: &Value) -> &'static str {
    let t = if let Some(any) = schema.get("anyOf").and_then(Value::as_array) {
        any.iter()
            .filter_map(|s| s.get("type").and_then(Value::as_str))
            .find(|t| *t != "null")
            .unwrap_or("string")
    } else {
        schema
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or("string")
    };
    match t {
        "boolean" => "Bool",
        "integer" => "Int",
        "number" => "Num",
        "array" => "StrArray",
        _ => "Str",
    }
}

/// True for object schemas or arrays of non-scalars (not flag-able).
fn is_complex(schema: &Value) -> bool {
    let t = schema.get("type").and_then(Value::as_str);
    if t == Some("object") || schema.get("properties").is_some() {
        return true;
    }
    if t == Some("array") {
        if let Some(items) = schema.get("items") {
            let it = items.get("type").and_then(Value::as_str);
            return it == Some("object") || items.get("properties").is_some();
        }
    }
    false
}

fn resolve_ref(spec: &Value, schema: &Value) -> Value {
    if let Some(r) = schema.get("$ref").and_then(Value::as_str) {
        let name = r.rsplit('/').next().unwrap_or("");
        if let Some(s) = spec.pointer(&format!("/components/schemas/{name}")) {
            return s.clone();
        }
    }
    schema.clone()
}

/// Sanity-check the invariants the runtime tree relies on.
fn assert_invariants(eps: &[Ep]) {
    // No duplicate command paths.
    let mut seen = std::collections::BTreeSet::new();
    for e in eps {
        let key = e.cmd.join(" ");
        assert!(seen.insert(key.clone()), "duplicate command path: {key}");
    }
    // No runnable-parents (a command path that is a strict prefix of another).
    let all: Vec<&Vec<String>> = eps.iter().map(|e| &e.cmd).collect();
    for c in &all {
        assert!(
            !all.iter()
                .any(|o| o.len() > c.len() && o[..c.len()] == c[..]),
            "runnable-parent: {}",
            c.join(" ")
        );
    }
}

// ===== Emitters =====

fn esc(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn render_params(ps: &[Param]) -> String {
    if ps.is_empty() {
        return "&[]".to_string();
    }
    let mut s = String::from("&[");
    for p in ps {
        s.push_str(&format!(
            "Param{{name:\"{}\",flag:\"{}\",ty:Ty::{},required:{},help:\"{}\"}},",
            esc(&p.name),
            esc(&p.flag),
            p.ty,
            p.required,
            esc(&p.help),
        ));
    }
    s.push(']');
    s
}

fn render_table(eps: &[Ep]) -> String {
    let mut out = String::new();
    out.push_str(
        "// @generated by `cargo run -p xtask -- gen` from specs/*.json — do not edit by hand.\n",
    );
    out.push_str("#![allow(clippy::all)]\n\n");
    out.push_str("use super::{Endpoint, Param, Ty};\n\n");
    out.push_str(&format!(
        "/// Every ScrapeBadger endpoint as a nested CLI command ({} total).\n",
        eps.len()
    ));
    out.push_str("pub static ENDPOINTS: &[Endpoint] = &[\n");
    // Sort for stable output.
    let mut sorted: Vec<&Ep> = eps.iter().collect();
    sorted.sort_by(|a, b| a.cmd.cmp(&b.cmd));
    for e in sorted {
        let cmd = e
            .cmd
            .iter()
            .map(|s| format!("\"{}\"", esc(s)))
            .collect::<Vec<_>>()
            .join(",");
        let pp = e
            .path_params
            .iter()
            .map(|s| format!("\"{}\"", esc(s)))
            .collect::<Vec<_>>()
            .join(",");
        let alias = match &e.leaf_alias {
            Some(a) => format!("Some(\"{}\")", esc(a)),
            None => "None".to_string(),
        };
        out.push_str(&format!(
            "    Endpoint{{platform:\"{}\",path_cmd:&[{}],leaf_alias:{},method:\"{}\",path:\"{}\",path_params:&[{}],query:{},body:{},has_body:{},summary:\"{}\"}},\n",
            esc(&e.platform),
            cmd,
            alias,
            e.verb,
            esc(&e.path),
            pp,
            render_params(&e.query),
            render_params(&e.body),
            e.has_body,
            esc(&e.summary),
        ));
    }
    out.push_str("];\n");
    out
}

fn render_docs(eps: &[Ep]) -> String {
    let titles: &[(&str, &str)] = &[
        ("account", "Account"),
        ("amazon", "Amazon"),
        ("depop", "Depop"),
        ("ebay", "eBay"),
        ("google", "Google"),
        ("idealista", "Idealista"),
        ("immobiliare", "Immobiliare"),
        ("leboncoin", "Leboncoin"),
        ("linkedin", "LinkedIn"),
        ("loopnet", "LoopNet"),
        ("realtor", "Realtor"),
        ("redfin", "Redfin"),
        ("reddit", "Reddit"),
        ("tiktok", "TikTok"),
        ("twitter", "Twitter / X"),
        ("vinted", "Vinted"),
        ("web", "Web Scraping"),
        ("youtube", "YouTube"),
        ("zillow", "Zillow"),
    ];
    let mut s = String::new();
    s.push_str("# scrapebadger CLI — command reference\n\n");
    s.push_str(&format!(
        "All **{}** endpoints as nested subcommands, generated from `specs/*.json` — the same source as the SDK. The general shape is:\n\n",
        eps.len()
    ));
    s.push_str(
        "```text\nscrapebadger PLATFORM GROUP [SUBGROUP] ACTION [IDS...] [--flags]\n```\n\n",
    );
    s.push_str("An `<arg>` below is a required positional; run `scrapebadger <command> --help` for full per-flag help, or `scrapebadger <platform> --help` to list every command for that platform at once.\n");
    for (key, title) in titles {
        let mut group: Vec<&Ep> = eps.iter().filter(|e| e.platform == *key).collect();
        if group.is_empty() {
            continue;
        }
        group.sort_by(|a, b| a.cmd.cmp(&b.cmd));
        s.push_str(&format!("\n## {title}\n\n"));
        s.push_str(&format!(
            "`scrapebadger {key}` — {} endpoints\n\n",
            group.len()
        ));
        s.push_str("```text\n");
        let sigs: Vec<(String, &str)> = group
            .iter()
            .map(|e| {
                let mut sig = e.cmd.join(" ");
                for p in &e.path_params {
                    sig.push_str(&format!(" <{p}>"));
                }
                (sig, e.summary.as_str())
            })
            .collect();
        let width = sigs.iter().map(|(s, _)| s.len()).max().unwrap_or(0);
        for (sig, summary) in sigs {
            s.push_str(&format!("{sig:<width$}  {summary}\n"));
        }
        s.push_str("```\n");
    }
    s
}

fn rel(p: &Path, root: &Path) -> String {
    p.strip_prefix(root)
        .unwrap_or(p)
        .to_string_lossy()
        .to_string()
}
