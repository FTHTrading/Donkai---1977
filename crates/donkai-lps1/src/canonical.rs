use crate::error::{Lps1Error, Result};
use serde::Serialize;
use serde_json::{Map, Value};

/// Canonicalizes any serializable LPS-1 object into deterministic JSON bytes.
/// Rules:
/// 1. UTF-8 encoded
/// 2. Object keys sorted recursively in lexicographical order
/// 3. Compact formatting with no insignificant whitespace
/// 4. Preserves exact string prose and casing
pub fn canonicalize<T: Serialize>(value: &T) -> Result<CanonicalBytes> {
    let json_val = serde_json::to_value(value)?;
    let canonical_val = sort_json_value(json_val);
    let canonical_string = serde_json::to_string(&canonical_val)?;
    Ok(CanonicalBytes(canonical_string.into_bytes()))
}

/// Canonicalizes a raw JSON string or Value into deterministic format.
pub fn canonicalize_json_str(raw_json: &str) -> Result<CanonicalBytes> {
    let json_val: Value = serde_json::from_str(raw_json)
        .map_err(|e| Lps1Error::Validation(format!("Invalid JSON for canonicalization: {}", e)))?;
    let canonical_val = sort_json_value(json_val);
    let canonical_string = serde_json::to_string(&canonical_val)?;
    Ok(CanonicalBytes(canonical_string.into_bytes()))
}

use unicode_normalization::UnicodeNormalization;

fn sort_json_value(val: Value) -> Value {
    match val {
        Value::String(s) => {
            let normalized_crlf = s.replace("\r\n", "\n").replace('\r', "\n");
            let nfc_string: String = normalized_crlf.nfc().collect();
            Value::String(nfc_string)
        }
        Value::Object(map) => {
            let mut sorted_map = Map::new();
            let mut keys: Vec<String> = map.keys().cloned().collect();
            keys.sort(); // Lexicographical sort
            for key in keys {
                if let Some(v) = map.get(&key) {
                    let key_nfc: String = key.nfc().collect();
                    sorted_map.insert(key_nfc, sort_json_value(v.clone()));
                }
            }
            Value::Object(sorted_map)
        }
        Value::Array(arr) => {
            let sorted_arr: Vec<Value> = arr.into_iter().map(sort_json_value).collect();
            Value::Array(sorted_arr)
        }
        other => other,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalBytes(pub Vec<u8>);

impl CanonicalBytes {
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).unwrap_or("<invalid utf-8>")
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_lexicographical_sorting() {
        let unordered = json!({
            "zebra": 1,
            "apple": "fruit",
            "metadata": {
                "tags": ["b", "a"],
                "version": 2,
                "author": "Alice"
            }
        });

        let canon = canonicalize(&unordered).unwrap();
        let expected = r#"{"apple":"fruit","metadata":{"author":"Alice","tags":["b","a"],"version":2},"zebra":1}"#;
        assert_eq!(canon.as_str(), expected);
    }
}
