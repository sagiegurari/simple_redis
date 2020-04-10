use simple_redis;
use simple_redis::Message;
use std::{thread, time};

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
        Ok(mut client) => assert!(!client.is_connection_open()),
        _ => panic!("test error"),
    };
}

#[test]
fn run_command() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    assert!(!client.is_connection_open());

    match client.run_command::<String>("ECHO", vec!["testing"]) {
        Ok(value) => assert_eq!(value, "testing"),
        _ => panic!("test error"),
    }

    assert!(client.is_connection_open());
}

#[test]
fn run_command_invalid() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    let result = client.run_command::<String>("BADCOMMAND", vec!["testing"]);

    assert!(result.is_err());
}

#[test]
fn run_command_typed_response() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    assert!(!client.is_connection_open());

    let empty_result = client.run_command_empty_response("SET", vec!["int_test_1", "my_value"]);
    assert!(empty_result.is_ok());

    assert!(client.is_connection_open());

    let string_result = client
        .run_command_string_response("GET", vec!["int_test_1"])
        .unwrap();
    assert_eq!(string_result, "my_value");

    let error_result = client.run_command_string_response("BADCOMMAND", vec!["int_test_1"]);
    assert!(error_result.is_err());
}

#[test]
fn quit_no_connection() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    assert!(!client.is_connection_open());

    client.quit().unwrap();

    assert!(!client.is_connection_open());

    match client.echo("testing") {
        Ok(value) => assert_eq!(value, "testing"),
        _ => panic!("test error"),
    }

    assert!(client.is_connection_open());
}

#[test]
fn quit_no_subscriptions() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    assert!(!client.is_connection_open());

    match client.echo("testing") {
        Ok(value) => assert_eq!(value, "testing"),
        _ => panic!("test error"),
    }

    assert!(client.is_connection_open());

    client.quit().unwrap();

    assert!(!client.is_connection_open());

    match client.echo("testing") {
        Ok(value) => assert_eq!(value, "testing"),
        _ => panic!("test error"),
    }

    assert!(client.is_connection_open());
}

#[test]
fn quit_internal_subscriptions() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    assert!(!client.is_connection_open());

    match client.echo("testing") {
        Ok(value) => assert_eq!(value, "testing"),
        _ => panic!("test error"),
    }

    assert!(client.is_connection_open());

    assert!(!client.is_subscribed("quit_internal_subscriptions"));
    assert!(!client.is_psubscribed("quit_internal_*"));

    client.subscribe("quit_internal_subscriptions").unwrap();
    client.psubscribe("quit_internal_*").unwrap();

    assert!(client.is_subscribed("quit_internal_subscriptions"));
    assert!(client.is_psubscribed("quit_internal_*"));

    client.quit().unwrap();
    assert!(!client.is_connection_open());

    assert!(!client.is_subscribed("quit_internal_subscriptions"));
    assert!(!client.is_psubscribed("quit_internal_*"));

    match client.echo("testing") {
        Ok(value) => assert_eq!(value, "testing"),
        _ => panic!("test error"),
    }

    assert!(client.is_connection_open());
}

#[test]
fn quit_live_subscriptions() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    assert!(!client.is_connection_open());

    match client.echo("testing") {
        Ok(value) => assert_eq!(value, "testing"),
        _ => panic!("test error"),
    }

    assert!(client.is_connection_open());

    assert!(!client.is_subscribed("quit_live_subscriptions"));
    assert!(!client.is_psubscribed("quit_live_*"));

    client.subscribe("quit_live_subscriptions").unwrap();
    client.psubscribe("quit_live_*").unwrap();

    assert!(client.is_subscribed("quit_live_subscriptions"));
    assert!(client.is_psubscribed("quit_live_*"));

    thread::spawn(|| {
        thread::sleep(time::Duration::from_secs(2));
        let mut publisher = simple_redis::create("redis://127.0.0.1:6379/").unwrap();
        publisher
            .publish("quit_live_subscriptions_TEST", "test pub_sub message")
            .unwrap();
    });

    client
        .fetch_messages(&|message: Message| -> bool {
            let payload: String = message.get_payload().unwrap();
            assert_eq!(payload, "test pub_sub message");
            true
        })
        .unwrap();

    client.quit().unwrap();
    assert!(!client.is_connection_open());

    assert!(!client.is_subscribed("quit_live_subscriptions"));
    assert!(!client.is_psubscribed("quit_live_*"));

    match client.echo("testing") {
        Ok(value) => assert_eq!(value, "testing"),
        _ => panic!("test error"),
    }

    assert!(client.is_connection_open());
}
