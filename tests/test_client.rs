extern crate simple_redis;

#[test]
fn create_invalid_url() {
    let result = simple_redis::create("test/bad/url");
    assert!(result.is_err());
}

#[test]
fn create_valid_url() {
    let result = simple_redis::create("redis://127.0.0.1:6379/");
    assert!(result.is_ok());

    match result {
        Ok(client) => assert!(!client.is_connection_open()),
        _ => panic!("test error"),
    };
}

#[test]
fn run_command() {
    match simple_redis::create("redis://127.0.0.1:6379/") {
        Ok(mut client) => {
            assert!(!client.is_connection_open());

            match client.run_command::<String>("ECHO", vec!["testing"]) {
                Ok(value) => assert_eq!(value, "testing"),
                _ => panic!("test error"),
            }

            assert!(client.is_connection_open());
        }
        _ => panic!("test error"),
    };
}

#[test]
fn run_command_typed_response() {
    match simple_redis::create("redis://127.0.0.1:6379/") {
        Ok(mut client) => {
            assert!(!client.is_connection_open());

            let result = client.run_command_empty_response("SET", vec!["int_test_1", "my_value"]);
            assert!(result.is_ok());

            assert!(client.is_connection_open());

            match client.run_command_string_response("GET", vec!["int_test_1"]) {
                Ok(value) => assert_eq!(value, "my_value"),
                _ => panic!("test error"),
            }
        }
        _ => panic!("test error"),
    };
}
