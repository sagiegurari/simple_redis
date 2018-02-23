extern crate simple_redis;
use std::{thread, time};

fn main() {
    match simple_redis::create("redis://127.0.0.1:6379/") {
        Ok(mut subscriber) => {
            println!("Created Redis Client");

            // simple subscription to pub_sub_example channel
            subscriber.subscribe("pub_sub_example").unwrap();

            // pattern based subscription
            subscriber.psubscribe("pub_sub_*").unwrap();

            thread::spawn(|| {
                thread::sleep(time::Duration::from_secs(2));

                match simple_redis::create("redis://127.0.0.1:6379/") {
                    Ok(mut publisher) => {
                        publisher
                            .publish("pub_sub_example", "example message")
                            .unwrap();

                        publisher.publish("pub_sub_test", "test message").unwrap();
                    }
                    _ => panic!("test error"),
                };
            });

            // get next message
            match subscriber.get_message(0) {
                Ok(message) => {
                    let payload: String = message.get_payload().unwrap();
                    println!("Read message: {}", payload);
                    assert_eq!(payload, "example message")
                }
                _ => panic!("test error"),
            }

            // we get the first message again, since we subscribed via pattern to same channel
            match subscriber.get_message(0) {
                Ok(message) => {
                    let payload: String = message.get_payload().unwrap();
                    println!("Read message: {}", payload);
                    assert_eq!(payload, "example message")
                }
                _ => panic!("test error"),
            }

            // wait for another message which only comes on the pattern channel
            match subscriber.get_message(0) {
                Ok(message) => {
                    let payload: String = message.get_payload().unwrap();
                    println!("Read message: {}", payload);
                    assert_eq!(payload, "test message")
                }
                _ => panic!("test error"),
            }

            subscriber.subscribe("pub_sub_second_run").unwrap();
            subscriber.unsubscribe("pub_sub_example").unwrap();
            subscriber.punsubscribe("pub_sub_*").unwrap();

            thread::spawn(|| {
                thread::sleep(time::Duration::from_secs(2));

                match simple_redis::create("redis://127.0.0.1:6379/") {
                    Ok(mut publisher) => {
                        publisher
                            .publish("pub_sub_example", "example message")
                            .unwrap();

                        publisher
                            .publish("pub_sub_second_run", "second message")
                            .unwrap();
                    }
                    _ => panic!("test error"),
                };
            });

            match subscriber.get_message(0) {
                Ok(message) => {
                    let payload: String = message.get_payload().unwrap();
                    println!("Read message: {}", payload);
                    assert_eq!(payload, "second message")
                }
                _ => panic!("test error"),
            }
        }
        Err(error) => println!("Unable to create Redis client: {}", error),
    }
}
