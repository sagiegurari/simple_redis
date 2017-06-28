//! # types
//!
//! Defines the various types and aliases used or exposed by the simple_redis library.
//!

#[cfg(test)]
#[path = "./types_test.rs"]
mod types_test;

extern crate redis;
use std::error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
/// Holds the error information
pub enum ErrorInfo {
    /// Root redis error
    RedisError(redis::RedisError),
    /// Description text of the error reason
    Description(&'static str),
    /// TimeoutError error
    TimeoutError(&'static str)
}

#[derive(Debug)]
/// Redis Error struct
pub struct RedisError {
    /// Holds the error information
    pub info: ErrorInfo
}

impl error::Error for RedisError {
    /// A short description of the error.
    fn description(&self) -> &str {
        match self.info {
            ErrorInfo::RedisError(ref cause) => cause.description(),
            ErrorInfo::Description(description) => description,
            ErrorInfo::TimeoutError(description) => description,
        }
    }

    /// The lower-level cause of this error, if any.
    fn cause(&self) -> Option<&error::Error> {
        match self.info {
            ErrorInfo::RedisError(ref cause) => Some(cause as &error::Error),
            _ => None,
        }
    }
}

impl Display for RedisError {
    /// Formats the value using the given formatter.
    fn fmt(
        &self,
        format: &mut fmt::Formatter,
    ) -> Result<(), fmt::Error> {
        match self.info {
            ErrorInfo::RedisError(ref cause) => cause.fmt(format),
            ErrorInfo::Description(description) => description.fmt(format),
            ErrorInfo::TimeoutError(description) => description.fmt(format),
        }
    }
}

/// Defines a redis command argument
pub trait RedisArg: Sized + ToString {}

macro_rules! as_redis_arg {
    ($t:ty) => (
        impl RedisArg for $t {}
    )
}

impl<'a> RedisArg for &'a str {}

as_redis_arg!(i8);
as_redis_arg!(i16);
as_redis_arg!(u16);
as_redis_arg!(i32);
as_redis_arg!(u32);
as_redis_arg!(i64);
as_redis_arg!(u64);
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

/// Holds pubsub message result or error
pub type RedisMessageResult = RedisResult<Message>;

/// Holds string result or error
pub type RedisStringResult = RedisResult<String>;

/// Holds bool result or error
pub type RedisBoolResult = RedisResult<bool>;
