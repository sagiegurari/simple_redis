extern crate simple_redis;
use simple_redis::types::ErrorInfo::TimeoutError;
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

    thread::spawn(|| {
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
    });

    match subscriber.get_message(0) {
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

    thread::spawn(|| {
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
    });

    match subscriber.get_message(0) {
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

    thread::spawn(|| {
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
    });

    match subscriber.get_message(0) {
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

    thread::spawn(|| {
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
    });

    match subscriber.get_message(0) {
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

    thread::spawn(|| {
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
    });

    match subscriber.get_message(0) {
        Ok(message) => {
            let payload: String = message.get_payload().unwrap();
            assert_eq!(payload, "good")
        }
        _ => panic!("test error"),
    }
}

#[test]
fn pub_sub_timeout() {
    let mut subscriber = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    assert!(!subscriber.is_connection_open());

    let result = subscriber.subscribe("pub_sub_timeout");
    assert!(result.is_ok());

    let message_result = subscriber.get_message(500);

    assert!(message_result.is_err());

    match message_result {
        Err(error) => {
            match error.info {
                TimeoutError(description) => {
                    println!("Got timeout error: {}", description);
                    ()
                }
                _ => panic!("Invalid Error Type: {}", error),
            }
        }
        _ => panic!("Invalid Result"),
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
fn pexpire() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    assert!(!client.is_connection_open());

    let mut result = client.set("pexpire", "my_value");
    assert!(result.is_ok());

    assert!(client.is_connection_open());

    match client.exists("pexpire") {
        Ok(value) => assert!(value),
        _ => panic!("test error"),
    }

    result = client.pexpire("pexpire", 50);
    assert!(result.is_ok());

    match client.exists("pexpire") {
        Ok(value) => assert!(value),
        _ => panic!("test error"),
    }

    thread::sleep(time::Duration::from_millis(75));

    match client.exists("pexpire") {
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

#[test]
fn keys() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    client.set("keys_1", "12345").unwrap();
    client.set("keys_2", "12345").unwrap();
    client.set("keys_3", "12345").unwrap();
    let result = client.keys("keys_*").unwrap();
    assert_eq!(result.len(), 3);
    assert!(result.contains(&String::from("keys_1")));
    assert!(result.contains(&String::from("keys_2")));
    assert!(result.contains(&String::from("keys_3")));
}

#[test]
fn hget_hset_hdel() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    client.del("hget_hset_hdel").unwrap();

    let mut bool_result = client.hexists("hget_hset_hdel", "field1").unwrap();
    assert!(!bool_result);
    bool_result = client.hexists("hget_hset_hdel", "field2").unwrap();
    assert!(!bool_result);

    client.hset("hget_hset_hdel", "field1", 12.5f64).unwrap();
    client.hset("hget_hset_hdel", "field2", "test").unwrap();

    bool_result = client.hexists("hget_hset_hdel", "field1").unwrap();
    assert!(bool_result);
    bool_result = client.hexists("hget_hset_hdel", "field2").unwrap();
    assert!(bool_result);

    let float_result = client.hget::<f64>("hget_hset_hdel", "field1").unwrap();
    assert_eq!(float_result, 12.5f64);

    let mut string_result = client.hget_string("hget_hset_hdel", "field1").unwrap();
    assert_eq!(string_result, "12.5");
    string_result = client.hget_string("hget_hset_hdel", "field2").unwrap();
    assert_eq!(string_result, "test");

    client.hdel("hget_hset_hdel", "field1").unwrap();
    client.hdel("hget_hset_hdel", "field2").unwrap();

    let mut bool_result = client.hexists("hget_hset_hdel", "field1").unwrap();
    assert!(!bool_result);
    bool_result = client.hexists("hget_hset_hdel", "field2").unwrap();
    assert!(!bool_result);
}

#[test]
fn hgetall() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    client.del("hgetall").unwrap();

    client.hset("hgetall", "field1", 12.5f64).unwrap();
    client.hset("hgetall", "field2", "test").unwrap();

    let mut bool_result = client.hexists("hgetall", "field1").unwrap();
    assert!(bool_result);
    bool_result = client.hexists("hgetall", "field2").unwrap();
    assert!(bool_result);

    let float_result = client.hget::<f64>("hgetall", "field1").unwrap();
    assert_eq!(float_result, 12.5f64);

    let mut string_result = client.hget_string("hgetall", "field1").unwrap();
    assert_eq!(string_result, "12.5");
    string_result = client.hget_string("hgetall", "field2").unwrap();
    assert_eq!(string_result, "test");

    let map = client.hgetall("hgetall").unwrap();
    assert_eq!(map.get("field1").unwrap(), "12.5");
    assert_eq!(map.get("field2").unwrap(), "test");
}

#[test]
fn hsetnx() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    client.del("hsetnx").unwrap();

    let bool_result = client.hexists("hsetnx", "field").unwrap();
    assert!(!bool_result);

    client.hsetnx("hsetnx", "field", "test").unwrap();

    let mut string_result = client.hget_string("hsetnx", "field").unwrap();
    assert_eq!(string_result, "test");

    client.hsetnx("hsetnx", "field", "test2").unwrap();

    string_result = client.hget_string("hsetnx", "field").unwrap();
    assert_eq!(string_result, "test");
}

#[test]
fn hkeys() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    client.del("hkeys").unwrap();

    client.hset("hkeys", "field1", 12.5f64).unwrap();
    client.hset("hkeys", "field2", "test").unwrap();

    let result = client.hkeys("hkeys").unwrap();
    assert_eq!(result.len(), 2);
    assert!(result.contains(&String::from("field1")));
    assert!(result.contains(&String::from("field2")));
}

#[test]
fn hvals() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    client.del("hvals").unwrap();

    client.hset("hvals", "field1", 12.5f64).unwrap();
    client.hset("hvals", "field2", "test").unwrap();

    let result = client.hvals("hvals").unwrap();
    assert_eq!(result.len(), 2);
    assert!(result.contains(&String::from("12.5")));
    assert!(result.contains(&String::from("test")));
}

#[test]
fn list_multi() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    client.del("list_multi").unwrap();

    client.lpush::<f32>("list_multi", 12.5f32).unwrap();
    client.lpush("list_multi", "first").unwrap();
    client.rpush("list_multi", "last").unwrap();

    let mut int_result = client.llen("list_multi").unwrap();
    assert_eq!(int_result, 3);

    let float_result = client.lindex::<f32>("list_multi", 1).unwrap();
    assert_eq!(float_result, 12.5f32);
    let mut string_result = client.lindex_string("list_multi", 1).unwrap();
    assert_eq!(string_result, "12.5");

    string_result = client.lpop("list_multi").unwrap();
    assert_eq!(string_result, "first");
    int_result = client.llen("list_multi").unwrap();
    assert_eq!(int_result, 2);

    client.lset("list_multi", 0, 1).unwrap();
    client.lset("list_multi", 1, 2).unwrap();
    client.rpush("list_multi", 3).unwrap();
    client.rpush("list_multi", 4).unwrap();
    client.rpush("list_multi", "last").unwrap();

    let vec_result = client.lrange("list_multi", 1, 3).unwrap();
    assert_eq!(vec_result[0], "2");
    assert_eq!(vec_result[1], "3");
    assert_eq!(vec_result[2], "4");

    client.rpush("list_multi", "last").unwrap();
    client.rpush("list_multi", "last").unwrap();
    int_result = client.llen("list_multi").unwrap();
    assert_eq!(int_result, 7);
    client.lrem("list_multi", 2, "last").unwrap();
    int_result = client.llen("list_multi").unwrap();
    assert_eq!(int_result, 5);

    client.del("list_multi").unwrap();

    client.lpush::<f32>("list_multi", 12.5f32).unwrap();
    client.lpush("list_multi", "first").unwrap();
    client.rpush("list_multi", "last").unwrap();

    string_result = client.rpop("list_multi").unwrap();
    assert_eq!(string_result, "last");
    int_result = client.llen("list_multi").unwrap();
    assert_eq!(int_result, 2);
}

#[test]
fn list_xpushx() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    client.del("list_xpushx").unwrap();

    client.lpushx("list_xpushx", "test").unwrap();
    client.rpushx("list_xpushx", "test").unwrap();

    let bool_result = client.exists("list_xpushx").unwrap();
    assert!(!bool_result);
}

#[test]
fn list_ltrim() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    client.del("list_ltrim").unwrap();

    client.rpush("list_ltrim", "1").unwrap();
    client.rpush("list_ltrim", "2").unwrap();
    client.rpush("list_ltrim", "3").unwrap();

    client.ltrim("list_ltrim", 1, -1).unwrap();

    let vec_result = client.lrange("list_ltrim", 0, -1).unwrap();
    assert_eq!(vec_result[0], "2");
    assert_eq!(vec_result[1], "3");
}

#[test]
fn set_all() {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    client.del("set_all_1").unwrap();
    client.del("set_all_2").unwrap();

    client.sadd("set_all_1", "member1").unwrap();
    client.sadd("set_all_1", "member2").unwrap();
    client.sadd("set_all_1", "member3").unwrap();
    client.sadd("set_all_1", "member4").unwrap();

    let mut int_result = client.scard("set_all_1").unwrap();
    assert_eq!(int_result, 4);

    client.sadd("set_all_2", "member1").unwrap();
    client.sadd("set_all_2", "member3").unwrap();
    client.sadd("set_all_2", "member100").unwrap();

    int_result = client.scard("set_all_2").unwrap();
    assert_eq!(int_result, 3);

    let mut vec_result = client.sdiff(vec!["set_all_1", "set_all_2"]).unwrap();
    assert_eq!(vec_result.len(), 2);
    assert!(vec_result.contains(&String::from("member2")));
    assert!(vec_result.contains(&String::from("member4")));

    vec_result = client.smembers("set_all_1").unwrap();
    assert_eq!(vec_result.len(), 4);
    assert!(vec_result.contains(&String::from("member1")));
    assert!(vec_result.contains(&String::from("member2")));
    assert!(vec_result.contains(&String::from("member3")));
    assert!(vec_result.contains(&String::from("member4")));

    let mut bool_result = client.sismember("set_all_1", "member4").unwrap();
    assert!(bool_result);
    bool_result = client.sismember("set_all_1", "BAD").unwrap();
    assert!(!bool_result);

    bool_result = client.sismember("set_all_1", "member100").unwrap();
    assert!(!bool_result);
    client.smove("set_all_2", "set_all_1", "member100").unwrap();
    bool_result = client.sismember("set_all_1", "member100").unwrap();
    assert!(bool_result);

    client.srem("set_all_1", "member100").unwrap();
    bool_result = client.sismember("set_all_1", "member100").unwrap();
    assert!(!bool_result);
}
