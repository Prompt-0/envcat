use crate::parser::dotenv::classify_value_type;
use crate::secret::{is_secret, mask_value};
use crate::types::{EnvEntry, MaskMode};
use regex::Regex;

pub fn read_system_env(
    mask_mode: MaskMode,
    custom_pattern: Option<&Regex>,
) -> Vec<EnvEntry> {
    let mut vars: Vec<(String, String)> = std::env::vars().collect();
    vars.sort_by(|a, b| a.0.cmp(&b.0));

    let mut result = Vec::new();
    for (line_num, (key, raw_val)) in vars.into_iter().enumerate() {
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
            section: Some("System Process Env".to_string()),
        });
    }

    result
}
