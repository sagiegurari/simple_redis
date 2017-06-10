extern crate simple_redis;
use std::{thread, time};

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

            match subscriber.get_message(0) {
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

            match subscriber.get_message(0) {
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
fn pub_psub() {
    match simple_redis::create("redis://127.0.0.1:6379/") {
        Ok(mut subscriber) => {
            assert!(!subscriber.is_connection_open());

            let mut result = subscriber.psubscribe("int_pub_psub::*");
            assert!(result.is_ok());

            thread::spawn(
                || {
                    thread::sleep(time::Duration::from_secs(2));

                    match simple_redis::create("redis://127.0.0.1:6379/") {
                        Ok(mut publisher) => {
                            assert!(!publisher.is_connection_open());

                            let result = publisher.publish("int_pub_psub::123", "test pub_sub message");
                            assert!(result.is_ok());

                            assert!(publisher.is_connection_open());
                        }
                        _ => panic!("test error"),
                    };
                }
            );

            match subscriber.get_message(0) {
                Ok(message) => {
                    let payload: String = message.get_payload().unwrap();
                    assert_eq!(payload, "test pub_sub message")
                }
                _ => panic!("test error"),
            }

            result = subscriber.psubscribe("int_pub_psub2::*");
            assert!(result.is_ok());

            result = subscriber.punsubscribe("int_pub_psub::*");
            assert!(result.is_ok());

            thread::spawn(
                || {
                    thread::sleep(time::Duration::from_secs(2));

                    match simple_redis::create("redis://127.0.0.1:6379/") {
                        Ok(mut publisher) => {
                            assert!(!publisher.is_connection_open());

                            let mut result = publisher.publish("int_pub_psub::123", "bad");
                            assert!(result.is_ok());

                            assert!(publisher.is_connection_open());

                            thread::sleep(time::Duration::from_secs(1));

                            result = publisher.publish("int_pub_psub2::123", "good");
                            assert!(result.is_ok());
                        }
                        _ => panic!("test error"),
                    };
                }
            );

            match subscriber.get_message(0) {
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
