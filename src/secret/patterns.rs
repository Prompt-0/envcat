use regex::Regex;
use std::sync::OnceLock;

static SECRET_KEY_REGEX: OnceLock<Regex> = OnceLock::new();
static JWT_REGEX: OnceLock<Regex> = OnceLock::new();
static PEM_REGEX: OnceLock<Regex> = OnceLock::new();
static AWS_KEY_REGEX: OnceLock<Regex> = OnceLock::new();
static GITHUB_TOKEN_REGEX: OnceLock<Regex> = OnceLock::new();
static SLACK_TOKEN_REGEX: OnceLock<Regex> = OnceLock::new();
static DB_URI_REGEX: OnceLock<Regex> = OnceLock::new();

pub fn get_secret_key_regex() -> &'static Regex {
    SECRET_KEY_REGEX.get_or_init(|| {
        Regex::new(r"(?i).*(secret|password|passwd|pass|token|auth|credential|private|apikey|api_key|dsn|connstr|cert|bearer|access_key|access_token|client_secret|db_pass|aws_access|sig|signature|hash|jwt|private_key|ssl_key).*").unwrap()
    })
}

pub fn get_jwt_regex() -> &'static Regex {
    JWT_REGEX.get_or_init(|| {
        Regex::new(r"^eyJ[A-Za-z0-9_-]+\.eyJ[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+$").unwrap()
    })
}

pub fn get_pem_regex() -> &'static Regex {
    PEM_REGEX.get_or_init(|| {
        Regex::new(r"-----BEGIN (RSA |EC |OPENSSH |DSA )?PRIVATE KEY-----").unwrap()
    })
}

pub fn get_aws_key_regex() -> &'static Regex {
    AWS_KEY_REGEX.get_or_init(|| {
        Regex::new(r"^(AKIA|ASIA)[0-9A-Z]{16}$").unwrap()
    })
}

pub fn get_github_token_regex() -> &'static Regex {
    GITHUB_TOKEN_REGEX.get_or_init(|| {
        Regex::new(r"^(ghp|gho|ghu|ghs|ghr)_[a-zA-Z0-9]{36,255}$").unwrap()
    })
}

pub fn get_slack_token_regex() -> &'static Regex {
    SLACK_TOKEN_REGEX.get_or_init(|| {
        Regex::new(r"^xox[baprs]-[0-9a-zA-Z]{10,48}").unwrap()
    })
}

pub fn get_db_uri_regex() -> &'static Regex {
    DB_URI_REGEX.get_or_init(|| {
        Regex::new(r"^(?i)([a-z0-9+.-]+)://([^:@\s]+):([^@\s]+)@(.*)$").unwrap()
    })
}

/// Calculate Shannon entropy for a string
pub fn calculate_entropy(s: &str) -> f64 {
    if s.is_empty() {
        return 0.0;
    }
    let mut frequency = std::collections::HashMap::new();
    for ch in s.chars() {
        *frequency.entry(ch).or_insert(0) += 1;
    }
    let len = s.len() as f64;
    let mut entropy = 0.0;
    for &count in frequency.values() {
        let p = count as f64 / len;
        entropy -= p * p.log2();
    }
    entropy
}

/// Check if key or value matches secret indicators
pub fn is_secret(key: &str, value: &str, custom_pattern: Option<&Regex>) -> bool {
    if value.trim().is_empty() {
        return false;
    }

    // Custom pattern match
    if let Some(re) = custom_pattern {
        if re.is_match(key) || re.is_match(value) {
            return true;
        }
    }

    // Key name check
    if get_secret_key_regex().is_match(key) {
        return true;
    }

    // Specific value patterns
    if get_jwt_regex().is_match(value)
        || get_pem_regex().is_match(value)
        || get_aws_key_regex().is_match(value)
        || get_github_token_regex().is_match(value)
        || get_slack_token_regex().is_match(value)
        || get_db_uri_regex().is_match(value)
    {
        return true;
    }

    // High entropy check for random hashes / tokens (len >= 20 and entropy >= 4.5)
    if value.len() >= 20 && calculate_entropy(value) > 4.5 {
        // Exclude JSON or URLs that aren't credentials
        if !value.starts_with('{') && !value.starts_with('[') && !value.starts_with("http") {
            return true;
        }
    }

    false
}
