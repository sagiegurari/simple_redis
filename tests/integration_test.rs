extern crate simple_redis;
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

#[test]
fn set_get() {
    match simple_redis::create("redis://127.0.0.1:6379/") {
        Ok(mut client) => {
            assert!(!client.is_connection_open());

            let result = client.set("int_set_get", "my_value");
            assert!(result.is_ok());

            assert!(client.is_connection_open());

            match client.get("int_set_get") {
                Ok(value) => assert_eq!(value, "my_value"),
                _ => panic!("test error"),
            }
        }
        _ => panic!("test error"),
    };
}

#[test]
fn pub_sub() {
    match simple_redis::create("redis://127.0.0.1:6379/") {
        Ok(mut subscriber) => {
            assert!(!subscriber.is_connection_open());

            let mut result = subscriber.subscribe("int_pub_sub");
            assert!(result.is_ok());

            thread::spawn(
                || {
                    thread::sleep(time::Duration::from_secs(2));

                    match simple_redis::create("redis://127.0.0.1:6379/") {
                        Ok(mut publisher) => {
                            assert!(!publisher.is_connection_open());

                            let result = publisher.publish("int_pub_sub", "test pub_sub message");
                            assert!(result.is_ok());

                            assert!(publisher.is_connection_open());
                        }
                        _ => panic!("test error"),
                    };
                }
            );

            match subscriber.get_message() {
                Ok(message) => {
                    let payload: String = message.get_payload().unwrap();
                    assert_eq!(payload, "test pub_sub message")
                }
                _ => panic!("test error"),
            }

            result = subscriber.subscribe("int_pub_sub2");
            assert!(result.is_ok());

            result = subscriber.unsubscribe("int_pub_sub");
            assert!(result.is_ok());

            thread::spawn(
                || {
                    thread::sleep(time::Duration::from_secs(2));

                    match simple_redis::create("redis://127.0.0.1:6379/") {
                        Ok(mut publisher) => {
                            assert!(!publisher.is_connection_open());

                            let mut result = publisher.publish("int_pub_sub", "bad");
                            assert!(result.is_ok());

                            assert!(publisher.is_connection_open());

                            thread::sleep(time::Duration::from_secs(1));

                            result = publisher.publish("int_pub_sub2", "good");
                            assert!(result.is_ok());
                        }
                        _ => panic!("test error"),
                    };
                }
            );

            match subscriber.get_message() {
                Ok(message) => {
                    let payload: String = message.get_payload().unwrap();
                    assert_eq!(payload, "good")
                }
                _ => panic!("test error"),
            }
        }
        _ => panic!("test error"),
    };
}

#[test]
fn pub_psub_pattern() {
    match simple_redis::create("redis://127.0.0.1:6379/") {
        Ok(mut subscriber) => {
            assert!(!subscriber.is_connection_open());

            let mut result = subscriber.psubscribe("int_pub_psub_pattern::*");
            assert!(result.is_ok());

            thread::spawn(
                || {
                    thread::sleep(time::Duration::from_secs(2));

                    match simple_redis::create("redis://127.0.0.1:6379/") {
                        Ok(mut publisher) => {
                            assert!(!publisher.is_connection_open());

                            let result = publisher.publish("int_pub_psub_pattern::123", "test pub_sub message");
                            assert!(result.is_ok());

                            assert!(publisher.is_connection_open());
                        }
                        _ => panic!("test error"),
                    };
                }
            );

            match subscriber.get_message() {
                Ok(message) => {
                    let payload: String = message.get_payload().unwrap();
                    assert_eq!(payload, "test pub_sub message")
                }
                _ => panic!("test error"),
            }

            result = subscriber.psubscribe("int_pub_psub_pattern2::*");
            assert!(result.is_ok());

            result = subscriber.punsubscribe("int_pub_psub_pattern::*");
            assert!(result.is_ok());

            thread::spawn(
                || {
                    thread::sleep(time::Duration::from_secs(2));

                    match simple_redis::create("redis://127.0.0.1:6379/") {
                        Ok(mut publisher) => {
                            assert!(!publisher.is_connection_open());

                            let mut result = publisher.publish("int_pub_psub_pattern::123", "bad");
                            assert!(result.is_ok());

                            assert!(publisher.is_connection_open());

                            thread::sleep(time::Duration::from_secs(1));

                            result = publisher.publish("int_pub_psub_pattern2::123", "good");
                            assert!(result.is_ok());
                        }
                        _ => panic!("test error"),
                    };
                }
            );

            match subscriber.get_message() {
                Ok(message) => {
                    let payload: String = message.get_payload().unwrap();
                    assert_eq!(payload, "good")
                }
                _ => panic!("test error"),
            }
        }
        _ => panic!("test error"),
    };
}
