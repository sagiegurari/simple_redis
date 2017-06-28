use super::*;

#[test]
fn create_invalid_url() {
    let result = create("test/bad/url");
    assert!(result.is_err());
}

#[test]
fn create_valid_url() {
    let client = create("redis://127.0.0.1:6379/").unwrap();
    assert!(!client.is_connection_open());
}

#[test]
fn run_command() {
    let mut client = create("redis://127.0.0.1:6379/").unwrap();
    assert!(!client.is_connection_open());

    let value = client.run_command::<String>("ECHO", vec!["testing"]).unwrap();
    assert_eq!(value, "testing");

    assert!(client.is_connection_open());
}

#[test]
fn run_command_typed_response() {
    let mut client = create("redis://127.0.0.1:6379/").unwrap();
    assert!(!client.is_connection_open());

    let result = client.run_command_empty_response("SET", vec!["client_test1", "my_value"]);
    assert!(result.is_ok());

    assert!(client.is_connection_open());

    let value = client.run_command_string_response("GET", vec!["client_test1"]).unwrap();
    assert_eq!(value, "my_value");
}
