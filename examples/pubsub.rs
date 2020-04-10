use simple_redis;
use simple_redis::Message;
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
            subscriber
                .fetch_messages(&|message: Message| -> bool {
                    let payload: String = message.get_payload().unwrap();
                    println!("Read message: {}", payload);
                    assert_eq!(payload, "example message");

                    true
                })
                .unwrap();

            // we get the first message again, since we subscribed via pattern to same channel
            subscriber
                .fetch_messages(&|message: Message| -> bool {
                    let payload: String = message.get_payload().unwrap();
                    println!("Read message: {}", payload);
                    assert_eq!(payload, "example message");

                    true
                })
                .unwrap();

            // wait for another message which only comes on the pattern channel
            subscriber
                .fetch_messages(&|message: Message| -> bool {
                    let payload: String = message.get_payload().unwrap();
                    println!("Read message: {}", payload);
                    assert_eq!(payload, "test message");

                    true
                })
                .unwrap();

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

            subscriber
                .fetch_messages(&|message: Message| -> bool {
                    let payload: String = message.get_payload().unwrap();
                    println!("Read message: {}", payload);
                    assert_eq!(payload, "second message");

                    true
                })
                .unwrap();
        }
        Err(error) => println!("Unable to create Redis client: {}", error),
    }
}
