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
                let non_null: Vec<&Value> = branches.iter().filter(|b| !is_null_schema(b)).collect();
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
            let base = types.iter().find_map(|t| t.as_str().filter(|s| *s != "null"));
            let inner = match base {
                Some(t) => self.scalar_or_complex(t, schema, hint),
                None => "Value".to_string(),
            };
            return if nullable { wrap_option(inner) } else { inner };
        }

        // Nullable via OpenAPI 3.0 `nullable: true`.
        let nullable = schema.get("nullable").and_then(Value::as_bool).unwrap_or(false);

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
        }
        body.push('}');

        self.model_defs.insert(name.to_string(), body);
    }

    /// Scalar Rust type for a query parameter, plus whether it is an array.
    pub fn query_param_type(&self, schema: &Value) -> (String, bool) {
        let schema = self.resolve(schema);
        // dig through anyOf nullable
        let effective = unwrap_anyof_nullable(&schema);
        let t = effective.get("type");
        if t.and_then(Value::as_str) == Some("array") {
            let items = effective.get("items").cloned().unwrap_or(Value::Null);
            let items = unwrap_anyof_nullable(&items);
            let elem = scalar_name(items.get("type").and_then(Value::as_str));
            return (elem, true);
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

    /// Resolve the success-response Rust type for an operation.
    pub fn response_type(&mut self, op: &Map<String, Value>, hint: &str) -> String {
        let responses = match op.get("responses").and_then(Value::as_object) {
            Some(r) => r,
            None => return "Value".to_string(),
        };
        let schema = ["200", "201", "2XX", "default"]
            .iter()
            .find_map(|code| {
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

fn scalar_name(t: Option<&str>) -> String {
    match t {
        Some("integer") => "i64".to_string(),
        Some("number") => "f64".to_string(),
        Some("boolean") => "bool".to_string(),
        _ => "String".to_string(),
    }
}
