//! # types
//!
//! Defines the various types and aliases used or exposed by the simple_redis library.
//!

#[cfg(test)]
#[path = "./types_test.rs"]
mod types_test;

use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
/// Holds the error information
pub enum RedisError {
    /// Root redis error
    RedisError(redis::RedisError),
    /// Description text of the error reason
    Description(&'static str),
}

impl Display for RedisError {
    /// Formats the value using the given formatter.
    fn fmt(&self, format: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::RedisError(ref cause) => cause.fmt(format),
            Self::Description(description) => description.fmt(format),
        }
    }
}

impl Error for RedisError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::RedisError(error) => Some(error),
            Self::Description(_) => None,
        }
    }
}

/// Defines a redis command argument
pub trait RedisArg: Sized + ToString {}

macro_rules! as_redis_arg {
    ($t:ty) => {
        impl RedisArg for $t {}
    };
}

impl<'a> RedisArg for &'a str {}

as_redis_arg!(u8);
as_redis_arg!(i8);
as_redis_arg!(i16);
as_redis_arg!(u16);
as_redis_arg!(i32);
as_redis_arg!(u32);
as_redis_arg!(i64);
as_redis_arg!(u64);
as_redis_arg!(i128);
as_redis_arg!(u128);
as_redis_arg!(f32);
as_redis_arg!(f64);
as_redis_arg!(isize);
as_redis_arg!(usize);
as_redis_arg!(bool);

/// PubSub message
pub type Message = redis::Msg;

/// Redis result which either holds a value or a Redis error
pub type RedisResult<T> = Result<T, RedisError>;

/// Holds empty result or error
pub type RedisEmptyResult = RedisResult<()>;

/// Holds string result or error
pub type RedisStringResult = RedisResult<String>;

/// Holds bool result or error
pub type RedisBoolResult = RedisResult<bool>;

#[derive(Debug, Clone, Copy, Default)]
/// Enable to modify blocking operations.
pub struct Interrupts {
    /// Notify blocking operation to stop
    pub stop: bool,
    /// Next polling time in millies
    pub next_polling_time: Option<u64>,
}

impl Interrupts {
    /// Returns a new instance.
    pub fn new() -> Interrupts {
        Default::default()
    }
}
