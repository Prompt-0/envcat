use envcat::secret::{calculate_entropy, is_secret, mask_value, scrub_url_credentials};
use envcat::types::MaskMode;

#[test]
fn test_secret_key_name_heuristics() {
    assert!(is_secret("DATABASE_PASSWORD", "my_pass", None));
    assert!(is_secret("AWS_SECRET_ACCESS_KEY", "xyz", None));
    assert!(is_secret("STRIPE_API_KEY", "123", None));
    assert!(is_secret("AUTH_TOKEN", "token", None));
    assert!(is_secret("CLIENT_SECRET", "secret", None));
    assert!(is_secret("DB_DSN", "dsn", None));
    assert!(!is_secret("APP_NAME", "my-app", None));
    assert!(!is_secret("PORT", "8080", None));
}

#[test]
fn test_jwt_token_detection() {
    let jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
    assert!(is_secret("NORMAL_KEY", jwt, None));

    let masked = mask_value("NORMAL_KEY", jwt, true, MaskMode::Auto);
    assert!(masked.contains("[MASKED]"));
    assert!(masked.starts_with("eyJ"));
}

#[test]
fn test_database_url_scrubbing() {
    let raw_url = "postgres://admin_user:super_secret_db_pass@db.internal.company.com:5432/prod_db";
    let scrubbed = scrub_url_credentials(raw_url);
    assert_eq!(
        scrubbed,
        "postgres://admin_user:******@db.internal.company.com:5432/prod_db"
    );

    assert!(is_secret("DB_URI", raw_url, None));
    let masked = mask_value("DB_URI", raw_url, true, MaskMode::Auto);
    assert_eq!(
        masked,
        "postgres://admin_user:******@db.internal.company.com:5432/prod_db"
    );
}

#[test]
fn test_entropy_analysis() {
    let low_entropy = "aaaaaa";
    let high_entropy = "9f8a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b";

    assert!(calculate_entropy(low_entropy) < 1.0);
    assert!(calculate_entropy(high_entropy) > 3.5);
}
