use crate::types::EnvEntry;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Color, ContentArrangement, Table};

pub fn render_table_view(entries: &[EnvEntry], show_raw: bool) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("#").fg(Color::Cyan),
            Cell::new("Key").fg(Color::Yellow),
            Cell::new("Type").fg(Color::Green),
            Cell::new("Secret").fg(Color::Red),
            Cell::new("Value").fg(Color::White),
            Cell::new("Comment").fg(Color::DarkGrey),
        ]);

    for entry in entries {
        let val = if show_raw {
            &entry.raw_value
        } else {
            &entry.masked_value
        };

        let secret_str = if entry.is_secret { "YES (🔒)" } else { "NO" };
        let secret_color = if entry.is_secret {
            Color::Red
        } else {
            Color::DarkGrey
        };

        let val_cell = if entry.is_secret && !show_raw {
            Cell::new(val).fg(Color::Red)
        } else {
            Cell::new(val)
        };

        table.add_row(vec![
            Cell::new(entry.line_number),
            Cell::new(&entry.key).fg(Color::Cyan),
            Cell::new(entry.value_type.to_string()),
            Cell::new(secret_str).fg(secret_color),
            val_cell,
            Cell::new(entry.comment.as_deref().unwrap_or("")),
        ]);
    }

    println!("{table}");
}
