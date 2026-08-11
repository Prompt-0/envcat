use envcat::parser::parse_dotenv;
use envcat::types::{IssueSeverity, MaskMode};
use envcat::validator::{check_against_example, lint_entries};

#[test]
fn test_validator_missing_required_keys() {
    let example_env = r#"
PORT=8080
API_KEY=your_key_here
DB_HOST=localhost
"#;

    let target_env = r#"
PORT=3000
"#;

    let example_entries = parse_dotenv(example_env, MaskMode::Auto, None);
    let target_entries = parse_dotenv(target_env, MaskMode::Auto, None);

    let issues = check_against_example(&target_entries, &example_entries);

    assert_eq!(issues.len(), 2);
    assert!(issues.iter().all(|i| i.severity == IssueSeverity::Error));
}

#[test]
fn test_lint_duplicate_keys_and_syntax() {
    let target_env = r#"
PORT=8080
PORT=9090
EMPTY_KEY=
INVALID_LINE_WITHOUT_EQUALS
"#;

    let target_entries = parse_dotenv(target_env, MaskMode::Auto, None);
    let issues = lint_entries(&target_entries, target_env);

    assert!(issues
        .iter()
        .any(|i| i.message.contains("Duplicate key declaration")));
    assert!(issues
        .iter()
        .any(|i| i.message.contains("Syntax Error")));
}
