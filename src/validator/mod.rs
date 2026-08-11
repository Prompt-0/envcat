pub mod rules;

use crate::types::{EnvEntry, IssueSeverity};
use colored::Colorize;
pub use rules::{check_against_example, lint_entries};

pub fn run_validation(
    target_entries: &[EnvEntry],
    raw_content: &str,
    example_entries: Option<&[EnvEntry]>,
) -> bool {
    let mut issues = lint_entries(target_entries, raw_content);

    if let Some(ex_entries) = example_entries {
        issues.extend(check_against_example(target_entries, ex_entries));
    }

    let error_count = issues
        .iter()
        .filter(|i| i.severity == IssueSeverity::Error)
        .count();
    let warning_count = issues
        .iter()
        .filter(|i| i.severity == IssueSeverity::Warning)
        .count();
    let info_count = issues
        .iter()
        .filter(|i| i.severity == IssueSeverity::Info)
        .count();

    println!();
    println!(
        "╭─ [ {} ] VALIDATION & LINT REPORT ──────────────────────────",
        "ENVCAT".bold().cyan()
    );
    println!(
        "│ Errors: {}  │  Warnings: {}  │  Info: {}",
        error_count.to_string().bold().red(),
        warning_count.to_string().bold().yellow(),
        info_count.to_string().bold().blue()
    );
    println!("{}", "├─────────────────────────────────────────────────────────────".bright_black());

    if issues.is_empty() {
        println!(
            "{}",
            "│  ✨ All checks passed! No issues detected.".bold().green()
        );
    } else {
        for issue in &issues {
            let sev_badge = match issue.severity {
                IssueSeverity::Error => "✖ ERROR".bold().red(),
                IssueSeverity::Warning => "⚠ WARN ".bold().yellow(),
                IssueSeverity::Info => "ℹ INFO ".bold().blue(),
            };

            let loc_str = if let Some(line) = issue.line_number {
                format!(" [Line {}]", line).dimmed().to_string()
            } else {
                String::new()
            };

            println!(
                "│ {} {}{}",
                sev_badge, issue.message, loc_str
            );

            if let Some(sug) = &issue.suggestion {
                println!(
                    "│      💡 Suggestion: {}",
                    sug.italic().dimmed()
                );
            }
        }
    }

    println!("{}", "╰─────────────────────────────────────────────────────────────".bright_black());
    println!();

    error_count == 0
}
