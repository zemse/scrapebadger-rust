//! Lenient `deserialize_with` helpers for scraped data.
//!
//! Upstream scraping responses don't always match their OpenAPI spec: numbers
//! and booleans frequently arrive as strings, and a field the spec calls a
//! string occasionally arrives as an object. The generated response structs
//! route every scalar field through these helpers so a shape mismatch degrades
//! gracefully (wrong scalar → coerced; unexpected object → JSON-stringified)
//! instead of failing the whole response. Unknown fields are still captured by
//! each struct's `extra` catch-all.

// Not every scalar variant is exercised by the current specs, but all are kept
// so regeneration stays stable as fields come and go.
#![allow(dead_code)]

use serde::{Deserialize, Deserializer};
use serde_json::Value;

fn value<'de, D: Deserializer<'de>>(d: D) -> Result<Value, D::Error> {
    Value::deserialize(d)
}

/// `Option<i64>` accepting a JSON number, a numeric string, or null.
pub(crate) fn opt_i64<'de, D: Deserializer<'de>>(d: D) -> Result<Option<i64>, D::Error> {
    Ok(match value(d)? {
        Value::Null => None,
        Value::Number(n) => n.as_i64().or_else(|| n.as_f64().map(|f| f as i64)),
        Value::String(s) => s
            .trim()
            .parse::<i64>()
            .ok()
            .or_else(|| s.trim().parse::<f64>().ok().map(|f| f as i64)),
        _ => None,
    })
}

/// `Option<f64>` accepting a JSON number, a numeric string, or null.
pub(crate) fn opt_f64<'de, D: Deserializer<'de>>(d: D) -> Result<Option<f64>, D::Error> {
    Ok(match value(d)? {
        Value::Null => None,
        Value::Number(n) => n.as_f64(),
        Value::String(s) => s.trim().parse::<f64>().ok(),
        _ => None,
    })
}

/// `Option<bool>` accepting a JSON bool, `"true"`/`"false"`/`"1"`/`"0"`, a
/// number (0 = false), or null.
pub(crate) fn opt_bool<'de, D: Deserializer<'de>>(d: D) -> Result<Option<bool>, D::Error> {
    Ok(match value(d)? {
        Value::Null => None,
        Value::Bool(b) => Some(b),
        Value::Number(n) => n.as_f64().map(|f| f != 0.0),
        Value::String(s) => match s.trim().to_ascii_lowercase().as_str() {
            "true" | "1" | "yes" => Some(true),
            "false" | "0" | "no" | "" => Some(false),
            _ => None,
        },
        _ => None,
    })
}

/// `Option<String>` accepting a string, or coercing a number/bool to its text,
/// or JSON-stringifying an unexpected object/array (rather than failing).
pub(crate) fn opt_string<'de, D: Deserializer<'de>>(d: D) -> Result<Option<String>, D::Error> {
    Ok(match value(d)? {
        Value::Null => None,
        Value::String(s) => Some(s),
        Value::Bool(b) => Some(b.to_string()),
        Value::Number(n) => Some(n.to_string()),
        other => Some(other.to_string()),
    })
}

/// Required `i64` (defaults to 0 on null/absent/uncoercible).
pub(crate) fn req_i64<'de, D: Deserializer<'de>>(d: D) -> Result<i64, D::Error> {
    Ok(opt_i64(d)?.unwrap_or_default())
}

/// Required `f64` (defaults to 0.0).
pub(crate) fn req_f64<'de, D: Deserializer<'de>>(d: D) -> Result<f64, D::Error> {
    Ok(opt_f64(d)?.unwrap_or_default())
}

/// Required `bool` (defaults to false).
pub(crate) fn req_bool<'de, D: Deserializer<'de>>(d: D) -> Result<bool, D::Error> {
    Ok(opt_bool(d)?.unwrap_or_default())
}

/// Required `String` (defaults to empty).
pub(crate) fn req_string<'de, D: Deserializer<'de>>(d: D) -> Result<String, D::Error> {
    Ok(opt_string(d)?.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct S {
        #[serde(default, deserialize_with = "super::opt_i64")]
        n: Option<i64>,
        #[serde(default, deserialize_with = "super::opt_string")]
        s: Option<String>,
        #[serde(default, deserialize_with = "super::opt_bool")]
        b: Option<bool>,
    }

    #[test]
    fn number_as_string_coerces() {
        let s: S = serde_json::from_str(r#"{"n":"2300","b":"true"}"#).unwrap();
        assert_eq!(s.n, Some(2300));
        assert_eq!(s.b, Some(true));
    }

    #[test]
    fn real_number_still_works() {
        let s: S = serde_json::from_str(r#"{"n":42,"b":false}"#).unwrap();
        assert_eq!(s.n, Some(42));
        assert_eq!(s.b, Some(false));
    }

    #[test]
    fn object_in_string_field_is_stringified() {
        let s: S =
            serde_json::from_str(r#"{"s":{"amount":"20.0","currency_code":"EUR"}}"#).unwrap();
        assert!(s.s.unwrap().contains("currency_code"));
    }

    #[test]
    fn nulls_and_missing_are_none() {
        let s: S = serde_json::from_str(r#"{"n":null}"#).unwrap();
        assert_eq!(s.n, None);
        assert_eq!(s.s, None);
    }
}
