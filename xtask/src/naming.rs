//! Identifier and method-name derivation.

use heck::{ToPascalCase, ToSnakeCase};

const KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false", "fn",
    "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
    "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
    "use", "where", "while", "async", "await", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try", "union",
];

/// Platforms whose name appears redundantly inside operationIds; stripped from
/// generated method names (e.g. `get_amazon_product` -> `get_product`).
const STRIP_TOKENS: &[&str] = &["amazon", "vinted", "google"];

pub fn to_pascal(s: &str) -> String {
    let p = s.to_pascal_case();
    if p.is_empty() {
        "Unnamed".to_string()
    } else {
        p
    }
}

pub fn to_snake(s: &str) -> String {
    s.to_snake_case()
}

/// A Rust type name (PascalCase, valid identifier).
pub fn sanitize_type(s: &str) -> String {
    let mut p = to_pascal(s);
    if p.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(true) {
        p = format!("T{p}");
    }
    p
}

/// A snake_case identifier, keyword-escaped.
pub fn sanitize_ident(s: &str) -> String {
    let mut id = to_snake(s);
    if id.is_empty() {
        id = "field".to_string();
    }
    if id
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false)
    {
        id = format!("n{id}");
    }
    if KEYWORDS.contains(&id.as_str()) {
        id.push('_');
    }
    id
}

/// Returns `(ident, needs_serde_rename)` for a struct/param field.
pub fn sanitize_field(name: &str) -> (String, bool) {
    let ident = sanitize_ident(name);
    let needs_rename = ident != name;
    (ident, needs_rename)
}

/// Derive a clean method name from an operationId (falling back to the path).
///
/// FastAPI auto-IDs look like `flights_search_api_v1_flights_search_get`; the
/// clean handler name is the prefix before the embedded `_v1_`/`_api_v1_`.
/// camelCase IDs (`getAmazonProduct`) are just snake-cased.
pub fn method_name(op_id: &str, path: &str, module: &str) -> String {
    let base = if op_id.is_empty() {
        path_to_name(path)
    } else if let Some(idx) = find_marker(op_id) {
        op_id[..idx].to_string()
    } else {
        op_id.to_string()
    };

    let snake = to_snake(&base);
    let stripped: Vec<&str> = snake
        .split('_')
        .filter(|seg| !(seg.is_empty() || STRIP_TOKENS.contains(&module) && *seg == module))
        .collect();
    let mut name = stripped.join("_");
    if name.is_empty() {
        name = to_snake(&path_to_name(path));
    }
    if name.is_empty() {
        name = "call".to_string();
    }
    if KEYWORDS.contains(&name.as_str()) {
        name.push('_');
    }
    name
}

fn find_marker(s: &str) -> Option<usize> {
    s.find("_api_v1_").or_else(|| s.find("_v1_"))
}

/// Build a name from a path when no usable operationId exists.
fn path_to_name(path: &str) -> String {
    path.split('/')
        .filter(|seg| !seg.is_empty() && !seg.starts_with('{') && *seg != "v1" && *seg != "api")
        .collect::<Vec<_>>()
        .join("_")
}
