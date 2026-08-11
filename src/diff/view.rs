use crate::types::{DiffEntry, DiffStatus};
use colored::Colorize;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Color, ContentArrangement, Table};

pub fn render_diff_view(
    diffs: &[DiffEntry],
    left_label: &str,
    right_label: &str,
    only_changes: bool,
) {
    let mut matched_cnt = 0;
    let mut changed_cnt = 0;
    let mut missing_left_cnt = 0;
    let mut missing_right_cnt = 0;

    for d in diffs {
        match d.status {
            DiffStatus::Matched => matched_cnt += 1,
            DiffStatus::ValueChanged | DiffStatus::TypeMismatch => changed_cnt += 1,
            DiffStatus::MissingInLeft => missing_left_cnt += 1,
            DiffStatus::MissingInRight => missing_right_cnt += 1,
        }
    }

    println!();
    println!(
        "{}",
        format!(
            "╭─ [ {} ] DIFF: {} ↔ {} ──────────────────────────────────",
            "ENVCAT".bold().cyan(),
            left_label.bold().yellow(),
            right_label.bold().yellow()
        )
        .bright_black()
    );
    println!(
        "{}",
        format!(
            "│ Matched: {}  │  Changed: {}  │  Missing in Left: {}  │  Missing in Right: {}",
            matched_cnt.to_string().bold().green(),
            changed_cnt.to_string().bold().yellow(),
            missing_left_cnt.to_string().bold().red(),
            missing_right_cnt.to_string().bold().red()
        )
        .bright_black()
    );
    println!("{}", "├─────────────────────────────────────────────────────────────".bright_black());

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Status").fg(Color::Cyan),
            Cell::new("Key").fg(Color::Yellow),
            Cell::new(left_label).fg(Color::Magenta),
            Cell::new(right_label).fg(Color::Blue),
        ]);

    for d in diffs {
        if only_changes && d.status == DiffStatus::Matched {
            continue;
        }

        let status_cell = match d.status {
            DiffStatus::Matched => Cell::new("MATCH").fg(Color::Green),
            DiffStatus::ValueChanged => Cell::new("DIFF").fg(Color::Yellow),
            DiffStatus::TypeMismatch => Cell::new("TYPE MISMATCH").fg(Color::Magenta),
            DiffStatus::MissingInLeft => Cell::new("+ ADDED IN RIGHT").fg(Color::Blue),
            DiffStatus::MissingInRight => Cell::new("- MISSING IN RIGHT").fg(Color::Red),
        };

        let left_val = d.left_value.as_deref().unwrap_or("<absent>");
        let right_val = d.right_value.as_deref().unwrap_or("<absent>");

        let left_cell = if d.status == DiffStatus::MissingInLeft {
            Cell::new(left_val).fg(Color::DarkGrey)
        } else if d.is_secret {
            Cell::new(left_val).fg(Color::Red)
        } else {
            Cell::new(left_val)
        };

        let right_cell = if d.status == DiffStatus::MissingInRight {
            Cell::new(right_val).fg(Color::DarkGrey)
        } else if d.is_secret {
            Cell::new(right_val).fg(Color::Red)
        } else {
            Cell::new(right_val)
        };

        table.add_row(vec![
            status_cell,
            Cell::new(&d.key).fg(Color::Cyan),
            left_cell,
            right_cell,
        ]);
    }

    println!("{table}");
    println!();
}
