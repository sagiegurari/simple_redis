#![deny(anonymous_parameters,
        const_err,
        dead_code,
        deprecated,
        deprecated_attr,
        exceeding_bitshifts,
        extra_requirement_in_impl,
        fat_ptr_transmutes,
        illegal_floating_point_literal_pattern,
        improper_ctypes,
        invalid_type_param_default,
        legacy_constructor_visibility,
        legacy_directory_ownership,
        legacy_imports,
        missing_copy_implementations,
        missing_docs,
        missing_fragment_specifier,
        mutable_transmutes,
        no_mangle_const_items,
        no_mangle_generic_items,
        non_camel_case_types,
        non_shorthand_field_patterns,
        non_snake_case,
        non_upper_case_globals,
        overflowing_literals,
        parenthesized_params_in_types_and_modules,
        path_statements,
        patterns_in_fns_without_body,
        plugin_as_library,
        private_in_public,
        private_no_mangle_fns,
        private_no_mangle_statics,
        renamed_and_removed_lints,
        resolve_trait_on_defaulted_unit,
        safe_extern_statics,
        stable_features,
        trivial_numeric_casts,
        unconditional_recursion,
        unions_with_drop_fields,
        unknown_crate_types,
        unknown_lints,
        unreachable_code,
        unreachable_patterns,
        unsafe_code,
        unstable_features,
        unused_allocation,
        unused_assignments,
        unused_attributes,
        unused_comparisons,
        unused_extern_crates,
        unused_features,
        unused_import_braces,
        unused_imports,
        unused_macros,
        unused_must_use,
        unused_mut,
        unused_parens,
        unused_qualifications,
        unused_unsafe,
        unused_variables,
        while_true)]
#![warn(unknown_lints)]
#![allow(box_pointers,
        missing_debug_implementations,
        trivial_casts,
        unused_results,
        variant_size_differences,
        warnings)]
#![cfg_attr(feature="clippy", feature(plugin))]

//! # simple_redis
//!
//! [![crates.io](https://img.shields.io/crates/v/simple_redis.svg)](https://crates.io/crates/simple_redis)
//! [![license](https://img.shields.io/crates/l/simple_redis.svg)](https://github.
//! com/sagiegurari/simple_redis/blob/master/LICENSE)
//!
//! > Simple and resilient [redis](https://redis.io/) client based on [redis-rs](https://crates.io/crates/redis) with
//! internal connection and subscription handling.
//!
//! This library provides a very basic, simple API for the most common redis operations.<br>
//! While not as comprehensive or flexiable as [redis-rs](https://crates.io/crates/redis),
//! it does provide a simpler api for most common use cases and operations as well as automatic and resilient internal
//! connection and subscription (pubsub) handling.<br>
//! In addition, the entire API is accessible via redis client and there is no need to manage connection or pubsub
//! instances in parallel.<br>
//! <br>
//! Connection resiliency is managed by verifying the internally managed connection before every operation against the
//! redis server.<br>
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
//! # Installation
//! In order to use this library, just add it as a dependency:
//!
//! ```ini
//! [dependencies]
//! simple_redis = "*"
//! ```
//!

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
