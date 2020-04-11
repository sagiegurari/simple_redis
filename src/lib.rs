//! # simple_redis
//!
//! Simple and resilient [redis](https://redis.io/) client based on [redis-rs](https://crates.io/crates/redis) with
//! internal connection and subscription handling.
//!
//! This library provides a very basic, simple API for the most common redis operations.<br>
//! While not as comprehensive or flexiable as [redis-rs](https://crates.io/crates/redis),
//! it does provide a simpler api for most common use cases and operations as well as automatic and resilient internal
//! connection and subscription (pubsub) handling.<br>
//! In addition, the entire API is accessible via redis client and there is no need to manage connection or pubsub
//! instances in parallel.<br>
//!
//! ## Connection Resiliency
//!
//! Connection resiliency is managed by verifying the internally managed connection before every operation against the
//! redis server.<br>
//! In case of any connection issue, a new connection will be allocated to ensure the operation is invoked on a valid
//! connection only.<br>
//! However, this comes at a small performance cost of PING operation to the redis server.<br>
//!
//! ## Subscription Resiliency
//!
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
//!             match client.get_string("my_key") {
//!                 Ok(value) => println!("Read value from Redis: {}", value),
//!                 Err(error) => println!("Unable to get value from Redis: {}", error)
//!             };
//!
//!             match client.set("my_numeric_key", 255.5) {
//!                 Err(error) => println!("Unable to set value in Redis: {}", error),
//!                 _ => println!("Value set in Redis")
//!             };
//!
//!             match client.get::<f32>("my_numeric_key") {
//!                 Ok(value) => println!("Read value from Redis: {}", value),
//!                 Err(error) => println!("Unable to get value from Redis: {}", error)
//!             };
//!
//!             match client.hgetall("my_map") {
//!                 Ok(map) => {
//!                     match map.get("my_field") {
//!                         Some(value) => println!("Got field value from map: {}", value),
//!                         None => println!("Map field is emtpy"),
//!                     }
//!                 },
//!                 Err(error) => println!("Unable to read map from Redis: {}", error),
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
//! use simple_redis::{Interrupts, Message};
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
//!             // fetch messages from all subscriptions
//!             client.fetch_messages(
//!                 &mut |message: Message| -> bool {
//!                     let payload : String = message.get_payload().unwrap();
//!                     println!("Got message: {}", payload);
//!
//!                     // continue fetching
//!                     false
//!                 },
//!                 &mut || -> Interrupts { Interrupts::new() },
//!             ).unwrap();
//!         },
//!         Err(error) => println!("Unable to create Redis client: {}", error)
//!     }
//! }
//! ```
//!
//! ## Closing Connection
//!
//! ```rust
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
//!             match client.quit() {
//!                 Err(error) => println!("Error: {}", error),
//!                 _ => println!("Connection Closed.")
//!             }
//!         },
//!         Err(error) => println!("Unable to create Redis client: {}", error)
//!     }
//! }
//! ```
//!
//! # Installation
//! In order to use this library, just add it as a dependency:
//!
//! ```ini
//! [dependencies]
//! simple_redis = "*"
//! ```
//!
//! # Contributing
//! See [contributing guide](https://github.com/sagiegurari/simple_redis/blob/master/.github/CONTRIBUTING.md)
//!
//! # License
//! Developed by Sagie Gur-Ari and licensed under the
//! [Apache 2](https://github.com/sagiegurari/simple_redis/blob/master/LICENSE) open source license.
//!

#[cfg(test)]
#[path = "./lib_test.rs"]
mod lib_test;

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

pub mod client;
mod commands;
mod connection;
mod subscriber;
pub mod types;

/// Error Type
pub type RedisError = types::RedisError;

/// Error Info
pub type ErrorInfo = types::ErrorInfo;

/// PubSub message
pub type Message = types::Message;

/// Blocking operations interrupts
pub type Interrupts = types::Interrupts;

/// Redis result which either holds a value or a Redis error
pub type RedisResult<T> = types::RedisResult<T>;

/// Constructs a new redis client.<br>
/// The redis connection string must be in the following format: `redis://[:<passwd>@]<hostname>[:port][/<db>]`
///
/// # Arguments
///
/// * `connection_string` - The connection string in the format of: `redis://[:<passwd>@]<hostname>[:port][/<db>]`
///
/// # Example
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
