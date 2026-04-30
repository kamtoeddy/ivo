use serde_json::Value;
use std::collections::HashSet;

pub fn is_null_or_undefined(value: &Value) -> bool {
    value.is_null()
}

pub fn is_one_of(value: &Value, candidates: &[Value]) -> bool {
    candidates.iter().any(|c| c == value)
}

pub fn to_array<T: Clone>(value: Vec<T>) -> Vec<T> {
    value
}

pub fn get_unique(list: &[Value]) -> Vec<Value> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();

    for v in list {
        let s = serde_json::to_string(v).unwrap_or_default();
        if seen.insert(s.clone()) {
            // parse back to Value
            let parsed = serde_json::from_str(&s).unwrap_or(v.clone());
            out.push(parsed);
        }
    }

    out
}

pub fn get_unique_by(list: &[Value], key: &str) -> Vec<Value> {
    let mut map = std::collections::HashMap::new();

    for v in list {
        if let Some(k) = get_deep_value(v, key) {
            let s = serde_json::to_string(&k).unwrap_or_default();
            map.entry(s).or_insert_with(|| v.clone());
        }
    }

    map.into_values().collect()
}

fn get_deep_value<'a>(value: &'a Value, key: &str) -> Option<Value> {
    let mut cur = value;

    for part in key.split('.') {
        match cur {
            Value::Object(map) => cur = map.get(part)?,
            _ => return None,
        }
    }

    Some(cur.clone())
}

pub fn sort_strings(mut data: Vec<String>) -> Vec<String> {
    data.sort();
    data
}

pub fn sort_keys(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut keys: Vec<_> = map.keys().cloned().collect();
            keys.sort();
            let mut out = serde_json::Map::new();
            for k in keys {
                out.insert(k.clone(), sort_keys(&map[&k]));
            }
            Value::Object(out)
        }
        other => other.clone(),
    }
}
