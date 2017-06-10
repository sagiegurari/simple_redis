//! # client
//!
//! Implements the redis client capabilities.
//!

extern crate redis;
use connection;
use std::str::FromStr;
use subscriber;
use types::{ErrorInfo, RedisBoolResult, RedisEmptyResult, RedisError, RedisMessageResult, RedisResult, RedisStringResult};

/// The redis client which enables to invoke redis operations.
pub struct Client {
    /// Internal redis client
    client: redis::Client,
    /// Holds the current redis connection
    connection: connection::Connection,
    /// Internal subscriber
    subscriber: subscriber::Subscriber
}

fn run_command_on_connection<T: redis::FromRedisValue>(
    connection: &redis::Connection,
    command: &str,
    args: Vec<&str>,
) -> RedisResult<T> {
    let mut cmd = redis::cmd(command);

    for arg in args {
        cmd.arg(arg);
    }

    let result: redis::RedisResult<T> = cmd.query(connection);

    match result {
        Err(error) => Err(RedisError { info: ErrorInfo::RedisError(error) }),
        Ok(output) => Ok(output),
    }
}

impl Client {
    /// Returns true if the currently stored connection is valid, otherwise false.<br>
    /// There is no need to call this function as any redis operation invocation will
    /// ensure a valid connection is created.
    pub fn is_connection_open(self: &Client) -> bool {
        self.connection.is_connection_open()
    }

    /// Invokes the requested command with the provided arguments (all provided via args) and returns the operation
    /// response.<br>
    /// This function ensures that we have a valid connection and it is used internally by all other exposed
    /// commands.<br>
    /// This function is also public to enable invoking operations that are not directly exposed by the client.
    ///
    /// # Arguments
    ///
    /// * `command` - The Redis command, for example: `GET`
    /// * `args` - Vector of arguments for the given command
    ///
    /// # Example
    ///
    /// ```
    /// # match simple_redis::create("redis://127.0.0.1:6379/") {
    /// #     Ok(mut client) =>  {
    ///           match client.run_command::<String>("ECHO", vec!["testing"]) {
    ///               Ok(value) => assert_eq!(value, "testing"),
    ///               _ => panic!("test error"),
    ///           }
    /// #     },
    /// #     Err(error) => println!("Unable to create Redis client: {}", error)
    /// # }
    /// ```
    pub fn run_command<T: redis::FromRedisValue>(
        self: &mut Client,
        command: &str,
        args: Vec<&str>,
    ) -> RedisResult<T> {
        match self.connection.get_redis_connection(&self.client) {
            Ok(ref connection) => run_command_on_connection::<T>(connection, command, args),
            Err(error) => Err(error),
        }
    }

    /// invokes the run_command and returns typed result
    pub fn run_command_from_string_response<T: FromStr>(
        self: &mut Client,
        command: &str,
        args: Vec<&str>,
    ) -> RedisResult<T> {
        match self.run_command::<String>(command, args) {
            Ok(value) => {
                match T::from_str(&value) {
                    Ok(typed_value) => Ok(typed_value),
                    _ => Err(RedisError { info: ErrorInfo::Description("Unable to parse output value.") }),
                }
            }
            Err(error) => Err(error),
        }
    }

    /// invokes the run_command but returns empty result
    pub fn run_command_empty_response(
        self: &mut Client,
        command: &str,
        args: Vec<&str>,
    ) -> RedisEmptyResult {
        self.run_command(command, args)
    }

    /// invokes the run_command but returns string result
    pub fn run_command_string_response(
        self: &mut Client,
        command: &str,
        args: Vec<&str>,
    ) -> RedisStringResult {
        self.run_command(command, args)
    }

    /// invokes the run_command but returns bool result
    pub fn run_command_bool_response(
        self: &mut Client,
        command: &str,
        args: Vec<&str>,
    ) -> RedisBoolResult {
        self.run_command(command, args)
    }

    /// Subscribes to the provided channel.<br>
    /// Actual subscription only occurs at the first call to get_message.
    ///
    /// # Arguments
    ///
    /// * `channel` - The channel name, for example: `level_info`
    ///
    /// # Example
    ///
    /// ```
    /// # match simple_redis::create("redis://127.0.0.1:6379/") {
    /// #     Ok(mut client) =>  {
    ///           client.subscribe("important_notifications");
    /// #
    /// #         println!("Subscribed to channel.");
    /// #     },
    /// #     Err(error) => println!("Unable to create Redis client: {}", error)
    /// # }
    /// ```
    pub fn subscribe(
        self: &mut Client,
        channel: &str,
    ) -> RedisEmptyResult {
        self.subscriber.subscribe(channel)
    }

    /// Subscribes to the provided channel pattern.<br>
    /// Actual subscription only occurs at the first call to get_message.
    ///
    /// # Arguments
    ///
    /// * `channel` - The channel pattern, for example: `level_*`
    ///
    /// # Example
    ///
    /// ```
    /// # match simple_redis::create("redis://127.0.0.1:6379/") {
    /// #     Ok(mut client) =>  {
    ///           client.psubscribe("important_notifications*");
    /// #
    /// #         println!("Subscribed to channel.");
    /// #     },
    /// #     Err(error) => println!("Unable to create Redis client: {}", error)
    /// # }
    /// ```
    pub fn psubscribe(
        self: &mut Client,
        channel: &str,
    ) -> RedisEmptyResult {
        self.subscriber.psubscribe(channel)
    }

    /// Unsubscribes from the provided channel.
    pub fn unsubscribe(
        self: &mut Client,
        channel: &str,
    ) -> RedisEmptyResult {
        self.subscriber.unsubscribe(channel)
    }

    /// Unsubscribes from the provided channel pattern.
    pub fn punsubscribe(
        self: &mut Client,
        channel: &str,
    ) -> RedisEmptyResult {
        self.subscriber.punsubscribe(channel)
    }

    /// Fetches the next message from any of the subscribed channels.<br>
    /// This function will return a TimeoutError in case no message was read in the provided timeout value (defined in
    /// millies).<br>
    /// If the provided timeout value is 0, there will be no timeout and the call will block until a message is read or
    /// a connection error happens.
    ///
    /// # Arguments
    ///
    /// * `timeout` - The timeout value in millies or 0 for no timeout
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # match simple_redis::create("redis://127.0.0.1:6379/") {
    /// #     Ok(mut client) =>  {
    ///           client.subscribe("important_notifications");
    ///
    ///           // get next message (wait up to 5 seconds, 0 for no timeout)
    ///           match client.get_message(5000) {
    ///               Ok(message) => {
    ///                   let payload : String = message.get_payload().unwrap();
    ///                   println!("Got message: {}", payload);
    ///               },
    ///               Err(error) => println!("Error while fetching message, should retry again, info: {}", error),
    ///           }
    /// #
    /// #         println!("Subscribed to channel.");
    /// #     },
    /// #     Err(error) => println!("Unable to create Redis client: {}", error)
    /// # }
    /// ```
    pub fn get_message(
        self: &mut Client,
        timeout: u64,
    ) -> RedisMessageResult {
        self.subscriber.get_message(&self.client, timeout)
    }
}

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
/// fn main() {
///     match simple_redis::create("redis://127.0.0.1:6379/") {
///         Ok(client) => println!("Created Redis Client"),
///         Err(error) => println!("Unable to create Redis client: {}", error)
///     }
/// }
/// ```
pub fn create(connection_string: &str) -> Result<Client, RedisError> {
    match redis::Client::open(connection_string) {
        Ok(redis_client) => {
            let redis_connection = connection::create();
            let redis_pubsub = subscriber::create();

            let client = Client { client: redis_client, connection: redis_connection, subscriber: redis_pubsub };

            Ok(client)

        }
        Err(error) => Err(RedisError { info: ErrorInfo::RedisError(error) }),
    }
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

    #[test]
    fn run_command() {
        match create("redis://127.0.0.1:6379/") {
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
    fn run_command_typed_response() {
        match create("redis://127.0.0.1:6379/") {
            Ok(mut client) => {
                assert!(!client.is_connection_open());

                let result = client.run_command_empty_response("SET", vec!["client_test1", "my_value"]);
                assert!(result.is_ok());

                assert!(client.is_connection_open());

                match client.run_command_string_response("GET", vec!["client_test1"]) {
                    Ok(value) => assert_eq!(value, "my_value"),
                    _ => panic!("test error"),
                }
            }
            _ => panic!("test error"),
        };
    }
}
