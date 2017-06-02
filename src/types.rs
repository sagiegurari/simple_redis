//! # types
//!
//! Defines the various types and aliases used or exposed by the simple_redis library.
//!

extern crate redis;
use std::error;
use std::fmt;

#[derive(Debug)]
/// Holds the error information
pub enum ErrorInfo {
    /// Root redis error
    RedisError(redis::RedisError),
    /// Description text of the error reason
    Description(&'static str)
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

impl fmt::Display for RedisError {
    /// Formats the value using the given formatter.
    fn fmt(
        &self,
        format: &mut fmt::Formatter,
    ) -> Result<(), fmt::Error> {
        match self.info {
            ErrorInfo::RedisError(ref cause) => cause.fmt(format),
            ErrorInfo::Description(description) => description.fmt(format),
        }
    }
}

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
