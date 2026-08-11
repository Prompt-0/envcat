use envcat::parser::{parse_dotenv, parse_ini, parse_json, parse_toml, parse_yaml};
use envcat::types::{MaskMode, ValueType};

#[test]
fn test_dotenv_parser_quotes_and_comments() {
    let dotenv_sample = r#"
# --- APP CONFIG ---
APP_NAME="My Super App"
APP_ENV=production # production server
PORT=8080
export IS_DEBUG=false
MULTILINE_VAL="Hello\nWorld"
SECRET_KEY='super_secret_token_123'
EMPTY_VAR=
"#;

    let entries = parse_dotenv(dotenv_sample, MaskMode::Auto, None);

    assert_eq!(entries.len(), 7);

    // APP_NAME
    assert_eq!(entries[0].key, "APP_NAME");
    assert_eq!(entries[0].raw_value, "My Super App");
    assert_eq!(entries[0].section, Some("APP CONFIG".to_string()));

    // APP_ENV with inline comment
    assert_eq!(entries[1].key, "APP_ENV");
    assert_eq!(entries[1].raw_value, "production");
    assert_eq!(entries[1].comment, Some("production server".to_string()));

    // PORT
    assert_eq!(entries[2].key, "PORT");
    assert_eq!(entries[2].raw_value, "8080");
    assert_eq!(entries[2].value_type, ValueType::Number);

    // IS_DEBUG with export keyword
    assert_eq!(entries[3].key, "IS_DEBUG");
    assert_eq!(entries[3].raw_value, "false");
    assert_eq!(entries[3].value_type, ValueType::Boolean);

    // SECRET_KEY
    assert_eq!(entries[5].key, "SECRET_KEY");
    assert!(entries[5].is_secret);
    assert_ne!(entries[5].masked_value, "super_secret_token_123");

    // EMPTY_VAR
    assert_eq!(entries[6].key, "EMPTY_VAR");
    assert_eq!(entries[6].value_type, ValueType::Empty);
}

#[test]
fn test_json_parser_flattening() {
    let json_sample = r#"{
        "server": {
            "host": "127.0.0.1",
            "port": 9000,
            "ssl": true
        },
        "db": {
            "password": "mypassword123"
        }
    }"#;

    let entries = parse_json(json_sample, MaskMode::Auto, None).unwrap();
    assert_eq!(entries.len(), 4);

    let host_entry = entries.iter().find(|e| e.key == "server.host").unwrap();
    assert_eq!(host_entry.raw_value, "127.0.0.1");

    let pass_entry = entries.iter().find(|e| e.key == "db.password").unwrap();
    assert!(pass_entry.is_secret);
}

#[test]
fn test_yaml_parser_flattening() {
    let yaml_sample = r#"
server:
  host: "0.0.0.0"
  port: 3000
database:
  api_key: "secret_yaml_key_999"
"#;

    let entries = parse_yaml(yaml_sample, MaskMode::Auto, None).unwrap();
    assert_eq!(entries.len(), 3);

    let key_entry = entries.iter().find(|e| e.key == "database.api_key").unwrap();
    assert!(key_entry.is_secret);
}

#[test]
fn test_toml_parser_flattening() {
    let toml_sample = r#"
[server]
port = 5000
enabled = true

[auth]
jwt_secret = "toml_jwt_secret_token_xyz"
"#;

    let entries = parse_toml(toml_sample, MaskMode::Auto, None).unwrap();
    assert_eq!(entries.len(), 3);

    let jwt_entry = entries.iter().find(|e| e.key == "auth.jwt_secret").unwrap();
    assert!(jwt_entry.is_secret);
}

#[test]
fn test_ini_parser_sections() {
    let ini_sample = r#"
[database]
user = admin
pass = secretpass

[web]
port = 80
"#;

    let entries = parse_ini(ini_sample, MaskMode::Auto, None).unwrap();
    assert_eq!(entries.len(), 3);

    let pass_entry = entries.iter().find(|e| e.key == "database.pass").unwrap();
    assert!(pass_entry.is_secret);
}
