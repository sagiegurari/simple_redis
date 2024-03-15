//! # commands
//!
//! Manages the redis connection and ensures it is valid.
//!

#[cfg(test)]
#[path = "./connection_test.rs"]
mod connection_test;

use crate::types::{RedisEmptyResult, RedisError, RedisResult};

/// The redis client which enables to invoke redis operations.
pub(crate) struct Connection {
    /// Holds the current redis connection
    connection: Option<redis::Connection>,
}

/// If the client connection is not open or not valid, this function will create
/// a new redis connection and modify the client to store this new connection.
fn open_connection(connection: &mut Connection, client: &redis::Client) -> RedisEmptyResult {
    let output: RedisEmptyResult;

    if !connection.is_connection_open() {
        output = match client.get_connection() {
            Ok(redis_connection) => {
                connection.connection = Some(redis_connection);
                Ok(())
            }
            Err(error) => Err(RedisError::RedisError(error)),
        }
    } else {
        output = Ok(());
    }

    output
}

impl Connection {
    /// Returns true if the currently stored connection is valid, otherwise false.<br>
    /// There is no need to call this function as any redis operation invocation will
    /// ensure a valid connection is created.
    pub(crate) fn is_connection_open(self: &mut Connection) -> bool {
        let open;

        match self.connection {
            Some(ref mut redis_connection) => {
                let result: redis::RedisResult<()> = redis::cmd("PING").query(redis_connection);

                open = result.is_ok();
            }
            None => open = false,
        }

        open
    }

    pub(crate) fn get_redis_connection(
        self: &mut Connection,
        client: &redis::Client,
    ) -> RedisResult<&mut redis::Connection> {
        match open_connection(self, client) {
            Err(error) => Err(error),
            _ => match self.connection {
                Some(ref mut redis_connection) => Ok(redis_connection),
                None => Err(RedisError::Description("Redis connection not available.")),
            },
        }
    }
}

/// Creates and returns a new connection
pub(crate) fn create() -> Connection {
    Connection { connection: None }
}
