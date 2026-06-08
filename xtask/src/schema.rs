//! OpenAPI schema -> Rust type mapping, with inline-object synthesis.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Map, Value};

use crate::naming::{sanitize_field, sanitize_type, to_pascal};

pub struct SchemaCtx {
    root: Value,
    /// Named component types (PascalCase) -> generated definition.
    model_defs: BTreeMap<String, String>,
    /// Param structs, in operation order.
    extra_defs: Vec<String>,
    /// Names of known component schemas (so unknown $refs fall back to Value).
    known: BTreeSet<String>,
}

impl SchemaCtx {
    pub fn new(root: &Value) -> Self {
        Self {
            root: root.clone(),
            model_defs: BTreeMap::new(),
            extra_defs: Vec::new(),
            known: BTreeSet::new(),
        }
    }

    pub fn register_named(&mut self, raw_name: &str) {
        self.known.insert(sanitize_type(raw_name));
    }

    pub fn push_def(&mut self, def: String) {
        self.extra_defs.push(def);
    }

    pub fn take_defs(&mut self) -> Vec<String> {
        let mut out: Vec<String> = self
            .model_defs
            .values()
            .filter(|d| !d.is_empty())
            .cloned()
            .collect();
        out.append(&mut self.extra_defs);
        out
    }

    /// Resolve a single `$ref` (if present) to the referenced schema value.
    pub fn resolve(&self, schema: &Value) -> Value {
        if let Some(r) = schema.get("$ref").and_then(Value::as_str) {
            if let Some(target) = self.deref(r) {
                return target;
            }
        }
        schema.clone()
    }

    fn deref(&self, reference: &str) -> Option<Value> {
        let ptr = reference.trim_start_matches('#');
        self.root.pointer(ptr).cloned()
    }

    /// Emit a top-level component schema under `name`.
    pub fn emit_named(&mut self, name: &str, schema: &Value) {
        if self.model_defs.contains_key(name) {
            return;
        }
        if schema.get("enum").is_some() {
            // Enums are surfaced as plain strings for ergonomics/robustness.
            self.model_defs
                .insert(name.to_string(), format!("pub type {name} = String;"));
            return;
        }
        if schema.get("properties").is_some()
            || schema.get("type").and_then(Value::as_str) == Some("object")
        {
            if schema.get("properties").is_some() {
                self.emit_struct(name, schema);
            } else {
                // object with no properties -> open map or value
                let ty = self.object_fallback(schema, name);
                self.model_defs
                    .insert(name.to_string(), format!("pub type {name} = {ty};"));
            }
            return;
        }
        // Array / union / primitive alias.
        let ty = self.rust_type(schema, name);
        let ty = if ty == name { "Value".to_string() } else { ty };
        self.model_defs
            .insert(name.to_string(), format!("pub type {name} = {ty};"));
    }

    /// Map any schema to a Rust type string, registering inline structs as needed.
    pub fn rust_type(&mut self, schema: &Value, hint: &str) -> String {
        let schema = match schema.as_object() {
            Some(_) => schema,
            None => return "Value".to_string(),
        };

        // $ref
        if let Some(r) = schema.get("$ref").and_then(Value::as_str) {
            let name = sanitize_type(r.rsplit('/').next().unwrap_or(r));
            if self.known.contains(&name) {
                return name;
            }
            // Reference into something we did not pre-register: emit it now.
            if let Some(target) = self.deref(r) {
                self.known.insert(name.clone());
                self.emit_named(&name, &target);
                return name;
            }
            return "Value".to_string();
        }

        // Nullable via anyOf/oneOf.
        for key in ["anyOf", "oneOf"] {
            if let Some(branches) = schema.get(key).and_then(Value::as_array) {
                let non_null: Vec<&Value> =
                    branches.iter().filter(|b| !is_null_schema(b)).collect();
                match non_null.len() {
                    0 => return "Value".to_string(),
                    1 => {
                        let inner = self.rust_type(non_null[0], hint);
                        return wrap_option(inner);
                    }
                    _ => return "Value".to_string(),
                }
            }
        }

        // Nullable via type array, e.g. ["string","null"].
        if let Some(types) = schema.get("type").and_then(Value::as_array) {
            let nullable = types.iter().any(|t| t.as_str() == Some("null"));
            let base = types
                .iter()
                .find_map(|t| t.as_str().filter(|s| *s != "null"));
            let inner = match base {
                Some(t) => self.scalar_or_complex(t, schema, hint),
                None => "Value".to_string(),
            };
            return if nullable { wrap_option(inner) } else { inner };
        }

        // Nullable via OpenAPI 3.0 `nullable: true`.
        let nullable = schema
            .get("nullable")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        let ty = match schema.get("type").and_then(Value::as_str) {
            Some(t) => self.scalar_or_complex(t, schema, hint),
            None => {
                if schema.get("properties").is_some() {
                    self.emit_struct(hint, schema);
                    hint.to_string()
                } else {
                    "Value".to_string()
                }
            }
        };
        if nullable {
            wrap_option(ty)
        } else {
            ty
        }
    }

    fn scalar_or_complex(&mut self, ty: &str, schema: &Value, hint: &str) -> String {
        match ty {
            "string" => "String".to_string(),
            "integer" => "i64".to_string(),
            "number" => "f64".to_string(),
            "boolean" => "bool".to_string(),
            "array" => {
                let items = schema.get("items").cloned().unwrap_or(Value::Null);
                let inner = self.rust_type(&items, &format!("{hint}Item"));
                format!("Vec<{inner}>")
            }
            "object" => {
                if schema.get("properties").is_some() {
                    self.emit_struct(hint, schema);
                    hint.to_string()
                } else {
                    self.object_fallback(schema, hint)
                }
            }
            _ => "Value".to_string(),
        }
    }

    /// `object` schemas without `properties`: typed map or open value.
    fn object_fallback(&mut self, schema: &Value, hint: &str) -> String {
        match schema.get("additionalProperties") {
            Some(Value::Bool(true)) | None => "HashMap<String, Value>".to_string(),
            Some(Value::Bool(false)) => "Value".to_string(),
            Some(ap) => {
                let inner = self.rust_type(ap, &format!("{hint}Value"));
                format!("HashMap<String, {inner}>")
            }
        }
    }

    /// Generate a struct for an object schema with `properties`.
    fn emit_struct(&mut self, name: &str, schema: &Value) {
        if self.model_defs.contains_key(name) {
            return;
        }
        // Reserve to break self-referential cycles.
        self.model_defs.insert(name.to_string(), String::new());
        self.known.insert(name.to_string());

        let empty = Map::new();
        let props = schema
            .get("properties")
            .and_then(Value::as_object)
            .unwrap_or(&empty);

        let mut body = String::new();
        if let Some(desc) = schema.get("description").and_then(Value::as_str) {
            for line in desc.lines().take(3) {
                body.push_str(&format!("/// {}\n", line.trim()));
            }
        }
        body.push_str("#[derive(Debug, Clone, Default, Serialize, Deserialize)]\n");
        body.push_str("#[serde(default)]\n");
        body.push_str(&format!("pub struct {name} {{\n"));

        let mut idents: BTreeSet<String> = BTreeSet::new();
        for (pname, psch) in props {
            let field_hint = format!("{name}{}", to_pascal(pname));
            let raw = self.rust_type(psch, &field_hint);
            let ty = field_type(&raw, name);
            if let Some(desc) = psch.get("description").and_then(Value::as_str) {
                for line in desc.lines().take(2) {
                    body.push_str(&format!("    /// {}\n", line.trim()));
                }
            }
            let (ident, rename) = sanitize_field(pname);
            if rename {
                body.push_str(&format!("    #[serde(rename = \"{}\")]\n", pname));
            }
            body.push_str(&format!("    pub {ident}: {ty},\n"));
            idents.insert(ident);
        }

        // Lossless forward-compat: capture any response fields not in the spec
        // into a flattened catch-all instead of silently dropping them. Pick a
        // field name that does not collide with a modeled field.
        if let Some(catch_all) = ["extra", "extra_fields", "additional_fields"]
            .into_iter()
            .find(|c| !idents.contains(*c))
        {
            body.push_str("    /// Fields present in the response but not in the spec.\n");
            body.push_str("    #[serde(flatten)]\n");
            body.push_str(&format!("    pub {catch_all}: HashMap<String, Value>,\n"));
        }
        body.push('}');

        self.model_defs.insert(name.to_string(), body);
    }

    /// Scalar Rust type for a query parameter, plus whether it is an array.
    ///
    /// String parameters with a fixed `enum` value-set are surfaced as generated
    /// Rust enums (named after `hint`) for type safety; everything else maps to a
    /// scalar. Enums are generated only for *query inputs* — never response or
    /// model fields, where an unrecognized future value must still deserialize.
    pub fn query_param_type(&mut self, schema: &Value, hint: &str) -> (String, bool) {
        let schema = self.resolve(schema);
        // dig through anyOf nullable
        let effective = unwrap_anyof_nullable(&schema);
        let t = effective.get("type");
        if t.and_then(Value::as_str) == Some("array") {
            let items = effective.get("items").cloned().unwrap_or(Value::Null);
            let items = unwrap_anyof_nullable(&items);
            if let Some(values) = string_enum_values(&items) {
                let doc = items.get("description").and_then(Value::as_str);
                return (self.emit_param_enum(hint, &values, doc), true);
            }
            let elem = scalar_name(items.get("type").and_then(Value::as_str));
            return (elem, true);
        }
        if let Some(values) = string_enum_values(&effective) {
            let doc = effective.get("description").and_then(Value::as_str);
            return (self.emit_param_enum(hint, &values, doc), false);
        }
        // type may be array like ["string","null"]
        let scalar = match t {
            Some(Value::String(s)) => scalar_name(Some(s.as_str())),
            Some(Value::Array(arr)) => {
                let s = arr.iter().find_map(|v| v.as_str().filter(|x| *x != "null"));
                scalar_name(s)
            }
            _ => "String".to_string(),
        };
        (scalar, false)
    }

    /// Pick a type name not already taken by a model/enum, suffixing `2`, `3`, …
    /// on collision so generated enum names never clash.
    fn unique_type_name(&self, base: &str) -> String {
        let taken = |n: &str| self.known.contains(n) || self.model_defs.contains_key(n);
        if !taken(base) {
            return base.to_string();
        }
        let mut i = 2;
        loop {
            let cand = format!("{base}{i}");
            if !taken(&cand) {
                return cand;
            }
            i += 1;
        }
    }

    /// Emit a `#[derive(...)]` enum for a fixed string value-set and return its
    /// type name. Each variant serializes to (and `Display`s as) its wire value.
    fn emit_param_enum(&mut self, hint: &str, values: &[String], doc: Option<&str>) -> String {
        let name = self.unique_type_name(&sanitize_type(hint));
        // Reserve the name immediately so concurrent collisions can't reuse it.
        self.known.insert(name.clone());

        // Unique variant identifiers (preserve wire value via serde rename).
        let mut seen: BTreeSet<String> = BTreeSet::new();
        let mut variants: Vec<(String, String)> = Vec::new();
        for v in values {
            let mut ident = variant_ident(v);
            if seen.contains(&ident) {
                let mut i = 2;
                while seen.contains(&format!("{ident}{i}")) {
                    i += 1;
                }
                ident = format!("{ident}{i}");
            }
            seen.insert(ident.clone());
            variants.push((ident, v.clone()));
        }

        let mut s = String::new();
        match doc {
            Some(d) => {
                for line in d.lines().take(2) {
                    s.push_str(&format!("/// {}\n", line.trim()));
                }
            }
            None => s.push_str("/// Allowed values for a fixed-value query parameter.\n"),
        }
        s.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]\n");
        // Forward-compat: the API may add values; downstream matches need a wildcard.
        s.push_str("#[non_exhaustive]\n");
        s.push_str(&format!("pub enum {name} {{\n"));
        for (ident, wire) in &variants {
            s.push_str(&format!("    /// `{wire}`\n"));
            if ident != wire {
                s.push_str(&format!("    #[serde(rename = \"{wire}\")]\n"));
            }
            s.push_str(&format!("    {ident},\n"));
        }
        s.push_str("}\n\n");
        s.push_str(&format!("impl std::fmt::Display for {name} {{\n"));
        s.push_str("    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n");
        s.push_str("        f.write_str(match self {\n");
        for (ident, wire) in &variants {
            s.push_str(&format!("            {name}::{ident} => \"{wire}\",\n"));
        }
        s.push_str("        })\n    }\n}");

        self.model_defs.insert(name.clone(), s);
        name
    }

    /// Resolve the success-response Rust type for an operation.
    pub fn response_type(&mut self, op: &Map<String, Value>, hint: &str) -> String {
        let responses = match op.get("responses").and_then(Value::as_object) {
            Some(r) => r,
            None => return "Value".to_string(),
        };
        let schema = ["200", "201", "2XX", "default"].iter().find_map(|code| {
            responses
                .get(*code)
                .and_then(|r| r.pointer("/content/application~1json/schema"))
                .cloned()
        });
        match schema {
            Some(s) => self.rust_type(&s, hint),
            None => "Value".to_string(),
        }
    }
}

/// Apply field-level wrapping rules: arrays/maps stay as-is (default empty),
/// everything else becomes `Option<...>`; self-references are boxed.
fn field_type(raw: &str, struct_name: &str) -> String {
    if raw.starts_with("Vec<") || raw.starts_with("HashMap<") {
        return raw.to_string();
    }
    if raw == struct_name {
        return format!("Option<Box<{struct_name}>>");
    }
    if raw == format!("Option<{struct_name}>") {
        return format!("Option<Box<{struct_name}>>");
    }
    if raw.starts_with("Option<") {
        return raw.to_string();
    }
    format!("Option<{raw}>")
}

fn wrap_option(inner: String) -> String {
    if inner.starts_with("Option<") {
        inner
    } else {
        format!("Option<{inner}>")
    }
}

fn is_null_schema(v: &Value) -> bool {
    match v.get("type") {
        Some(Value::String(s)) => s == "null",
        Some(Value::Array(a)) => a.iter().all(|x| x.as_str() == Some("null")),
        _ => false,
    }
}

fn unwrap_anyof_nullable(schema: &Value) -> Value {
    for key in ["anyOf", "oneOf"] {
        if let Some(branches) = schema.get(key).and_then(Value::as_array) {
            if let Some(nn) = branches.iter().find(|b| !is_null_schema(b)) {
                return nn.clone();
            }
        }
    }
    schema.clone()
}

/// Extract a fixed string `enum` value-set from a schema, if it is one.
///
/// Returns `Some(values)` only when every `enum` entry is a string and the
/// schema is string-typed (or untyped). Integer/other enums are left as scalars.
fn string_enum_values(schema: &Value) -> Option<Vec<String>> {
    let en = schema.get("enum")?.as_array()?;
    let is_string = schema
        .get("type")
        .and_then(Value::as_str)
        .map(|t| t == "string")
        .unwrap_or(true);
    if !is_string {
        return None;
    }
    let vals: Vec<String> = en
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();
    if !vals.is_empty() && vals.len() == en.len() {
        Some(vals)
    } else {
        None
    }
}

/// A valid PascalCase enum-variant identifier for a wire value.
fn variant_ident(v: &str) -> String {
    let mut id = to_pascal(v);
    if id.is_empty() {
        id = "Empty".to_string();
    }
    if id
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false)
    {
        id = format!("V{id}");
    }
    if id == "Self" {
        id = "SelfValue".to_string();
    }
    id
}

fn scalar_name(t: Option<&str>) -> String {
    match t {
        Some("integer") => "i64".to_string(),
        Some("number") => "f64".to_string(),
        Some("boolean") => "bool".to_string(),
        _ => "String".to_string(),
    }
}
