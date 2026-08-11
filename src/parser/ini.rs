use crate::parser::dotenv::classify_value_type;
use crate::secret::{is_secret, mask_value};
use crate::types::{EnvEntry, MaskMode};
use configparser::ini::Ini;
use regex::Regex;

pub fn parse_ini(
    content: &str,
    mask_mode: MaskMode,
    custom_pattern: Option<&Regex>,
) -> anyhow::Result<Vec<EnvEntry>> {
    let mut config = Ini::new();
    let map = config
        .read(content.to_string())
        .map_err(|e| anyhow::anyhow!("Failed to parse INI config: {}", e))?;

    let mut result = Vec::new();
    let mut line_num = 1;

    for (section, properties) in map {
        let sec_name = if section.is_empty() || section == "default" {
            None
        } else {
            Some(section.clone())
        };

        for (key, val_opt) in properties {
            let raw_val = val_opt.as_deref().unwrap_or("");
            let full_key = match &sec_name {
                Some(s) => format!("{}.{}", s, key),
                None => key.to_string(),
            };

            let sec_flag = is_secret(&full_key, raw_val, custom_pattern);
            let val_type = classify_value_type(raw_val, sec_flag);
            let masked = mask_value(&full_key, raw_val, sec_flag, mask_mode);

            result.push(EnvEntry {
                key: full_key,
                raw_value: raw_val.to_string(),
                masked_value: masked,
                is_secret: sec_flag,
                value_type: val_type,
                comment: None,
                line_number: line_num,
                section: sec_name.clone(),
            });
            line_num += 1;
        }
    }

    Ok(result)
}
