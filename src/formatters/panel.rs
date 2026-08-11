use crate::types::{ConfigSource, EnvEntry, ValueType};
use colored::Colorize;

pub fn render_panel_view(
    entries: &[EnvEntry],
    source: ConfigSource,
    filename: Option<&str>,
    show_raw: bool,
) {
    let source_title = filename.unwrap_or(match source {
        ConfigSource::System => "System Environment",
        _ => "Configuration",
    });

    let total = entries.len();
    let secret_count = entries.iter().filter(|e| e.is_secret).count();

    println!();
    println!(
        "╭─ [ {} ] {} ──────────────────────────────────────────",
        "ENVCAT".bold().cyan(),
        source_title.bold().yellow()
    );
    println!(
        "│ Source: {}  │  Total Variables: {}  │  Secrets Masked: {}",
        source.to_string().magenta(),
        total.to_string().bold().green(),
        secret_count.to_string().bold().red()
    );
    println!("{}", "├─────────────────────────────────────────────────────────────".bright_black());

    let mut current_section: Option<String> = None;

    for entry in entries {
        if entry.section != current_section {
            current_section = entry.section.clone();
            if let Some(sec) = &current_section {
                println!(
                    "│  📂 SECTION: {}",
                    sec.bold().blue()
                );
            }
        }

        let type_badge = match entry.value_type {
            ValueType::Secret => "[secret]".bold().red(),
            ValueType::Url => "[url]".bold().magenta(),
            ValueType::Json => "[json]".bold().cyan(),
            ValueType::Boolean => "[bool]".bold().yellow(),
            ValueType::Number => "[number]".bold().green(),
            ValueType::String => "[string]".dimmed(),
            ValueType::Empty => "[empty]".italic().dimmed(),
        };

        let sec_indicator = if entry.is_secret {
            "🔒".red()
        } else {
            "  ".normal()
        };

        let val_display = if show_raw {
            &entry.raw_value
        } else {
            &entry.masked_value
        };

        let colored_val = if entry.is_secret && !show_raw {
            val_display.red()
        } else if entry.value_type == ValueType::Empty {
            "<empty>".italic().dimmed()
        } else if entry.value_type == ValueType::Boolean {
            val_display.yellow()
        } else if entry.value_type == ValueType::Number {
            val_display.green()
        } else {
            val_display.white()
        };

        println!(
            "│ {} {:<28} {:<10} = {}",
            sec_indicator,
            entry.key.bold().bright_cyan(),
            type_badge,
            colored_val
        );

        if let Some(cmt) = &entry.comment {
            println!(
                "│      💬 {}",
                cmt.italic().dimmed()
            );
        }
    }

    println!("{}", "╰─────────────────────────────────────────────────────────────".bright_black());
    println!();
}
