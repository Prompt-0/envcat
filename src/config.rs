use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    name = "envcat",
    author = "Antigravity AI <agy@deepmind.com>",
    version = "0.1.0",
    about = "Modern Terminal Environment & Config File Inspector, Secret Masker, Diff Engine, Validator, and TUI",
    long_about = "envcat inspects, masks secrets, validates, diffs, and interactively explores environment variables and configuration files (.env, YAML, TOML, JSON, INI, process environment)."
)]
pub struct Cli {
    /// Target file(s) to inspect or compare
    #[arg(value_name = "FILES")]
    pub files: Vec<String>,

    /// Launch interactive TUI mode
    #[arg(short = 'i', long = "interactive")]
    pub interactive: bool,

    /// Reveal raw secret values (disable secret masking)
    #[arg(short = 'r', long = "reveal")]
    pub reveal: bool,

    /// Mask all values regardless of secret detection
    #[arg(long = "mask-all")]
    pub mask_all: bool,

    /// Display formatted ASCII/ANSI table view
    #[arg(short = 't', long = "table")]
    pub table: bool,

    /// Filter entries matching substring or regex pattern
    #[arg(short = 'f', long = "filter", value_name = "PATTERN")]
    pub filter: Option<String>,

    /// Filter entries starting with specified key prefix (e.g. APP_)
    #[arg(long = "prefix", value_name = "PREFIX")]
    pub prefix: Option<String>,

    /// Read active system process environment variables
    #[arg(short = 's', long = "system")]
    pub system: bool,

    /// Custom regex pattern for secret detection
    #[arg(long = "custom-secret", value_name = "REGEX")]
    pub custom_secret: Option<String>,

    /// Output format export mode
    #[arg(long = "export", value_enum)]
    pub export: Option<ExportFormatArg>,

    /// Subcommands for specific workflows (diff, check, generate-example, completion)
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum ExportFormatArg {
    Dotenv,
    Shell,
    Json,
    Yaml,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Compare two configuration files or system environment
    Diff {
        /// Primary file (left side of diff)
        file1: Option<String>,
        /// Secondary file (right side of diff)
        file2: Option<String>,
        /// Compare system environment as right side
        #[arg(short = 's', long = "system")]
        system: bool,
        /// Show only changed / missing variables
        #[arg(long = "changes-only")]
        changes_only: bool,
    },
    /// Validate config file against schema / example file
    Check {
        /// Target config file to validate
        file: Option<String>,
        /// Reference example file (e.g. .env.example)
        #[arg(short = 'e', long = "example")]
        example: Option<String>,
    },
    /// Auto-generate a clean .env.example with scrubbed secrets and type hints
    GenerateExample {
        /// Source config file
        file: Option<String>,
    },
    /// Generate shell completion script
    Completion {
        /// Target shell (bash, zsh, fish, powershell)
        #[arg(value_name = "SHELL")]
        shell: String,
    },
}
