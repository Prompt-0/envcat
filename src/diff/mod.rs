use crate::types::{DiffEntry, DiffStatus, EnvEntry};
use std::collections::BTreeMap;

pub mod view;
pub use view::render_diff_view;

pub fn compare_environments(left: &[EnvEntry], right: &[EnvEntry]) -> Vec<DiffEntry> {
    let mut left_map: BTreeMap<&str, &EnvEntry> = BTreeMap::new();
    let mut right_map: BTreeMap<&str, &EnvEntry> = BTreeMap::new();

    for entry in left {
        left_map.insert(&entry.key, entry);
    }
    for entry in right {
        right_map.insert(&entry.key, entry);
    }

    let mut all_keys: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    for k in left_map.keys() {
        all_keys.insert(k);
    }
    for k in right_map.keys() {
        all_keys.insert(k);
    }

    let mut diffs = Vec::new();

    for key in all_keys {
        let left_entry = left_map.get(key);
        let right_entry = right_map.get(key);

        match (left_entry, right_entry) {
            (Some(l), Some(r)) => {
                let is_sec = l.is_secret || r.is_secret;
                if l.raw_value == r.raw_value {
                    diffs.push(DiffEntry {
                        key: key.to_string(),
                        status: DiffStatus::Matched,
                        left_value: Some(l.masked_value.clone()),
                        right_value: Some(r.masked_value.clone()),
                        left_type: Some(l.value_type),
                        right_type: Some(r.value_type),
                        is_secret: is_sec,
                    });
                } else if l.value_type != r.value_type {
                    diffs.push(DiffEntry {
                        key: key.to_string(),
                        status: DiffStatus::TypeMismatch,
                        left_value: Some(l.masked_value.clone()),
                        right_value: Some(r.masked_value.clone()),
                        left_type: Some(l.value_type),
                        right_type: Some(r.value_type),
                        is_secret: is_sec,
                    });
                } else {
                    diffs.push(DiffEntry {
                        key: key.to_string(),
                        status: DiffStatus::ValueChanged,
                        left_value: Some(l.masked_value.clone()),
                        right_value: Some(r.masked_value.clone()),
                        left_type: Some(l.value_type),
                        right_type: Some(r.value_type),
                        is_secret: is_sec,
                    });
                }
            }
            (Some(l), None) => {
                diffs.push(DiffEntry {
                    key: key.to_string(),
                    status: DiffStatus::MissingInRight,
                    left_value: Some(l.masked_value.clone()),
                    right_value: None,
                    left_type: Some(l.value_type),
                    right_type: None,
                    is_secret: l.is_secret,
                });
            }
            (None, Some(r)) => {
                diffs.push(DiffEntry {
                    key: key.to_string(),
                    status: DiffStatus::MissingInLeft,
                    left_value: None,
                    right_value: Some(r.masked_value.clone()),
                    left_type: None,
                    right_type: Some(r.value_type),
                    is_secret: r.is_secret,
                });
            }
            (None, None) => {}
        }
    }

    diffs
}
