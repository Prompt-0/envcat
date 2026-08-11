use envcat::diff::compare_environments;
use envcat::parser::parse_dotenv;
use envcat::types::{DiffStatus, MaskMode};

#[test]
fn test_environment_diff_engine() {
    let left_env = r#"
PORT=8080
DATABASE_URL=postgres://user:pass@localhost:5432/db1
FEATURE_FLAG=true
ONLY_IN_LEFT=hello
"#;

    let right_env = r#"
PORT=9090
DATABASE_URL=postgres://user:pass@localhost:5432/db1
FEATURE_FLAG=false
ONLY_IN_RIGHT=world
"#;

    let left_entries = parse_dotenv(left_env, MaskMode::Auto, None);
    let right_entries = parse_dotenv(right_env, MaskMode::Auto, None);

    let diffs = compare_environments(&left_entries, &right_entries);

    // PORT (ValueChanged)
    let port_diff = diffs.iter().find(|d| d.key == "PORT").unwrap();
    assert_eq!(port_diff.status, DiffStatus::ValueChanged);

    // DATABASE_URL (Matched)
    let db_diff = diffs.iter().find(|d| d.key == "DATABASE_URL").unwrap();
    assert_eq!(db_diff.status, DiffStatus::Matched);

    // ONLY_IN_LEFT (MissingInRight)
    let left_diff = diffs.iter().find(|d| d.key == "ONLY_IN_LEFT").unwrap();
    assert_eq!(left_diff.status, DiffStatus::MissingInRight);

    // ONLY_IN_RIGHT (MissingInLeft)
    let right_diff = diffs.iter().find(|d| d.key == "ONLY_IN_RIGHT").unwrap();
    assert_eq!(right_diff.status, DiffStatus::MissingInLeft);
}
