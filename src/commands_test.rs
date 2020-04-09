use crate::client;

#[test]
fn create_invalid_url() {
    let result = client::create("test/bad/url");
    assert!(result.is_err());
}

#[test]
fn create_valid_url() {
    let client = client::create("redis://127.0.0.1:6379/").unwrap();
    assert!(!client.is_connection_open());
}

#[test]
fn run_command() {
    let mut client = client::create("redis://127.0.0.1:6379/").unwrap();
    assert!(!client.is_connection_open());

    let value = client
        .run_command::<String>("ECHO", vec!["testing"])
        .unwrap();
    assert_eq!(value, "testing");

    assert!(client.is_connection_open());
}

#[test]
fn set_get() {
    let mut client = client::create("redis://127.0.0.1:6379/").unwrap();

    assert!(!client.is_connection_open());

    let result = client.set("set_get", "my_value");
    assert!(result.is_ok());

    assert!(client.is_connection_open());

    let value = client.get_string("set_get").unwrap();
    assert_eq!(value, "my_value");
}
