use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValueType {
    String,
    Number,
    Boolean,
    Url,
    Json,
    Secret,
    Empty,
}

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::String => write!(f, "string"),
            ValueType::Number => write!(f, "number"),
            ValueType::Boolean => write!(f, "bool"),
            ValueType::Url => write!(f, "url"),
            ValueType::Json => write!(f, "json"),
            ValueType::Secret => write!(f, "secret"),
            ValueType::Empty => write!(f, "empty"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigSource {
    Dotenv,
    Yaml,
    Toml,
    Json,
    Ini,
    System,
}

impl std::fmt::Display for ConfigSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigSource::Dotenv => write!(f, "Dotenv (.env)"),
            ConfigSource::Yaml => write!(f, "YAML"),
            ConfigSource::Toml => write!(f, "TOML"),
            ConfigSource::Json => write!(f, "JSON"),
            ConfigSource::Ini => write!(f, "INI"),
            ConfigSource::System => write!(f, "System Env"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaskMode {
    Auto,
    Reveal,
    MaskAll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Panel,
    Table,
    Dotenv,
    ShellExport,
    Json,
    Yaml,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvEntry {
    pub key: String,
    pub raw_value: String,
    pub masked_value: String,
    pub is_secret: bool,
    pub value_type: ValueType,
    pub comment: Option<String>,
    pub line_number: usize,
    pub section: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiffStatus {
    Matched,
    ValueChanged,
    TypeMismatch,
    MissingInLeft,
    MissingInRight,
}

#[derive(Debug, Clone)]
pub struct DiffEntry {
    pub key: String,
    pub status: DiffStatus,
    pub left_value: Option<String>,
    pub right_value: Option<String>,
    pub left_type: Option<ValueType>,
    pub right_type: Option<ValueType>,
    pub is_secret: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueSeverity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub severity: IssueSeverity,
    pub key: Option<String>,
    pub line_number: Option<usize>,
    pub message: String,
    pub suggestion: Option<String>,
}
