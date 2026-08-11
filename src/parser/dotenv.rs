use crate::secret::{is_secret, mask_value};
use crate::types::{EnvEntry, MaskMode, ValueType};
use regex::Regex;

/// Classify the value type of an environment variable
pub fn classify_value_type(raw: &str, is_sec: bool) -> ValueType {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return ValueType::Empty;
    }
    if is_sec {
        return ValueType::Secret;
    }
    if trimmed == "true" || trimmed == "false" || trimmed == "TRUE" || trimmed == "FALSE" {
        return ValueType::Boolean;
    }
    if trimmed.parse::<i64>().is_ok() || trimmed.parse::<f64>().is_ok() {
        return ValueType::Number;
    }
    if ((trimmed.starts_with('{') && trimmed.ends_with('}'))
        || (trimmed.starts_with('[') && trimmed.ends_with(']')))
        && serde_json::from_str::<serde_json::Value>(trimmed).is_ok()
    {
        return ValueType::Json;
    }
    if trimmed.starts_with("http://")
        || trimmed.starts_with("https://")
        || trimmed.starts_with("postgres://")
        || trimmed.starts_with("mysql://")
        || trimmed.starts_with("mongodb://")
        || trimmed.starts_with("redis://")
    {
        return ValueType::Url;
    }
    ValueType::String
}

pub fn parse_dotenv(
    content: &str,
    mask_mode: MaskMode,
    custom_pattern: Option<&Regex>,
) -> Vec<EnvEntry> {
    let mut entries = Vec::new();
    let mut current_section: Option<String> = None;
    let mut pending_comment: Option<String> = None;

    let lines: Vec<&str> = content.lines().collect();
    let mut idx = 0;

    while idx < lines.len() {
        let line_num = idx + 1;
        let line = lines[idx].trim();

        if line.is_empty() {
            pending_comment = None;
            idx += 1;
            continue;
        }

        // Section header comment check (e.g. # --- DATABASE CONFIG --- or # [Database])
        if let Some(comment_raw) = line.strip_prefix('#') {
            let comment_text = comment_raw.trim();
            if comment_text.starts_with("---") || comment_text.starts_with('[') {
                let clean_section = comment_text
                    .trim_matches('-')
                    .trim_matches('[')
                    .trim_matches(']')
                    .trim();
                if !clean_section.is_empty() {
                    current_section = Some(clean_section.to_string());
                }
            } else {
                pending_comment = Some(comment_text.to_string());
            }
            idx += 1;
            continue;
        }

        // Remove optional leading 'export '
        let clean_line = line.strip_prefix("export ").unwrap_or(line).trim();

        // Key = Value splitting
        if let Some(eq_pos) = clean_line.find('=') {
            let key = clean_line[..eq_pos].trim().to_string();
            let rest = clean_line[eq_pos + 1..].trim();

            let raw_val;
            let mut inline_comment: Option<String> = None;

            if let Some(rest_content) = rest.strip_prefix('"') {
                // Double quoted value (may span multiline or contain escaped quotes)
                let mut val_buf = String::new();
                let mut escaped = false;

                let mut current_line_str = rest_content;
                let mut cur_line_idx = idx;

                'quote_loop: loop {
                    let chars: Vec<char> = current_line_str.chars().collect();
                    let mut i = 0;
                    while i < chars.len() {
                        let c = chars[i];
                        if escaped {
                            match c {
                                'n' => val_buf.push('\n'),
                                'r' => val_buf.push('\r'),
                                't' => val_buf.push('\t'),
                                _ => val_buf.push(c),
                            }
                            escaped = false;
                        } else if c == '\\' {
                            escaped = true;
                        } else if c == '"' {
                            // Check for trailing comment
                            let remaining: String = chars[i + 1..].iter().collect();
                            let rem_trim = remaining.trim();
                            if let Some(comment_part) = rem_trim.strip_prefix('#') {
                                inline_comment = Some(comment_part.trim().to_string());
                            }
                            break 'quote_loop;
                        } else {
                            val_buf.push(c);
                        }
                        i += 1;
                    }

                    if cur_line_idx + 1 >= lines.len() {
                        break;
                    }
                    cur_line_idx += 1;
                    val_buf.push('\n');
                    current_line_str = lines[cur_line_idx];
                }
                idx = cur_line_idx;
                raw_val = val_buf;
            } else if let Some(rest_content) = rest.strip_prefix('\'') {
                // Single quoted value
                let mut val_buf = String::new();
                let mut current_line_str = rest_content;
                let mut cur_line_idx = idx;

                'squote_loop: loop {
                    if let Some(q_pos) = current_line_str.find('\'') {
                        val_buf.push_str(&current_line_str[..q_pos]);
                        let remaining = &current_line_str[q_pos + 1..];
                        let rem_trim = remaining.trim();
                        if let Some(comment_part) = rem_trim.strip_prefix('#') {
                            inline_comment = Some(comment_part.trim().to_string());
                        }
                        break 'squote_loop;
                    } else {
                        val_buf.push_str(current_line_str);
                        if cur_line_idx + 1 >= lines.len() {
                            break;
                        }
                        cur_line_idx += 1;
                        val_buf.push('\n');
                        current_line_str = lines[cur_line_idx];
                    }
                }
                idx = cur_line_idx;
                raw_val = val_buf;
            } else {
                // Unquoted value with potential inline comment (#)
                if let Some(hash_pos) = rest.find(" #") {
                    raw_val = rest[..hash_pos].trim().to_string();
                    inline_comment =
                        Some(rest[hash_pos + 2..].trim().to_string());
                } else if let Some(comment_part) = rest.strip_prefix('#') {
                    raw_val = String::new();
                    inline_comment = Some(comment_part.trim().to_string());
                } else {
                    raw_val = rest.to_string();
                }
            }

            let comment = inline_comment.or(pending_comment.take());
            let sec_flag = is_secret(&key, &raw_val, custom_pattern);
            let val_type = classify_value_type(&raw_val, sec_flag);
            let masked = mask_value(&key, &raw_val, sec_flag, mask_mode);

            entries.push(EnvEntry {
                key,
                raw_value: raw_val,
                masked_value: masked,
                is_secret: sec_flag,
                value_type: val_type,
                comment,
                line_number: line_num,
                section: current_section.clone(),
            });
        } else {
            pending_comment = None;
        }

        idx += 1;
    }

    entries
}
