//! # commands
//!
//! Defines the redis commands exposed by the redis client.
//!

use client::Client;
use types::{RedisBoolResult, RedisEmptyResult, RedisStringResult};

/// Defines the redis commands exposed by the redis client.
impl Client {
    /// See redis [AUTH](https://redis.io/commands/auth) command.
    ///
    /// # Examples
    ///
    /// ```
    /// # match simple_redis::create("redis://127.0.0.1:6379/") {
    /// #     Ok(mut client) =>  {
    ///           match client.auth("my_password") {
    ///               Err(error) => println!("Auth error: {}", error),
    ///               _ => println!("Authenticated")
    ///           }
    /// #     },
    /// #     Err(error) => println!("Unable to create Redis client: {}", error)
    /// # }
    /// ```
    ///
    pub fn auth(
        &mut self,
        password: &str,
    ) -> RedisEmptyResult {
        self.run_command_empty_response("AUTH", vec![password])
    }

    /// See redis [ECHO](https://redis.io/commands/echo) command.
    pub fn echo(
        &mut self,
        value: &str,
    ) -> RedisStringResult {
        self.run_command_string_response("ECHO", vec![value])
    }

    /// See redis [PUBLISH](https://redis.io/commands/publish) command.
    ///
    /// # Examples
    ///
    /// ```
    /// # match simple_redis::create("redis://127.0.0.1:6379/") {
    /// #     Ok(mut client) =>  {
    ///           match client.publish("important_notifications", "message text") {
    ///               Err(error) => println!("Publish error: {}", error),
    ///               _ => println!("Message published")
    ///           }
    /// #     },
    /// #     Err(error) => println!("Unable to create Redis client: {}", error)
    /// # }
    /// ```
    ///
    pub fn publish(
        &mut self,
        channel: &str,
        message: &str,
    ) -> RedisEmptyResult {
        self.run_command_empty_response("PUBLISH", vec![channel, message])
    }

    /// See redis [GET](https://redis.io/commands/get) command.
    ///
    /// # Examples
    ///
    /// ```
    /// # match simple_redis::create("redis://127.0.0.1:6379/") {
    /// #     Ok(mut client) =>  {
    ///           match client.get("my_key") {
    ///               Ok(value) => println!("Read value from Redis: {}", value),
    ///               Err(error) => println!("Unable to get value from Redis: {}", error)
    ///           }
    /// #     },
    /// #     Err(error) => println!("Unable to create Redis client: {}", error)
    /// # }
    /// ```
    ///
    pub fn get(
        self: &mut Client,
        key: &str,
    ) -> RedisStringResult {
        self.run_command_string_response("GET", vec![key])
    }

    /// See redis [SET](https://redis.io/commands/set) command.
    ///
    /// # Examples
    ///
    /// ```
    /// # match simple_redis::create("redis://127.0.0.1:6379/") {
    /// #     Ok(mut client) =>  {
    ///           match client.set("my_key", "my_value") {
    ///               Err(error) => println!("Unable to set value in Redis: {}", error),
    ///               _ => println!("Value set in Redis")
    ///           }
    /// #     },
    /// #     Err(error) => println!("Unable to create Redis client: {}", error)
    /// # }
    /// ```
    ///
    pub fn set(
        self: &mut Client,
        key: &str,
        value: &str,
    ) -> RedisEmptyResult {
        self.run_command_empty_response("SET", vec![key, value])
    }

    /// See redis [SETEX](https://redis.io/commands/setex) command.
    ///
    /// # Examples
    ///
    /// ```
    /// # match simple_redis::create("redis://127.0.0.1:6379/") {
    /// #     Ok(mut client) =>  {
    ///           match client.setex("my_key", "my_value", 10) {
    ///               Err(error) => println!("Unable to set value in Redis: {}", error),
    ///               _ => println!("Value set in Redis and will expire in 10 seconds")
    ///           }
    /// #     },
    /// #     Err(error) => println!("Unable to create Redis client: {}", error)
    /// # }
    /// ```
    ///
    pub fn setex(
        &mut self,
        key: &str,
        value: &str,
        seconds: usize,
    ) -> RedisEmptyResult {
        self.run_command_empty_response("SETEX", vec![key, &*seconds.to_string(), value])
    }

    /// See redis [SETNX](https://redis.io/commands/setnx) command.
    pub fn setnx(
        &mut self,
        key: &str,
        value: &str,
    ) -> RedisEmptyResult {
        self.run_command_empty_response("SETNX", vec![key, value])
    }

    /// See redis [GETSET](https://redis.io/commands/getset) command.
    pub fn getset(
        &mut self,
        key: &str,
        value: &str,
    ) -> RedisStringResult {
        self.run_command_string_response("GETSET", vec![key, value])
    }

    /// See redis [DEL](https://redis.io/commands/del) command.
    pub fn del(
        &mut self,
        key: &str,
    ) -> RedisEmptyResult {
        self.run_command_empty_response("DEL", vec![key])
    }

    /// See redis [EXISTS](https://redis.io/commands/exists) command.
    pub fn exists(
        &mut self,
        key: &str,
    ) -> RedisBoolResult {
        self.run_command_bool_response("EXISTS", vec![key])
    }

    /// See redis [EXPIRE](https://redis.io/commands/expire) command.
    pub fn expire(
        &mut self,
        key: &str,
        seconds: usize,
    ) -> RedisEmptyResult {
        self.run_command_empty_response("EXPIRE", vec![key, &*seconds.to_string()])
    }

    /// See redis [PERSIST](https://redis.io/commands/persist) command.
    pub fn persist(
        &mut self,
        key: &str,
    ) -> RedisEmptyResult {
        self.run_command_empty_response("PERSIST", vec![key])
    }

    /// See redis [RENAME](https://redis.io/commands/rename) command.
    pub fn rename(
        &mut self,
        key: &str,
        new_key: &str,
    ) -> RedisEmptyResult {
        self.run_command_empty_response("RENAME", vec![key, new_key])
    }
}

#[cfg(test)]
mod tests {
    use client;
    use std::{thread, time};

    #[test]
    fn create_invalid_url() {
        let result = client::create("test/bad/url");
        assert!(result.is_err());
    }

    #[test]
    fn create_valid_url() {
        let result = client::create("redis://127.0.0.1:6379/");
        assert!(result.is_ok());

        match result {
            Ok(client) => assert!(!client.is_connection_open()),
            _ => panic!("test error"),
        };
    }

    #[test]
    fn run_command() {
        match client::create("redis://127.0.0.1:6379/") {
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
    fn auth() {
        match client::create("redis://127.0.0.1:6379/") {
            Ok(mut client) => {
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
            _ => panic!("test error"),
        };
    }

    #[test]
    fn echo() {
        match client::create("redis://127.0.0.1:6379/") {
            Ok(mut client) => {
                assert!(!client.is_connection_open());

                match client.echo("testing") {
                    Ok(value) => assert_eq!(value, "testing"),
                    _ => panic!("test error"),
                }

                assert!(client.is_connection_open());
            }
            _ => panic!("test error"),
        };
    }

    #[test]
    fn publish() {
        match client::create("redis://127.0.0.1:6379/") {
            Ok(mut client) => {
                assert!(!client.is_connection_open());

                let result = client.publish("publish_channel", "test message");
                assert!(result.is_ok());

                assert!(client.is_connection_open());
            }
            _ => panic!("test error"),
        };
    }

    #[test]
    fn pub_sub() {
        match client::create("redis://127.0.0.1:6379/") {
            Ok(mut subscriber) => {
                assert!(!subscriber.is_connection_open());

                let mut result = subscriber.subscribe("pub_sub");
                assert!(result.is_ok());

                thread::spawn(
                    || {
                        thread::sleep(time::Duration::from_secs(2));

                        match client::create("redis://127.0.0.1:6379/") {
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

                        match client::create("redis://127.0.0.1:6379/") {
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
            _ => panic!("test error"),
        };
    }

    #[test]
    fn pub_psub_simple() {
        match client::create("redis://127.0.0.1:6379/") {
            Ok(mut subscriber) => {
                assert!(!subscriber.is_connection_open());

                let result = subscriber.psubscribe("pub_psub_simple::123");
                assert!(result.is_ok());

                thread::spawn(
                    || {
                        thread::sleep(time::Duration::from_secs(2));

                        match client::create("redis://127.0.0.1:6379/") {
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
            _ => panic!("test error"),
        };
    }

    #[test]
    fn pub_psub_pattern() {
        match client::create("redis://127.0.0.1:6379/") {
            Ok(mut subscriber) => {
                assert!(!subscriber.is_connection_open());

                let mut result = subscriber.psubscribe("pub_psub_pattern::*");
                assert!(result.is_ok());

                thread::spawn(
                    || {
                        thread::sleep(time::Duration::from_secs(2));

                        match client::create("redis://127.0.0.1:6379/") {
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

                        match client::create("redis://127.0.0.1:6379/") {
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
            _ => panic!("test error"),
        };
    }

    #[test]
    fn set_get() {
        match client::create("redis://127.0.0.1:6379/") {
            Ok(mut client) => {
                assert!(!client.is_connection_open());

                let result = client.set("set_get", "my_value");
                assert!(result.is_ok());

                assert!(client.is_connection_open());

                match client.get("set_get") {
                    Ok(value) => assert_eq!(value, "my_value"),
                    _ => panic!("test error"),
                }
            }
            _ => panic!("test error"),
        };
    }

    #[test]
    fn setex() {
        match client::create("redis://127.0.0.1:6379/") {
            Ok(mut client) => {
                assert!(!client.is_connection_open());

                let result = client.setex("setex", "my_value", 1);
                assert!(result.is_ok());

                assert!(client.is_connection_open());

                match client.get("setex") {
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
            _ => panic!("test error"),
        };
    }

    #[test]
    fn del_setnx() {
        match client::create("redis://127.0.0.1:6379/") {
            Ok(mut client) => {
                assert!(!client.is_connection_open());

                let mut result = client.set("del_setnx", "my_value");
                assert!(result.is_ok());

                assert!(client.is_connection_open());

                match client.get("del_setnx") {
                    Ok(value) => assert_eq!(value, "my_value"),
                    _ => panic!("test error"),
                }

                result = client.setnx("del_setnx", "my_value2");
                assert!(result.is_ok());

                match client.get("del_setnx") {
                    Ok(value) => assert_eq!(value, "my_value"),
                    _ => panic!("test error"),
                }

                result = client.del("del_setnx");
                assert!(result.is_ok());

                let string_result = client.get("del_setnx");
                assert!(string_result.is_err());

                result = client.setnx("del_setnx", "my_value2");
                assert!(result.is_ok());

                match client.get("del_setnx") {
                    Ok(value) => assert_eq!(value, "my_value2"),
                    _ => panic!("test error"),
                }
            }
            _ => panic!("test error"),
        };
    }

    #[test]
    fn getset() {
        match client::create("redis://127.0.0.1:6379/") {
            Ok(mut client) => {
                assert!(!client.is_connection_open());

                let result = client.set("getset", "my_value");
                assert!(result.is_ok());

                assert!(client.is_connection_open());

                match client.get("getset") {
                    Ok(value) => assert_eq!(value, "my_value"),
                    _ => panic!("test error"),
                }

                match client.getset("getset", "my_value2") {
                    Ok(value) => assert_eq!(value, "my_value"),
                    _ => panic!("test error"),
                }

                match client.get("getset") {
                    Ok(value) => assert_eq!(value, "my_value2"),
                    _ => panic!("test error"),
                }
            }
            _ => panic!("test error"),
        };
    }

    #[test]
    fn exists() {
        match client::create("redis://127.0.0.1:6379/") {
            Ok(mut client) => {
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
            _ => panic!("test error"),
        };
    }

    #[test]
    fn expire() {
        match client::create("redis://127.0.0.1:6379/") {
            Ok(mut client) => {
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
            _ => panic!("test error"),
        };
    }

    #[test]
    fn persist() {
        match client::create("redis://127.0.0.1:6379/") {
            Ok(mut client) => {
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
            _ => panic!("test error"),
        };
    }

    #[test]
    fn rename() {
        match client::create("redis://127.0.0.1:6379/") {
            Ok(mut client) => {
                assert!(!client.is_connection_open());

                let mut result = client.set("rename", "my_value");
                assert!(result.is_ok());

                assert!(client.is_connection_open());

                result = client.del("rename2");
                assert!(result.is_ok());

                match client.get("rename") {
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

                match client.get("rename2") {
                    Ok(value) => assert_eq!(value, "my_value"),
                    _ => panic!("test error"),
                }
            }
            _ => panic!("test error"),
        };
    }
}
