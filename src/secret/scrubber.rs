use crate::secret::patterns::get_db_uri_regex;
use crate::types::MaskMode;

/// Scrub passwords from connection URLs (e.g. postgres://user:password@host:port/db)
pub fn scrub_url_credentials(url: &str) -> String {
    let db_re = get_db_uri_regex();
    if let Some(caps) = db_re.captures(url) {
        let scheme = &caps[1];
        let user = &caps[2];
        let rest = &caps[4];
        format!("{}://{}:******@{}", scheme, user, rest)
    } else {
        url.to_string()
    }
}

/// Create a masked representation of a secret value
pub fn mask_value(_key: &str, raw: &str, is_sec: bool, mode: MaskMode) -> String {
    if raw.is_empty() {
        return String::new();
    }

    match mode {
        MaskMode::Reveal => raw.to_string(),
        MaskMode::MaskAll => "******".to_string(),
        MaskMode::Auto => {
            if is_sec {
                // If it's a URL with credentials, scrub the credentials
                if get_db_uri_regex().is_match(raw) {
                    scrub_url_credentials(raw)
                } else if raw.starts_with("eyJ") && raw.len() > 20 {
                    // JWT prefix reveal
                    format!("{}...[MASKED]", &raw[..8])
                } else if raw.starts_with("ghp_") && raw.len() > 10 {
                    "ghp_...[MASKED]".to_string()
                } else if raw.starts_with("AKIA") && raw.len() > 8 {
                    "AKIA...[MASKED]".to_string()
                } else if raw.len() <= 6 {
                    "******".to_string()
                } else {
                    format!("{}******{}", &raw[..2], &raw[raw.len() - 2..])
                }
            } else {
                raw.to_string()
            }
        }
    }
}
