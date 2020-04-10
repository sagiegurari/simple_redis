use simple_redis;
use simple_redis::{Interrupts, Message};
use std::{thread, time};

#[test]
fn pub_sub() {
    match simple_redis::create("redis://127.0.0.1:6379/") {
        Ok(mut subscriber) => {
            assert!(!subscriber.is_connection_open());

            let mut result = subscriber.subscribe("int_pub_sub");
            assert!(result.is_ok());

            thread::spawn(|| {
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
            });

            subscriber
                .fetch_messages(
                    &mut |message: Message| -> bool {
                        let payload: String = message.get_payload().unwrap();
                        assert_eq!(payload, "test pub_sub message");
                        true
                    },
                    &mut || -> Interrupts { Interrupts::new() },
                )
                .unwrap();

            result = subscriber.subscribe("int_pub_sub2");
            assert!(result.is_ok());

            result = subscriber.unsubscribe("int_pub_sub");
            assert!(result.is_ok());

            thread::spawn(|| {
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
            });

            subscriber
                .fetch_messages(
                    &mut |message: Message| -> bool {
                        let payload: String = message.get_payload().unwrap();
                        assert_eq!(payload, "good");
                        true
                    },
                    &mut || -> Interrupts { Interrupts::new() },
                )
                .unwrap();

            thread::spawn(|| {
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
            });

            let mut counter = 0;
            subscriber
                .fetch_messages(
                    &mut |_message: Message| -> bool {
                        panic!("test error");
                    },
                    &mut || -> Interrupts {
                        counter = counter + 1;

                        let mut interrupts = Interrupts::new();
                        interrupts.stop = counter == 5;
                        interrupts.next_polling_time = Some(10);

                        interrupts
                    },
                )
                .unwrap();
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

            thread::spawn(|| {
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
            });

            subscriber
                .fetch_messages(
                    &mut |message: Message| -> bool {
                        let payload: String = message.get_payload().unwrap();
                        assert_eq!(payload, "test pub_sub message");
                        true
                    },
                    &mut || -> Interrupts { Interrupts::new() },
                )
                .unwrap();

            result = subscriber.psubscribe("int_pub_psub2::*");
            assert!(result.is_ok());

            result = subscriber.punsubscribe("int_pub_psub::*");
            assert!(result.is_ok());

            thread::spawn(|| {
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
            });

            subscriber
                .fetch_messages(
                    &mut |message: Message| -> bool {
                        let payload: String = message.get_payload().unwrap();
                        assert_eq!(payload, "good");
                        true
                    },
                    &mut || -> Interrupts { Interrupts::new() },
                )
                .unwrap();
        }
        _ => panic!("test error"),
    };
}
