#![deny(missing_docs)]
#![deny(unused_must_use)]
#![deny(non_camel_case_types)]
#![cfg_attr(feature="clippy", feature(plugin))]

//! # simple_redis
//!
//! Simple [redis](https://redis.io/) client based on [redis-rs](https://crates.io/crates/redis) with internal
//! connection and subscription handling.
//!
//! This library provides a very basic, simple API for the most common redis operations.<br>
//! While not as comprehensive or flexiable as [redis-rs](https://crates.io/crates/redis),
//! it does provide a simpler api for most common use cases and operations as well as automatic internal connection
//! and subscription (pubsub) handling.<br>
//! In addition, the entire API is accessible via redis client and there is no need to manage connection or pubsub
//! instances in parallel.<br>
//! <br>
//! Connection resiliency is managed by verifying the connection before every operation against the redis server.<br>
//! In case of any connection issue, a new connection will be allocated to ensure the operation is invoked on a valid
//! connection only.<br>
//! However, this comes at a small performance cost of PING operation to the redis server.<br>
//! <br>
//! Subscription resiliency is ensured by recreating the internal pubsub and issuing new subscription requests
//! automatically in case of any error while fetching a message from the subscribed channels.
//!
//! # Examples
//!
//! ## Initialization and Simple Operations
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
//!             };
//!
//!             match client.get("my_key") {
//!                 Ok(value) => println!("Read value from Redis: {}", value),
//!                 Err(error) => println!("Unable to get value from Redis: {}", error)
//!             };
//!
//!             /// run some command that is not built in the library
//!             match client.run_command::<String>("ECHO", vec!["testing"]) {
//!                 Ok(value) => assert_eq!(value, "testing"),
//!                 _ => panic!("test error"),
//!             };
//!
//!             /// publish messages
//!             let result = client.publish("news_channel", "test message");
//!             assert!(result.is_ok());
//!         },
//!         Err(error) => println!("Unable to create Redis client: {}", error)
//!     }
//! }
//! ```
//!
//! ## Subscription Flow
//!
//! ```rust,no_run
//! extern crate simple_redis;
//!
//! fn main() {
//!     match simple_redis::create("redis://127.0.0.1:6379/") {
//!         Ok(mut client) =>  {
//!             println!("Created Redis Client");
//!
//!             let mut result = client.subscribe("important_notifications");
//!             assert!(result.is_ok());
//!             result = client.psubscribe("*_notifications");
//!             assert!(result.is_ok());
//!
//!             loop {
//!                 /// fetch next message
//!                 match client.get_message() {
//!                     Ok(message) => {
//!                         let payload: String = message.get_payload().unwrap();
//!                         assert_eq!(payload, "my important message")
//!                     }
//!                     _ => panic!("test error"),
//!                 }
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
mod subscriber;
mod commands;
pub mod client;

/// Error Type
pub type RedisError = types::RedisError;

/// PubSub message
pub type Message = types::Message;

/// Redis result which either holds a value or a Redis error
pub type RedisResult<T> = types::RedisResult<T>;

/// Holds empty result or error
pub type RedisEmptyResult = types::RedisEmptyResult;

/// Holds pubsub message result or error
pub type RedisMessageResult = types::RedisMessageResult;

/// Holds string result or error
pub type RedisStringResult = types::RedisStringResult;

/// Holds bool result or error
pub type RedisBoolResult = types::RedisBoolResult;

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
