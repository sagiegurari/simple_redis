#![deny(missing_docs)]
#![deny(non_camel_case_types)]
#![cfg_attr(feature="clippy", feature(plugin))]

//! # simple_redis
//!
//! Simple [redis](https://redis.io/) client based on [redis-rs](https://crates.io/crates/redis) with internal
//! connection handling.
//!
//! This library provides a very basic, simple API for the most common redis operations.<br>
//! While not as comprehensive or flexiable as [redis-rs](https://crates.io/crates/redis),
//! it does provide a simpler api for most common use cases and operations as well as automatic internal connection
//! handling.<br>
//! Connection validation is done before every operation invocation, so there is no need to create/release or validate
//! connections before running any Redis operation.<br>
//! However, this comes at a small performance cost of PING operation to the redis server.
//!
//! # Examples
//!
//! ```
//! extern crate simple_redis;
//!
//! fn main() {
//!     match simple_redis::create("redis://127.0.0.1:6379/") {
//!         Ok(mut client) =>  {
//!             println!("Created Redis Client");
//!
//!             match client.set("my_key", "my_value") {
//!                 Err(error) => println!("Unable to set value in Redis: {}", error),
//!                 _ => println!("Value set in Redis")
//!             }
//!
//!             match client.get("my_key") {
//!                 Ok(value) => println!("Read value from Redis: {}", value),
//!                 Err(error) => println!("Unable to get value from Redis: {}", error)
//!             }
//!
//!             /// run some command that is not built in the library
//!             match client.run_command::<String>("ECHO", vec!["testing"]) {
//!                 Ok(value) => assert_eq!(value, "testing"),
//!                 _ => panic!("test error"),
//!             }
//!         },
//!         Err(error) => println!("Unable to create Redis client: {}", error)
//!     }
//! }
//! ```
//!

extern crate redis;

mod types;
mod connection;
pub mod commands;
pub mod client;

/// Redis Error struct
pub type RedisError = types::RedisError;

/// Redis result which either holds a value or a Redis error
pub type RedisResult<T> = types::RedisResult<T>;

/// Holds empty result or error
pub type RedisEmptyResult = types::RedisEmptyResult;

/// Holds string result or error
pub type RedisStringResult = types::RedisStringResult;

/// Constructs a new redis client.<br>
/// The redis connection string must be in the following format: `redis://[:<passwd>@]<hostname>[:port][/<db>]`
///
/// # Examples
///
/// ```
/// extern crate simple_redis;
///
/// fn main() {
///     match simple_redis::create("redis://127.0.0.1:6379/") {
///         Ok(client) => println!("Created Redis Client"),
///         Err(error) => println!("Unable to create Redis client: {}", error)
///     }
/// }
/// ```
pub fn create(connection_string: &str) -> Result<client::Client, RedisError> {
    client::create(connection_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_invalid_url() {
        let result = create("test/bad/url");
        assert!(result.is_err());
    }

    #[test]
    fn create_valid_url() {
        let result = create("redis://127.0.0.1:6379/");
        assert!(result.is_ok());

        match result {
            Ok(client) => assert!(!client.is_connection_open()),
            _ => panic!("test error"),
        };
    }
}
