//! # commands
//!
//! Defines the redis commands exposed by the redis client.
//!

use client::Client;
use std::str::FromStr;
use types::{RedisArg, RedisBoolResult, RedisEmptyResult, RedisResult, RedisStringResult};

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
    ///           match client.get::<i64>("my_key") {
    ///               Ok(value) => println!("Read value from Redis: {}", value),
    ///               Err(error) => println!("Unable to get value from Redis: {}", error)
    ///           }
    /// #     },
    /// #     Err(error) => println!("Unable to create Redis client: {}", error)
    /// # }
    /// ```
    ///
    pub fn get<T: FromStr>(
        self: &mut Client,
        key: &str,
    ) -> RedisResult<T> {
        self.run_command_from_string_response("GET", vec![key])
    }

    /// See redis [GET](https://redis.io/commands/get) command.<br>
    /// This function will always return a String response.
    ///
    /// # Examples
    ///
    /// ```
    /// # match simple_redis::create("redis://127.0.0.1:6379/") {
    /// #     Ok(mut client) =>  {
    ///           match client.get_string("my_key") {
    ///               Ok(value) => println!("Read value from Redis: {}", value),
    ///               Err(error) => println!("Unable to get value from Redis: {}", error)
    ///           }
    /// #     },
    /// #     Err(error) => println!("Unable to create Redis client: {}", error)
    /// # }
    /// ```
    ///
    pub fn get_string(
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
    pub fn set<T: RedisArg>(
        self: &mut Client,
        key: &str,
        value: T,
    ) -> RedisEmptyResult {
        self.run_command_empty_response("SET", vec![key, &value.to_string()])
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
    pub fn setex<T: RedisArg>(
        &mut self,
        key: &str,
        value: T,
        seconds: usize,
    ) -> RedisEmptyResult {
        self.run_command_empty_response("SETEX", vec![key, &*seconds.to_string(), &value.to_string()])
    }

    /// See redis [SETNX](https://redis.io/commands/setnx) command.
    pub fn setnx<T: RedisArg>(
        &mut self,
        key: &str,
        value: T,
    ) -> RedisEmptyResult {
        self.run_command_empty_response("SETNX", vec![key, &value.to_string()])
    }

    /// See redis [GETSET](https://redis.io/commands/getset) command.
    pub fn getset<T: RedisArg, V: FromStr>(
        &mut self,
        key: &str,
        value: T,
    ) -> RedisResult<V> {
        self.run_command_from_string_response::<V>("GETSET", vec![key, &value.to_string()])
    }

    /// See redis [GETSET](https://redis.io/commands/getset) command.
    pub fn getset_string<T: RedisArg>(
        &mut self,
        key: &str,
        value: T,
    ) -> RedisStringResult {
        self.run_command_string_response("GETSET", vec![key, &value.to_string()])
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

    /// See redis [RENAMENX](https://redis.io/commands/renamenx) command.
    pub fn renamenx(
        &mut self,
        key: &str,
        new_key: &str,
    ) -> RedisEmptyResult {
        self.run_command_empty_response("RENAMENX", vec![key, new_key])
    }

    /// See redis [APPEND](https://redis.io/commands/append) command.
    pub fn append(
        &mut self,
        key: &str,
        value: &str,
    ) -> RedisEmptyResult {
        self.run_command_empty_response("APPEND", vec![key, value])
    }

    /// See redis [INCR](https://redis.io/commands/incr) command.
    pub fn incr(
        &mut self,
        key: &str,
    ) -> RedisResult<i64> {
        self.run_command::<i64>("INCR", vec![key])
    }

    /// See redis [INCRBY](https://redis.io/commands/incrby) command.
    pub fn incrby<T: RedisArg>(
        &mut self,
        key: &str,
        value: T,
    ) -> RedisResult<i64> {
        self.run_command::<i64>("INCRBY", vec![key, &*value.to_string()])
    }

    /// See redis [INCRBYFLOAT](https://redis.io/commands/incrbyfloat) command.
    pub fn incrbyfloat<T: RedisArg>(
        &mut self,
        key: &str,
        value: T,
    ) -> RedisResult<f64> {
        self.run_command::<f64>("INCRBYFLOAT", vec![key, &*value.to_string()])
    }

    /// See redis [STRLEN](https://redis.io/commands/strlen) command.
    pub fn strlen(
        &mut self,
        key: &str,
    ) -> RedisResult<i32> {
        self.run_command::<i32>("STRLEN", vec![key])
    }
}

#[cfg(test)]
mod tests {
    use client;

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
        let mut client = client::create("redis://127.0.0.1:6379/").unwrap();

        assert!(!client.is_connection_open());

        match client.run_command::<String>("ECHO", vec!["testing"]) {
            Ok(value) => assert_eq!(value, "testing"),
            _ => panic!("test error"),
        }

        assert!(client.is_connection_open());
    }

    #[test]
    fn set_get() {
        let mut client = client::create("redis://127.0.0.1:6379/").unwrap();

        assert!(!client.is_connection_open());

        let result = client.set("set_get", "my_value");
        assert!(result.is_ok());

        assert!(client.is_connection_open());

        match client.get_string("set_get") {
            Ok(value) => assert_eq!(value, "my_value"),
            _ => panic!("test error"),
        }
    }
}
