use crate::parser::dotenv::classify_value_type;
use crate::secret::{is_secret, mask_value};
use crate::types::{EnvEntry, MaskMode};
use regex::Regex;
use serde_json::Value;

fn flatten_json(
    prefix: &str,
    val: &Value,
    entries: &mut Vec<(String, String)>,
) {
    match val {
        Value::Object(map) => {
            for (k, v) in map {
                let new_key = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{}.{}", prefix, k)
                };
                flatten_json(&new_key, v, entries);
            }
        }
        Value::Array(arr) => {
            for (idx, v) in arr.iter().enumerate() {
                let new_key = format!("{}[{}]", prefix, idx);
                flatten_json(&new_key, v, entries);
            }
        }
        Value::String(s) => {
            entries.push((prefix.to_string(), s.clone()));
        }
        Value::Number(n) => {
            entries.push((prefix.to_string(), n.to_string()));
        }
        Value::Bool(b) => {
            entries.push((prefix.to_string(), b.to_string()));
        }
        Value::Null => {
            entries.push((prefix.to_string(), String::new()));
        }
    }
}

pub fn parse_json(
    content: &str,
    mask_mode: MaskMode,
    custom_pattern: Option<&Regex>,
) -> anyhow::Result<Vec<EnvEntry>> {
    let root: Value = serde_json::from_str(content)?;
    let mut flat = Vec::new();
    flatten_json("", &root, &mut flat);

    let mut result = Vec::new();
    for (line_num, (key, raw_val)) in flat.into_iter().enumerate() {
        let sec_flag = is_secret(&key, &raw_val, custom_pattern);
        let val_type = classify_value_type(&raw_val, sec_flag);
        let masked = mask_value(&key, &raw_val, sec_flag, mask_mode);

        result.push(EnvEntry {
            key,
            raw_value: raw_val,
            masked_value: masked,
            is_secret: sec_flag,
            value_type: val_type,
            comment: None,
            line_number: line_num + 1,
            section: None,
        });
    }

    Ok(result)
}
