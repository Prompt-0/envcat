use crate::types::{EnvEntry, IssueSeverity, ValidationIssue};
use std::collections::{HashMap, HashSet};

pub fn check_against_example(
    target_entries: &[EnvEntry],
    example_entries: &[EnvEntry],
) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    let target_keys: HashMap<&str, &EnvEntry> =
        target_entries.iter().map(|e| (e.key.as_str(), e)).collect();

    for ex in example_entries {
        if !target_keys.contains_key(ex.key.as_str()) {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Error,
                key: Some(ex.key.clone()),
                line_number: None,
                message: format!("Missing required environment key: '{}'", ex.key),
                suggestion: Some(format!(
                    "Add '{}={}' to your config file.",
                    ex.key, ex.raw_value
                )),
            });
        } else {
            let target_entry = target_keys[ex.key.as_str()];
            if target_entry.raw_value.is_empty() && !ex.raw_value.is_empty() {
                issues.push(ValidationIssue {
                    severity: IssueSeverity::Warning,
                    key: Some(ex.key.clone()),
                    line_number: Some(target_entry.line_number),
                    message: format!("Key '{}' is empty in config file", ex.key),
                    suggestion: Some("Provide a non-empty value if required by app.".to_string()),
                });
            }
        }
    }

    issues
}

pub fn lint_entries(entries: &[EnvEntry], raw_content: &str) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    let mut seen_keys = HashSet::new();

    for entry in entries {
        // Check for duplicate keys
        if seen_keys.contains(&entry.key) {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Warning,
                key: Some(entry.key.clone()),
                line_number: Some(entry.line_number),
                message: format!("Duplicate key declaration found: '{}'", entry.key),
                suggestion: Some("Remove duplicate declaration or consolidate values.".to_string()),
            });
        } else {
            seen_keys.insert(entry.key.clone());
        }

        // Check for empty values
        if entry.raw_value.is_empty() {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Info,
                key: Some(entry.key.clone()),
                line_number: Some(entry.line_number),
                message: format!("Key '{}' has an empty value", entry.key),
                suggestion: None,
            });
        }
    }

    // Check raw lines for syntax issues like trailing whitespace or missing =
    for (line_idx, line) in raw_content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let clean_line = trimmed.strip_prefix("export ").unwrap_or(trimmed).trim();

        if !clean_line.contains('=') {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Error,
                key: None,
                line_number: Some(line_idx + 1),
                message: "Syntax Error: Line missing '=' assignment".to_string(),
                suggestion: Some("Format line as KEY=value".to_string()),
            });
        }
    }

    issues
}
