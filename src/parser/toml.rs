use crate::parser::dotenv::classify_value_type;
use crate::secret::{is_secret, mask_value};
use crate::types::{EnvEntry, MaskMode};
use regex::Regex;
use toml::Value;

fn flatten_toml(
    prefix: &str,
    val: &Value,
    entries: &mut Vec<(String, String)>,
) {
    match val {
        Value::Table(table) => {
            for (k, v) in table {
                let new_key = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{}.{}", prefix, k)
                };
                flatten_toml(&new_key, v, entries);
            }
        }
        Value::Array(arr) => {
            for (idx, v) in arr.iter().enumerate() {
                let new_key = format!("{}[{}]", prefix, idx);
                flatten_toml(&new_key, v, entries);
            }
        }
        Value::String(s) => {
            entries.push((prefix.to_string(), s.clone()));
        }
        Value::Integer(i) => {
            entries.push((prefix.to_string(), i.to_string()));
        }
        Value::Float(f) => {
            entries.push((prefix.to_string(), f.to_string()));
        }
        Value::Boolean(b) => {
            entries.push((prefix.to_string(), b.to_string()));
        }
        Value::Datetime(dt) => {
            entries.push((prefix.to_string(), dt.to_string()));
        }
    }
}

pub fn parse_toml(
    content: &str,
    mask_mode: MaskMode,
    custom_pattern: Option<&Regex>,
) -> anyhow::Result<Vec<EnvEntry>> {
    let root: Value = toml::from_str(content)?;
    let mut flat = Vec::new();
    flatten_toml("", &root, &mut flat);

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
