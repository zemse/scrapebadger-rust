//! Spec-generated CLI command tree.
//!
//! The whole `scrapebadger` command surface is derived from the OpenAPI specs:
//! `ENDPOINTS` (in the generated submodule) is a flat table of every endpoint
//! with its nested command path, positional path params, query/body flags, and
//! help text. `build_command` turns that table into a `clap::Command` tree at
//! runtime, and the binary maps a parsed leaf back to its `Endpoint` (via
//! `find_leaf` + `lookup`) and issues the request.
//!
//! This module is `#[doc(hidden)]` — it is an implementation detail of the
//! binary, not part of the SDK's stable API.

mod generated;

pub use generated::ENDPOINTS;

use clap::{Arg, ArgAction, ArgMatches, Command};

/// Value shape of a CLI flag, used to parse and to build the JSON/query value.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Ty {
    Str,
    Bool,
    Int,
    Num,
    /// Repeatable string flag (`--x a --x b`) or comma-separated list.
    StrArray,
}

/// A query or body parameter exposed as a `--flag`.
pub struct Param {
    /// Original API name (e.g. `render_js`), used as the wire key.
    pub name: &'static str,
    /// Kebab-cased long flag (e.g. `render-js`).
    pub flag: &'static str,
    pub ty: Ty,
    pub required: bool,
    pub help: &'static str,
}

/// One endpoint = one leaf command.
pub struct Endpoint {
    pub platform: &'static str,
    /// Full nested command path including the platform (e.g.
    /// `["reddit", "subreddits", "posts"]`).
    pub path_cmd: &'static [&'static str],
    /// Hidden alias for the leaf segment (the underscore spelling), if it
    /// differs from the kebab form (e.g. `advanced_search`).
    pub leaf_alias: Option<&'static str>,
    pub method: &'static str,
    /// Path template with `{param}` placeholders.
    pub path: &'static str,
    /// Path params in template order → positional args.
    pub path_params: &'static [&'static str],
    pub query: &'static [Param],
    /// Scalar body fields exposed as typed flags.
    pub body: &'static [Param],
    /// Whether the endpoint accepts a JSON body at all (enables `--body`).
    pub has_body: bool,
    pub summary: &'static str,
}

impl Endpoint {
    /// The leaf command name (last path segment).
    pub fn leaf(&self) -> &'static str {
        self.path_cmd.last().copied().unwrap_or(self.platform)
    }

    /// Human-readable signature, e.g. `reddit subreddits posts <subreddit>`.
    pub fn signature(&self) -> String {
        let mut s = self.path_cmd.join(" ");
        for p in self.path_params {
            s.push_str(&format!(" <{p}>"));
        }
        s
    }
}

// ===== Command-tree construction =====

/// Build the full clap command tree from [`ENDPOINTS`] plus the hand-written
/// `raw`, `config`, `commands`, and `completions` subcommands.
pub fn build_command(version: &'static str) -> Command {
    let mut root = Command::new("scrapebadger")
        .version(version)
        .about("CLI for the ScrapeBadger web-scraping API")
        .propagate_version(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("api_key")
                .long("api-key")
                .global(true)
                .env("SCRAPEBADGER_API_KEY")
                .hide_env_values(true)
                .help("API key (overrides SCRAPEBADGER_API_KEY)"),
        )
        .arg(
            Arg::new("help_all")
                .long("help-all")
                .action(ArgAction::SetTrue)
                .help("Print every command in the entire tree and exit"),
        );

    for g in global_flags() {
        root = root.arg(g);
    }

    // One subcommand tree per platform, built from the endpoint table.
    for platform in platforms() {
        root = root.subcommand(build_platform(platform));
    }

    root.subcommand(raw_command())
        .subcommand(config_command())
        .subcommand(commands_command())
        .subcommand(completions_command())
        .subcommand(man_command())
}

/// Distinct platforms in table order.
fn platforms() -> Vec<&'static str> {
    let mut v: Vec<&'static str> = Vec::new();
    for e in ENDPOINTS {
        if !v.contains(&e.platform) {
            v.push(e.platform);
        }
    }
    v
}

/// Output/UX flags shared by every leaf (declared global on the root).
fn global_flags() -> Vec<Arg> {
    vec![
        Arg::new("output")
            .long("output")
            .short('o')
            .global(true)
            .value_parser(["json", "jsonl", "raw"])
            .default_value("json")
            .help("Output format"),
        Arg::new("select")
            .long("select")
            .global(true)
            .value_name("PATH")
            .help("Project a field path from the response, e.g. '.posts[].title'"),
        Arg::new("all")
            .long("all")
            .global(true)
            .action(ArgAction::SetTrue)
            .help("Auto-follow pagination cursors until exhausted (best-effort)"),
        Arg::new("explain")
            .long("explain")
            .global(true)
            .action(ArgAction::SetTrue)
            .help("Print the resolved HTTP request instead of sending it"),
        Arg::new("curl")
            .long("curl")
            .global(true)
            .action(ArgAction::SetTrue)
            .help("Print an equivalent curl command instead of sending it"),
        Arg::new("reveal")
            .long("reveal")
            .global(true)
            .action(ArgAction::SetTrue)
            .help("With --curl, show the real API key instead of a placeholder"),
    ]
}

/// Build a platform subcommand and everything under it.
fn build_platform(platform: &'static str) -> Command {
    // Collect this platform's endpoints, build the trie of command paths.
    let eps: Vec<&'static Endpoint> = ENDPOINTS
        .iter()
        .filter(|e| e.platform == platform)
        .collect();
    let mut cmd = Command::new(platform).about(platform_about(platform));
    // Depth-1 children (groups or direct leaves), built recursively.
    cmd = attach_children(cmd, &eps, 1);
    // after_help: every descendant leaf at a glance.
    cmd = cmd.after_help(subtree_listing(&eps, 1));
    if has_subgroups(&eps, 1) {
        cmd = cmd.subcommand_required(true).arg_required_else_help(true);
    }
    cmd
}

/// Whether, at `depth`, this endpoint set branches into named subgroups
/// (rather than every endpoint being a direct leaf).
fn has_subgroups(eps: &[&'static Endpoint], depth: usize) -> bool {
    eps.iter().any(|e| e.path_cmd.len() > depth + 1)
}

/// Recursively attach children at `depth` (index into `path_cmd`).
fn attach_children(mut parent: Command, eps: &[&'static Endpoint], depth: usize) -> Command {
    // Partition endpoints by their segment at `depth`.
    let mut seen: Vec<&'static str> = Vec::new();
    for e in eps {
        if let Some(seg) = e.path_cmd.get(depth) {
            if !seen.contains(seg) {
                seen.push(seg);
            }
        }
    }
    for seg in seen {
        let group: Vec<&'static Endpoint> = eps
            .iter()
            .copied()
            .filter(|e| e.path_cmd.get(depth) == Some(&seg))
            .collect();
        // A leaf is an endpoint whose path ends exactly at this segment.
        let leaf = group
            .iter()
            .copied()
            .find(|e| e.path_cmd.len() == depth + 1);
        if let (1, Some(leaf)) = (group.len(), leaf) {
            parent = parent.subcommand(build_leaf(leaf));
        } else {
            // A named subgroup with deeper commands.
            let mut sub = Command::new(seg)
                .about(group_about(seg, &group))
                .subcommand_required(true)
                .arg_required_else_help(true)
                .after_help(subtree_listing(&group, depth + 1));
            sub = attach_children(sub, &group, depth + 1);
            parent = parent.subcommand(sub);
        }
    }
    parent
}

/// Build a leaf command for one endpoint: positionals + flags + alias.
fn build_leaf(e: &'static Endpoint) -> Command {
    let mut cmd = Command::new(e.leaf()).about(e.summary);
    if let Some(alias) = e.leaf_alias {
        cmd = cmd.alias(alias); // hidden by default
    }
    // Positional path params (required, in order). The arg id doubles as the
    // value name (clap renders it as `<param>`).
    for p in e.path_params {
        cmd = cmd.arg(Arg::new(*p).required(true));
    }
    // Query + body flags.
    for p in e.query.iter().chain(e.body.iter()) {
        cmd = cmd.arg(param_arg(p));
    }
    if e.has_body {
        cmd = cmd
            .arg(
                Arg::new("body")
                    .long("body")
                    .value_name("JSON")
                    .help("Raw JSON request body (typed flags override its fields)"),
            )
            .arg(
                Arg::new("body_file")
                    .long("body-file")
                    .value_name("PATH")
                    .help("Read the JSON request body from a file"),
            );
    }
    cmd
}

/// Build a clap `Arg` for a query/body parameter.
fn param_arg(p: &'static Param) -> Arg {
    let mut a = Arg::new(p.name).long(p.flag).help(p.help);
    match p.ty {
        Ty::Bool => a = a.action(ArgAction::SetTrue),
        Ty::StrArray => {
            a = a
                .action(ArgAction::Append)
                .value_delimiter(',')
                .value_name("VALUE");
        }
        Ty::Int | Ty::Num => a = a.value_name("N"),
        Ty::Str => a = a.value_name("VALUE"),
    }
    if p.required && p.ty != Ty::Bool {
        a = a.required(true);
    }
    a
}

/// Aligned one-line listing of every descendant leaf, for `after_help`.
fn subtree_listing(eps: &[&'static Endpoint], from_depth: usize) -> String {
    let mut rows: Vec<(String, &str)> = eps
        .iter()
        .map(|e| {
            let mut sig = e.path_cmd[from_depth..].join(" ");
            for p in e.path_params {
                sig.push_str(&format!(" <{p}>"));
            }
            (sig, e.summary)
        })
        .collect();
    rows.sort();
    let width = rows.iter().map(|(s, _)| s.len()).max().unwrap_or(0);
    let mut out = String::from("All commands:\n");
    for (sig, summary) in rows {
        out.push_str(&format!("  {sig:<width$}  {summary}\n"));
    }
    out.push_str("\nRun any command with --help for its arguments and flags.");
    out
}

fn platform_about(platform: &'static str) -> String {
    let n = ENDPOINTS.iter().filter(|e| e.platform == platform).count();
    format!("{platform} endpoints ({n})")
}

fn group_about(seg: &'static str, group: &[&'static Endpoint]) -> String {
    format!("{seg} ({} commands)", group.len())
}

// ===== Auxiliary subcommands =====

fn raw_command() -> Command {
    Command::new("raw")
        .about("Low-level request to any endpoint (covers the full API surface)")
        .arg(
            Arg::new("method")
                .long("method")
                .default_value("GET")
                .help("HTTP method"),
        )
        .arg(
            Arg::new("path")
                .required(true)
                .help("Request path, e.g. /v1/amazon/products/B08N5WRWNW"),
        )
        .arg(
            Arg::new("query")
                .long("query")
                .short('q')
                .value_name("KEY=VALUE")
                .action(ArgAction::Append)
                .help("Repeatable query parameter"),
        )
        .arg(
            Arg::new("body")
                .long("body")
                .short('d')
                .value_name("JSON")
                .help("JSON request body (for POST/PATCH)"),
        )
}

fn config_command() -> Command {
    Command::new("config")
        .about("Manage the global config (stored API key)")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("set-key")
                .about("Store an API key in the global config file")
                .arg(Arg::new("key").help("The API key; read from stdin if omitted")),
        )
        .subcommand(Command::new("path").about("Print the config file path"))
        .subcommand(Command::new("show").about("Show the current config (API key masked)"))
}

fn commands_command() -> Command {
    Command::new("commands")
        .about("Print every command (the full flat tree) for grepping")
        .arg(
            Arg::new("platform")
                .long("platform")
                .help("Restrict to one platform"),
        )
}

fn completions_command() -> Command {
    Command::new("completions")
        .about("Generate a shell completion script")
        .arg(
            Arg::new("shell")
                .required(true)
                .value_parser(["bash", "zsh", "fish", "powershell", "elvish"])
                .help("Target shell"),
        )
}

fn man_command() -> Command {
    Command::new("man")
        .about("Generate roff man pages (one per command) into a directory")
        .arg(
            Arg::new("out_dir")
                .required(true)
                .help("Output directory for the .1 man pages"),
        )
}

// ===== Dispatch helpers =====

/// Walk nested subcommand matches to the selected leaf, collecting the command
/// path (canonical names, not aliases).
pub fn find_leaf<'a>(m: &'a ArgMatches, path: &mut Vec<String>) -> &'a ArgMatches {
    if let Some((name, sub)) = m.subcommand() {
        path.push(name.to_string());
        find_leaf(sub, path)
    } else {
        m
    }
}

/// Look up the endpoint whose command path matches `path`.
pub fn lookup(path: &[String]) -> Option<&'static Endpoint> {
    ENDPOINTS.iter().find(|e| {
        e.path_cmd.len() == path.len() && e.path_cmd.iter().zip(path).all(|(a, b)| a == b)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn endpoint_count_is_269() {
        assert_eq!(ENDPOINTS.len(), 269);
    }

    #[test]
    fn no_duplicate_command_paths() {
        let mut seen = BTreeSet::new();
        for e in ENDPOINTS {
            let key = e.path_cmd.join(" ");
            assert!(seen.insert(key.clone()), "duplicate command path: {key}");
        }
    }

    #[test]
    fn no_runnable_parents() {
        // No command path may be a strict prefix of another — every node is
        // either a pure group or a leaf, never both.
        for a in ENDPOINTS {
            for b in ENDPOINTS {
                if b.path_cmd.len() > a.path_cmd.len()
                    && b.path_cmd[..a.path_cmd.len()] == *a.path_cmd
                {
                    panic!("runnable-parent: {}", a.path_cmd.join(" "));
                }
            }
        }
    }

    #[test]
    fn tree_builds_and_validates() {
        // clap panics on malformed command definitions; this exercises the
        // whole tree construction.
        build_command("0.0.0").debug_assert();
    }

    #[test]
    fn every_leaf_resolves() {
        for e in ENDPOINTS {
            let path: Vec<String> = e.path_cmd.iter().map(|s| s.to_string()).collect();
            assert!(lookup(&path).is_some(), "unresolved: {}", e.signature());
        }
    }
}
