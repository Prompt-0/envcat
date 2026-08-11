pub mod config;
pub mod diff;
pub mod formatters;
pub mod parser;
pub mod secret;
pub mod tui;
pub mod types;
pub mod validator;

use clap::{CommandFactory, Parser};
use config::{Cli, Commands, ExportFormatArg};
use diff::{compare_environments, render_diff_view};
use formatters::{
    generate_example_file, render_dotenv_export, render_json_export, render_panel_view,
    render_shell_export, render_table_view, render_yaml_export,
};
use parser::load_config;
use regex::Regex;
use types::MaskMode;
use validator::run_validation;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mask_mode = if cli.reveal {
        MaskMode::Reveal
    } else if cli.mask_all {
        MaskMode::MaskAll
    } else {
        MaskMode::Auto
    };

    let custom_pattern = if let Some(pat) = &cli.custom_secret {
        Some(Regex::new(pat)?)
    } else {
        None
    };

    // Subcommands dispatch
    if let Some(cmd) = &cli.command {
        match cmd {
            Commands::Diff {
                file1,
                file2,
                system,
                changes_only,
            } => {
                let left_path = file1.as_deref();
                let right_path = file2.as_deref();

                let (left_entries, left_source) =
                    load_config(left_path, left_path.is_none(), mask_mode, custom_pattern.as_ref())?;
                let (right_entries, right_source) = load_config(
                    right_path,
                    *system || right_path.is_none(),
                    mask_mode,
                    custom_pattern.as_ref(),
                )?;

                let left_source_str = left_source.to_string();
                let right_source_str = right_source.to_string();

                let left_label = left_path.unwrap_or(&left_source_str);
                let right_label = right_path.unwrap_or(&right_source_str);

                let diffs = compare_environments(&left_entries, &right_entries);
                render_diff_view(&diffs, left_label, right_label, *changes_only);
                return Ok(());
            }
            Commands::Check { file, example } => {
                let target_path = file.as_deref().or(cli.files.first().map(|s| s.as_str()));
                let (target_entries, _) =
                    load_config(target_path, target_path.is_none(), mask_mode, custom_pattern.as_ref())?;

                let raw_content = if let Some(p) = target_path {
                    std::fs::read_to_string(p).unwrap_or_default()
                } else {
                    String::new()
                };

                let example_entries = if let Some(ex_path) = example {
                    let (ex, _) =
                        load_config(Some(ex_path), false, mask_mode, custom_pattern.as_ref())?;
                    Some(ex)
                } else {
                    None
                };

                let clean = run_validation(&target_entries, &raw_content, example_entries.as_deref());
                if !clean {
                    std::process::exit(1);
                }
                return Ok(());
            }
            Commands::GenerateExample { file } => {
                let target_path = file.as_deref().or(cli.files.first().map(|s| s.as_str()));
                let (entries, _) =
                    load_config(target_path, target_path.is_none(), mask_mode, custom_pattern.as_ref())?;
                generate_example_file(&entries);
                return Ok(());
            }
            Commands::Completion { shell } => {
                let mut cmd = Cli::command();
                let shell_enum = match shell.to_lowercase().as_str() {
                    "bash" => clap_complete::Shell::Bash,
                    "zsh" => clap_complete::Shell::Zsh,
                    "fish" => clap_complete::Shell::Fish,
                    "powershell" => clap_complete::Shell::PowerShell,
                    _ => clap_complete::Shell::Bash,
                };
                clap_complete::generate(
                    shell_enum,
                    &mut cmd,
                    "envcat",
                    &mut std::io::stdout(),
                );
                return Ok(());
            }
        }
    }

    // Default inspection workflow
    let target_path = cli.files.first().map(|s| s.as_str());
    let (mut entries, source) =
        load_config(target_path, cli.system, mask_mode, custom_pattern.as_ref())?;

    // Filtering by search pattern
    if let Some(filter_str) = &cli.filter {
        let pat = filter_str.to_lowercase();
        entries.retain(|e| {
            e.key.to_lowercase().contains(&pat)
                || e.raw_value.to_lowercase().contains(&pat)
                || e.comment
                    .as_ref()
                    .map(|c| c.to_lowercase().contains(&pat))
                    .unwrap_or(false)
        });
    }

    // Filtering by prefix
    if let Some(prefix_str) = &cli.prefix {
        entries.retain(|e| e.key.starts_with(prefix_str));
    }

    // Interactive TUI Mode
    if cli.interactive {
        tui::run_tui(entries)?;
        return Ok(());
    }

    // Export formats
    if let Some(exp) = cli.export {
        match exp {
            ExportFormatArg::Dotenv => render_dotenv_export(&entries, cli.reveal),
            ExportFormatArg::Shell => render_shell_export(&entries, cli.reveal),
            ExportFormatArg::Json => render_json_export(&entries, cli.reveal)?,
            ExportFormatArg::Yaml => render_yaml_export(&entries, cli.reveal)?,
        }
        return Ok(());
    }

    // Table or Panel View
    if cli.table {
        render_table_view(&entries, cli.reveal);
    } else {
        render_panel_view(&entries, source, target_path, cli.reveal);
    }

    Ok(())
}
