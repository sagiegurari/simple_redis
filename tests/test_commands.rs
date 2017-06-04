extern crate simple_redis;
use std::{thread, time};

#[test]
fn auth() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    assert!(!client.is_connection_open());

    match client.echo("testing") {
        Ok(value) => assert_eq!(value, "testing"),
        _ => panic!("test error"),
    }

    assert!(client.is_connection_open());

    let result = client.auth("my_password");
    /// we are running with redis without auth, so we should be getting an error
    assert!(result.is_err());

    assert!(!client.is_connection_open());

    match client.echo("testing") {
        Ok(value) => assert_eq!(value, "testing"),
        _ => panic!("test error"),
    }

    assert!(client.is_connection_open());
}

#[test]
fn echo() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    assert!(!client.is_connection_open());

    match client.echo("testing") {
        Ok(value) => assert_eq!(value, "testing"),
        _ => panic!("test error"),
    }

    assert!(client.is_connection_open());
}

#[test]
fn publish() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    assert!(!client.is_connection_open());

    let result = client.publish("publish_channel", "test message");
    assert!(result.is_ok());

    assert!(client.is_connection_open());
}

#[test]
fn pub_sub() {
    let mut subscriber = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    assert!(!subscriber.is_connection_open());

    let mut result = subscriber.subscribe("pub_sub");
    assert!(result.is_ok());

    thread::spawn(
        || {
            thread::sleep(time::Duration::from_secs(2));

            match simple_redis::create("redis://127.0.0.1:6379/") {
                Ok(mut publisher) => {
                    assert!(!publisher.is_connection_open());

                    let result = publisher.publish("pub_sub", "test pub_sub message");
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

    result = subscriber.subscribe("pub_sub2");
    assert!(result.is_ok());

    result = subscriber.unsubscribe("pub_sub");
    assert!(result.is_ok());

    thread::spawn(
        || {
            thread::sleep(time::Duration::from_secs(2));

            match simple_redis::create("redis://127.0.0.1:6379/") {
                Ok(mut publisher) => {
                    assert!(!publisher.is_connection_open());

                    let mut result = publisher.publish("pub_sub", "bad");
                    assert!(result.is_ok());

                    assert!(publisher.is_connection_open());

                    thread::sleep(time::Duration::from_secs(1));

                    result = publisher.publish("pub_sub2", "good");
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

#[test]
fn pub_psub_simple() {
    let mut subscriber = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    assert!(!subscriber.is_connection_open());

    let result = subscriber.psubscribe("pub_psub_simple::123");
    assert!(result.is_ok());

    thread::spawn(
        || {
            thread::sleep(time::Duration::from_secs(2));

            match simple_redis::create("redis://127.0.0.1:6379/") {
                Ok(mut publisher) => {
                    assert!(!publisher.is_connection_open());

                    let result = publisher.publish("pub_psub_simple::123", "test pub_sub message");
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
}

#[test]
fn pub_psub_pattern() {
    let mut subscriber = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    assert!(!subscriber.is_connection_open());

    let mut result = subscriber.psubscribe("pub_psub_pattern::*");
    assert!(result.is_ok());

    thread::spawn(
        || {
            thread::sleep(time::Duration::from_secs(2));

            match simple_redis::create("redis://127.0.0.1:6379/") {
                Ok(mut publisher) => {
                    assert!(!publisher.is_connection_open());

                    let result = publisher.publish("pub_psub_pattern::123", "test pub_sub message");
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

    result = subscriber.psubscribe("pub_psub_pattern2::*");
    assert!(result.is_ok());

    result = subscriber.punsubscribe("pub_psub_pattern::*");
    assert!(result.is_ok());

    thread::spawn(
        || {
            thread::sleep(time::Duration::from_secs(2));

            match simple_redis::create("redis://127.0.0.1:6379/") {
                Ok(mut publisher) => {
                    assert!(!publisher.is_connection_open());

                    let mut result = publisher.publish("pub_psub_pattern::123", "bad");
                    assert!(result.is_ok());

                    assert!(publisher.is_connection_open());

                    thread::sleep(time::Duration::from_secs(1));

                    result = publisher.publish("pub_psub_pattern2::123", "good");
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

#[test]
fn set_get_string() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    assert!(!client.is_connection_open());

    let result = client.set("set_get_string", "my_value");
    assert!(result.is_ok());

    assert!(client.is_connection_open());

    match client.get_string("set_get_string") {
        Ok(value) => assert_eq!(value, "my_value"),
        _ => panic!("test error"),
    }
}

#[test]
fn set_get_i32() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    assert!(!client.is_connection_open());

    let result = client.set("set_get_i32", 32i32);
    assert!(result.is_ok());

    assert!(client.is_connection_open());

    match client.get::<i32>("set_get_i32") {
        Ok(value) => assert_eq!(value, 32i32),
        _ => panic!("test error"),
    }
}

#[test]
fn set_get_f64() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    assert!(!client.is_connection_open());

    let result = client.set("set_get_f64", 45.5f64);
    assert!(result.is_ok());

    assert!(client.is_connection_open());

    match client.get::<f64>("set_get_f64") {
        Ok(value) => assert_eq!(value, 45.5f64),
        _ => panic!("test error"),
    }
}

#[test]
fn set_get_bool() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    assert!(!client.is_connection_open());

    let result = client.set("set_get_bool", true);
    assert!(result.is_ok());

    assert!(client.is_connection_open());

    match client.get::<bool>("set_get_bool") {
        Ok(value) => assert_eq!(value, true),
        _ => panic!("test error"),
    }
}

#[test]
fn setex() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    assert!(!client.is_connection_open());

    let result = client.setex("setex", "my_value", 1);
    assert!(result.is_ok());

    assert!(client.is_connection_open());

    match client.get_string("setex") {
        Ok(value) => assert_eq!(value, "my_value"),
        _ => panic!("test error"),
    }

    match client.exists("setex") {
        Ok(value) => assert!(value),
        _ => panic!("test error"),
    }

    thread::sleep(time::Duration::from_secs(1));
    thread::sleep(time::Duration::from_millis(250));

    match client.exists("setex") {
        Ok(value) => assert!(!value),
        _ => panic!("test error"),
    }
}

#[test]
fn del_setnx() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    assert!(!client.is_connection_open());

    let mut result = client.set("del_setnx", "my_value");
    assert!(result.is_ok());

    assert!(client.is_connection_open());

    match client.get_string("del_setnx") {
        Ok(value) => assert_eq!(value, "my_value"),
        _ => panic!("test error"),
    }

    result = client.setnx("del_setnx", "my_value2");
    assert!(result.is_ok());

    match client.get_string("del_setnx") {
        Ok(value) => assert_eq!(value, "my_value"),
        _ => panic!("test error"),
    }

    result = client.del("del_setnx");
    assert!(result.is_ok());

    let string_result = client.get_string("del_setnx");
    assert!(string_result.is_err());

    result = client.setnx("del_setnx", "my_value2");
    assert!(result.is_ok());

    match client.get_string("del_setnx") {
        Ok(value) => assert_eq!(value, "my_value2"),
        _ => panic!("test error"),
    }
}

#[test]
fn getset_i32() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    assert!(!client.is_connection_open());

    let result = client.set("getset_i32", 50);
    assert!(result.is_ok());

    assert!(client.is_connection_open());

    match client.get_string("getset_i32") {
        Ok(value) => assert_eq!(value, "50"),
        _ => panic!("test error"),
    }

    match client.getset::<i32, i32>("getset_i32", 100) {
        Ok(value) => assert_eq!(value, 50),
        _ => panic!("test error"),
    }

    match client.get_string("getset_i32") {
        Ok(value) => assert_eq!(value, "100"),
        _ => panic!("test error"),
    }
}

#[test]
fn getset_string() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    assert!(!client.is_connection_open());

    let result = client.set("getset_string", "my_value");
    assert!(result.is_ok());

    assert!(client.is_connection_open());

    match client.get_string("getset_string") {
        Ok(value) => assert_eq!(value, "my_value"),
        _ => panic!("test error"),
    }

    match client.getset_string("getset_string", "my_value2") {
        Ok(value) => assert_eq!(value, "my_value"),
        _ => panic!("test error"),
    }

    match client.get_string("getset_string") {
        Ok(value) => assert_eq!(value, "my_value2"),
        _ => panic!("test error"),
    }
}

#[test]
fn exists() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    assert!(!client.is_connection_open());

    let result = client.set("exists_true", "my_value");
    assert!(result.is_ok());

    assert!(client.is_connection_open());

    match client.exists("exists_true") {
        Ok(value) => assert!(value),
        _ => panic!("test error"),
    }

    match client.exists("exists_false") {
        Ok(value) => assert!(!value),
        _ => panic!("test error"),
    }
}

#[test]
fn expire() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    assert!(!client.is_connection_open());

    let mut result = client.set("expire", "my_value");
    assert!(result.is_ok());

    assert!(client.is_connection_open());

    match client.exists("expire") {
        Ok(value) => assert!(value),
        _ => panic!("test error"),
    }

    result = client.expire("expire", 1);
    assert!(result.is_ok());

    match client.exists("expire") {
        Ok(value) => assert!(value),
        _ => panic!("test error"),
    }

    thread::sleep(time::Duration::from_secs(1));
    thread::sleep(time::Duration::from_millis(250));

    match client.exists("expire") {
        Ok(value) => assert!(!value),
        _ => panic!("test error"),
    }
}

#[test]
fn persist() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    assert!(!client.is_connection_open());

    let mut result = client.set("persist", "my_value");
    assert!(result.is_ok());

    assert!(client.is_connection_open());

    match client.exists("persist") {
        Ok(value) => assert!(value),
        _ => panic!("test error"),
    }

    result = client.expire("persist", 1);
    assert!(result.is_ok());

    match client.exists("persist") {
        Ok(value) => assert!(value),
        _ => panic!("test error"),
    }

    result = client.persist("persist");
    assert!(result.is_ok());

    thread::sleep(time::Duration::from_secs(1));
    thread::sleep(time::Duration::from_millis(250));

    match client.exists("persist") {
        Ok(value) => assert!(value),
        _ => panic!("test error"),
    }
}

#[test]
fn rename() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    assert!(!client.is_connection_open());

    let mut result = client.set("rename", "my_value");
    assert!(result.is_ok());

    assert!(client.is_connection_open());

    result = client.del("rename2");
    assert!(result.is_ok());

    match client.get_string("rename") {
        Ok(value) => assert_eq!(value, "my_value"),
        _ => panic!("test error"),
    }

    match client.exists("rename") {
        Ok(value) => assert!(value),
        _ => panic!("test error"),
    }

    match client.exists("rename2") {
        Ok(value) => assert!(!value),
        _ => panic!("test error"),
    }

    result = client.rename("rename", "rename2");
    assert!(result.is_ok());

    match client.exists("rename") {
        Ok(value) => assert!(!value),
        _ => panic!("test error"),
    }

    match client.exists("rename2") {
        Ok(value) => assert!(value),
        _ => panic!("test error"),
    }

    match client.get_string("rename2") {
        Ok(value) => assert_eq!(value, "my_value"),
        _ => panic!("test error"),
    }
}

#[test]
fn renamenx() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    client.set("renamenx1", "value1").unwrap();
    client.set("renamenx2", "value2").unwrap();
    client.set("renamenx3", "value3").unwrap();
    client.del("renamenx3").unwrap();

    client.renamenx("renamenx1", "renamenx3").unwrap();
    let mut string_result = client.get_string("renamenx3").unwrap();
    assert_eq!(string_result, "value1");
    let bool_result = client.exists("renamenx1").unwrap();
    assert!(!bool_result);

    client.renamenx("renamenx2", "renamenx3").unwrap();
    string_result = client.get_string("renamenx3").unwrap();
    assert_eq!(string_result, "value1");
    string_result = client.get_string("renamenx2").unwrap();
    assert_eq!(string_result, "value2");
}

#[test]
fn append() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    client.set("append", "value").unwrap();
    let mut result = client.get_string("append").unwrap();
    assert_eq!(result, "value");
    client.append("append", "12345").unwrap();
    result = client.get_string("append").unwrap();
    assert_eq!(result, "value12345");
}

#[test]
fn incr() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    client.set("incr", "10").unwrap();
    let mut result = client.incr("incr").unwrap();
    assert_eq!(result, 11);
    result = client.incr("incr").unwrap();
    assert_eq!(result, 12);
    let string_result = client.get_string("incr").unwrap();
    assert_eq!(string_result, "12");
}

#[test]
fn incrby() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    client.set("incrby", "10").unwrap();
    let mut result = client.incrby("incrby", 20).unwrap();
    assert_eq!(result, 30);
    result = client.incrby("incrby", 70).unwrap();
    assert_eq!(result, 100);
    let string_result = client.get_string("incrby").unwrap();
    assert_eq!(string_result, "100");
}

#[test]
fn incrbyfloat() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    client.set("incrbyfloat", "10").unwrap();
    let mut result = client.incrbyfloat("incrbyfloat", 1.5).unwrap();
    assert_eq!(result, 11.5);
    result = client.incrbyfloat("incrbyfloat", 8.5).unwrap();
    assert_eq!(result, 20f64);
    let string_result = client.get_string("incrbyfloat").unwrap();
    assert_eq!(string_result, "20");
}

#[test]
fn strlen() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    client.set("strlen", "12345").unwrap();
    let result = client.strlen("strlen").unwrap();
    assert_eq!(result, 5);
}
