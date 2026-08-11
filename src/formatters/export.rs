use crate::types::EnvEntry;

pub fn render_dotenv_export(entries: &[EnvEntry], show_raw: bool) {
    let mut current_section: Option<String> = None;

    for entry in entries {
        if entry.section != current_section {
            current_section = entry.section.clone();
            if let Some(sec) = &current_section {
                println!("\n# --- {} ---", sec);
            }
        }

        if let Some(cmt) = &entry.comment {
            println!("# {}", cmt);
        }

        let val = if show_raw {
            &entry.raw_value
        } else {
            &entry.masked_value
        };

        if val.contains(' ') || val.contains('\n') || val.contains('#') {
            let escaped = val.replace('"', "\\\"");
            println!("{}=\"{}\"", entry.key, escaped);
        } else {
            println!("{}={}", entry.key, val);
        }
    }
}

pub fn render_shell_export(entries: &[EnvEntry], show_raw: bool) {
    for entry in entries {
        let val = if show_raw {
            &entry.raw_value
        } else {
            &entry.masked_value
        };
        let escaped = val.replace('"', "\\\"");
        println!("export {}=\"{}\"", entry.key, escaped);
    }
}

pub fn render_json_export(entries: &[EnvEntry], show_raw: bool) -> anyhow::Result<()> {
    let mut map = std::collections::BTreeMap::new();
    for entry in entries {
        let val = if show_raw {
            &entry.raw_value
        } else {
            &entry.masked_value
        };
        map.insert(entry.key.clone(), val.clone());
    }
    let json_str = serde_json::to_string_pretty(&map)?;
    println!("{}", json_str);
    Ok(())
}

pub fn render_yaml_export(entries: &[EnvEntry], show_raw: bool) -> anyhow::Result<()> {
    let mut map = std::collections::BTreeMap::new();
    for entry in entries {
        let val = if show_raw {
            &entry.raw_value
        } else {
            &entry.masked_value
        };
        map.insert(entry.key.clone(), val.clone());
    }
    let yaml_str = serde_yaml::to_string(&map)?;
    print!("{}", yaml_str);
    Ok(())
}

pub fn generate_example_file(entries: &[EnvEntry]) {
    println!("# Auto-generated .env.example by envcat\n");
    let mut current_section: Option<String> = None;

    for entry in entries {
        if entry.section != current_section {
            current_section = entry.section.clone();
            if let Some(sec) = &current_section {
                println!("\n# --- {} ---", sec);
            }
        }

        if let Some(cmt) = &entry.comment {
            println!("# {}", cmt);
        }

        let placeholder = match entry.value_type {
            crate::types::ValueType::Secret => "your_secret_here",
            crate::types::ValueType::Url => "https://api.example.com",
            crate::types::ValueType::Boolean => "true",
            crate::types::ValueType::Number => "8080",
            crate::types::ValueType::Json => "{}",
            _ => "your_value_here",
        };

        println!("{}={}", entry.key, placeholder);
    }
}
