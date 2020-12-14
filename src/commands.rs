//! # commands
//!
//! Defines the redis commands exposed by the redis client.
//!

#[cfg(test)]
#[path = "./commands_test.rs"]
mod commands_test;

use crate::client::Client;
use crate::types::{RedisArg, RedisBoolResult, RedisEmptyResult, RedisResult, RedisStringResult};
use std::collections::HashMap;
use std::str::FromStr;

/// Defines the redis commands exposed by the redis client.
impl Client {
    /// See redis [AUTH](https://redis.io/commands/auth) command.
    ///
    /// # Example
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
    pub fn auth(&mut self, password: &str) -> RedisEmptyResult {
        self.run_command_empty_response("AUTH", vec![password])
    }

    /// See redis [ECHO](https://redis.io/commands/echo) command.
    pub fn echo(&mut self, value: &str) -> RedisStringResult {
        self.run_command_string_response("ECHO", vec![value])
    }

    /// See redis [PUBLISH](https://redis.io/commands/publish) command.
    ///
    /// # Example
    ///
    /// ```
    /// # let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();
    /// match client.publish("important_notifications", "message text") {
    ///   Err(error) => println!("Publish error: {}", error),
    ///   _ => println!("Message published")
    /// }
    /// ```
    ///
    pub fn publish(&mut self, channel: &str, message: &str) -> RedisEmptyResult {
        self.run_command_empty_response("PUBLISH", vec![channel, message])
    }

    /// See redis [GET](https://redis.io/commands/get) command.
    ///
    /// # Example
    ///
    /// ```
    /// # let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();
    /// match client.get::<i64>("my_key") {
    ///     Ok(value) => println!("Read value from Redis: {}", value),
    ///     Err(error) => println!("Unable to get value from Redis: {}", error)
    /// }
    /// ```
    ///
    pub fn get<T: FromStr>(self: &mut Client, key: &str) -> RedisResult<T> {
        self.run_command_from_string_response("GET", vec![key])
    }

    /// See redis [GET](https://redis.io/commands/get) command.<br>
    /// This function will always return a String response.
    ///
    /// # Example
    ///
    /// ```
    /// # let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();
    /// match client.get_string("my_key") {
    ///     Ok(value) => println!("Read value from Redis: {}", value),
    ///     Err(error) => println!("Unable to get value from Redis: {}", error)
    /// }
    /// ```
    ///
    pub fn get_string(self: &mut Client, key: &str) -> RedisStringResult {
        self.run_command_string_response("GET", vec![key])
    }

    /// See redis [SET](https://redis.io/commands/set) command.
    ///
    /// # Example
    ///
    /// ```
    /// # let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();
    /// match client.set("my_key", "my_value") {
    ///     Err(error) => println!("Unable to set value in Redis: {}", error),
    ///     _ => println!("Value set in Redis")
    /// }
    /// ```
    ///
    pub fn set<T: RedisArg>(self: &mut Client, key: &str, value: T) -> RedisEmptyResult {
        self.run_command_empty_response("SET", vec![key, &value.to_string()])
    }

    /// See redis [SETEX](https://redis.io/commands/setex) command.
    ///
    /// # Example
    ///
    /// ```
    /// # let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();
    /// match client.setex("my_key", "my_value", 10) {
    ///     Err(error) => println!("Unable to set value in Redis: {}", error),
    ///     _ => println!("Value set in Redis and will expire in 10 seconds")
    /// }
    /// ```
    ///
    pub fn setex<T: RedisArg>(&mut self, key: &str, value: T, seconds: usize) -> RedisEmptyResult {
        self.run_command_empty_response(
            "SETEX",
            vec![key, &*seconds.to_string(), &value.to_string()],
        )
    }

    /// See redis [SETNX](https://redis.io/commands/setnx) command.
    pub fn setnx<T: RedisArg>(&mut self, key: &str, value: T) -> RedisEmptyResult {
        self.run_command_empty_response("SETNX", vec![key, &value.to_string()])
    }

    /// See redis [GETSET](https://redis.io/commands/getset) command.
    pub fn getset<T: RedisArg, V: FromStr>(&mut self, key: &str, value: T) -> RedisResult<V> {
        self.run_command_from_string_response::<V>("GETSET", vec![key, &value.to_string()])
    }

    /// See redis [GETSET](https://redis.io/commands/getset) command.
    pub fn getset_string<T: RedisArg>(&mut self, key: &str, value: T) -> RedisStringResult {
        self.run_command_string_response("GETSET", vec![key, &value.to_string()])
    }

    /// See redis [DEL](https://redis.io/commands/del) command.
    pub fn del(&mut self, key: &str) -> RedisEmptyResult {
        self.run_command_empty_response("DEL", vec![key])
    }

    /// See redis [EXISTS](https://redis.io/commands/exists) command.
    pub fn exists(&mut self, key: &str) -> RedisBoolResult {
        self.run_command_bool_response("EXISTS", vec![key])
    }

    /// See redis [EXPIRE](https://redis.io/commands/expire) command.
    pub fn expire(&mut self, key: &str, seconds: usize) -> RedisEmptyResult {
        self.run_command_empty_response("EXPIRE", vec![key, &*seconds.to_string()])
    }

    /// See redis [PEXPIRE](https://redis.io/commands/pexpire) command.
    pub fn pexpire(&mut self, key: &str, millies: usize) -> RedisEmptyResult {
        self.run_command_empty_response("PEXPIRE", vec![key, &*millies.to_string()])
    }

    /// See redis [PERSIST](https://redis.io/commands/persist) command.
    pub fn persist(&mut self, key: &str) -> RedisEmptyResult {
        self.run_command_empty_response("PERSIST", vec![key])
    }

    /// See redis [RENAME](https://redis.io/commands/rename) command.
    pub fn rename(&mut self, key: &str, new_key: &str) -> RedisEmptyResult {
        self.run_command_empty_response("RENAME", vec![key, new_key])
    }

    /// See redis [RENAMENX](https://redis.io/commands/renamenx) command.
    pub fn renamenx(&mut self, key: &str, new_key: &str) -> RedisEmptyResult {
        self.run_command_empty_response("RENAMENX", vec![key, new_key])
    }

    /// See redis [APPEND](https://redis.io/commands/append) command.
    pub fn append(&mut self, key: &str, value: &str) -> RedisEmptyResult {
        self.run_command_empty_response("APPEND", vec![key, value])
    }

    /// See redis [INCR](https://redis.io/commands/incr) command.
    pub fn incr(&mut self, key: &str) -> RedisResult<i64> {
        self.run_command::<i64>("INCR", vec![key])
    }

    /// See redis [INCRBY](https://redis.io/commands/incrby) command.
    pub fn incrby<T: RedisArg>(&mut self, key: &str, value: T) -> RedisResult<i64> {
        self.run_command::<i64>("INCRBY", vec![key, &*value.to_string()])
    }

    /// See redis [INCRBYFLOAT](https://redis.io/commands/incrbyfloat) command.
    pub fn incrbyfloat<T: RedisArg>(&mut self, key: &str, value: T) -> RedisResult<f64> {
        self.run_command::<f64>("INCRBYFLOAT", vec![key, &*value.to_string()])
    }

    /// See redis [STRLEN](https://redis.io/commands/strlen) command.
    pub fn strlen(&mut self, key: &str) -> RedisResult<i32> {
        self.run_command::<i32>("STRLEN", vec![key])
    }

    /// See redis [KEYS](https://redis.io/commands/keys) command.
    pub fn keys(&mut self, pattern: &str) -> RedisResult<Vec<String>> {
        self.run_command::<Vec<String>>("KEYS", vec![pattern])
    }

    /// See redis [HGET](https://redis.io/commands/hget) command.
    pub fn hget<T: FromStr>(self: &mut Client, key: &str, field: &str) -> RedisResult<T> {
        self.run_command_from_string_response("HGET", vec![key, field])
    }

    /// See redis [HGET](https://redis.io/commands/hget) command.
    pub fn hget_string(self: &mut Client, key: &str, field: &str) -> RedisStringResult {
        self.run_command_string_response("HGET", vec![key, field])
    }

    /// See redis [HGETALL](https://redis.io/commands/hgetall) command.
    ///
    /// # Example
    ///
    /// ```
    /// # let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();
    /// match client.hgetall("my_map") {
    ///     Ok(map) => {
    ///         match map.get("my_field") {
    ///             Some(value) => println!("Got field value from map: {}", value),
    ///             None => println!("Map field is emtpy"),
    ///         }
    ///     },
    ///     Err(error) => println!("Unable to read map from Redis: {}", error),
    /// }
    /// ```
    ///
    pub fn hgetall(self: &mut Client, key: &str) -> RedisResult<HashMap<String, String>> {
        self.run_command::<HashMap<String, String>>("HGETALL", vec![key])
    }

    /// See redis [HSET](https://redis.io/commands/hset) command.
    pub fn hset<T: RedisArg>(
        self: &mut Client,
        key: &str,
        field: &str,
        value: T,
    ) -> RedisEmptyResult {
        self.run_command_empty_response("HSET", vec![key, field, &value.to_string()])
    }

    /// See redis [HSETNX](https://redis.io/commands/hsetnx) command.
    pub fn hsetnx<T: RedisArg>(
        self: &mut Client,
        key: &str,
        field: &str,
        value: T,
    ) -> RedisEmptyResult {
        self.run_command_empty_response("HSETNX", vec![key, field, &value.to_string()])
    }

    /// See redis [HDEL](https://redis.io/commands/hdel) command.
    pub fn hdel(self: &mut Client, key: &str, field: &str) -> RedisEmptyResult {
        self.run_command_empty_response("HDEL", vec![key, field])
    }

    /// See redis [HEXISTS](https://redis.io/commands/hexists) command.
    pub fn hexists(self: &mut Client, key: &str, field: &str) -> RedisBoolResult {
        self.run_command_bool_response("HEXISTS", vec![key, field])
    }

    /// See redis [HKEYS](https://redis.io/commands/hkeys) command.
    pub fn hkeys(&mut self, key: &str) -> RedisResult<Vec<String>> {
        self.run_command::<Vec<String>>("HKEYS", vec![key])
    }

    /// See redis [HVALS](https://redis.io/commands/hvals) command.
    pub fn hvals(&mut self, key: &str) -> RedisResult<Vec<String>> {
        self.run_command::<Vec<String>>("HVALS", vec![key])
    }

    /// See redis [LSET](https://redis.io/commands/lset) command.
    pub fn lset<T: RedisArg>(
        self: &mut Client,
        key: &str,
        index: isize,
        value: T,
    ) -> RedisEmptyResult {
        self.run_command_empty_response("LSET", vec![key, &index.to_string(), &value.to_string()])
    }

    /// See redis [HGET](https://redis.io/commands/lindex) command.
    pub fn lindex<T: FromStr>(self: &mut Client, key: &str, index: isize) -> RedisResult<T> {
        self.run_command_from_string_response("LINDEX", vec![key, &index.to_string()])
    }

    /// See redis [HGET](https://redis.io/commands/lindex) command.
    pub fn lindex_string(self: &mut Client, key: &str, index: isize) -> RedisStringResult {
        self.run_command_string_response("LINDEX", vec![key, &index.to_string()])
    }

    /// See redis [LLEN](https://redis.io/commands/llen) command.
    pub fn llen(self: &mut Client, key: &str) -> RedisResult<i32> {
        self.run_command::<i32>("LLEN", vec![key])
    }

    /// See redis [LPOP](https://redis.io/commands/lpop) command.
    pub fn lpop<T: FromStr>(self: &mut Client, key: &str) -> RedisResult<T> {
        self.run_command_from_string_response("LPOP", vec![key])
    }

    /// See redis [LPUSH](https://redis.io/commands/lpush) command.
    pub fn lpush<T: RedisArg>(self: &mut Client, key: &str, value: T) -> RedisEmptyResult {
        self.run_command_empty_response("LPUSH", vec![key, &value.to_string()])
    }

    /// See redis [LPUSHX](https://redis.io/commands/lpushx) command.
    pub fn lpushx<T: RedisArg>(self: &mut Client, key: &str, value: T) -> RedisEmptyResult {
        self.run_command_empty_response("LPUSHX", vec![key, &value.to_string()])
    }

    /// See redis [LRANGE](https://redis.io/commands/lrange) command.
    pub fn lrange(
        self: &mut Client,
        key: &str,
        start: isize,
        stop: isize,
    ) -> RedisResult<Vec<String>> {
        self.run_command::<Vec<String>>("LRANGE", vec![key, &start.to_string(), &stop.to_string()])
    }

    /// See redis [LREM](https://redis.io/commands/lrem) command.
    pub fn lrem<T: RedisArg>(
        self: &mut Client,
        key: &str,
        count: isize,
        value: T,
    ) -> RedisEmptyResult {
        self.run_command_empty_response("LREM", vec![key, &count.to_string(), &value.to_string()])
    }

    /// See redis [LTRIM](https://redis.io/commands/ltrim) command.
    pub fn ltrim(self: &mut Client, key: &str, start: isize, stop: isize) -> RedisEmptyResult {
        self.run_command_empty_response("LTRIM", vec![key, &start.to_string(), &stop.to_string()])
    }

    /// See redis [RPOP](https://redis.io/commands/rpop) command.
    pub fn rpop<T: FromStr>(self: &mut Client, key: &str) -> RedisResult<T> {
        self.run_command_from_string_response("RPOP", vec![key])
    }

    /// See redis [RPUSH](https://redis.io/commands/rpush) command.
    pub fn rpush<T: RedisArg>(self: &mut Client, key: &str, value: T) -> RedisEmptyResult {
        self.run_command_empty_response("RPUSH", vec![key, &value.to_string()])
    }

    /// See redis [RPUSHX](https://redis.io/commands/rpushx) command.
    pub fn rpushx<T: RedisArg>(self: &mut Client, key: &str, value: T) -> RedisEmptyResult {
        self.run_command_empty_response("RPUSHX", vec![key, &value.to_string()])
    }

    /// See redis [SADD](https://redis.io/commands/sadd) command.
    pub fn sadd(self: &mut Client, key: &str, member: &str) -> RedisResult<i32> {
        self.run_command::<i32>("SADD", vec![key, member])
    }

    /// See redis [SCARD](https://redis.io/commands/scard) command.
    pub fn scard(self: &mut Client, key: &str) -> RedisResult<i32> {
        self.run_command::<i32>("SCARD", vec![key])
    }

    /// See redis [SDIFF](https://redis.io/commands/sdiff) command.
    pub fn sdiff(self: &mut Client, keys: Vec<&str>) -> RedisResult<Vec<String>> {
        self.run_command::<Vec<String>>("SDIFF", keys)
    }

    /// See redis [SISMEMBER](https://redis.io/commands/sismember) command.
    pub fn sismember(self: &mut Client, key: &str, member: &str) -> RedisBoolResult {
        self.run_command("SISMEMBER", vec![key, member])
    }

    /// See redis [SMEMBERS](https://redis.io/commands/smembers) command.
    pub fn smembers(self: &mut Client, key: &str) -> RedisResult<Vec<String>> {
        self.run_command::<Vec<String>>("SMEMBERS", vec![key])
    }

    /// See redis [SMOVE](https://redis.io/commands/smove) command.
    pub fn smove(
        self: &mut Client,
        source_key: &str,
        destination_key: &str,
        member: &str,
    ) -> RedisEmptyResult {
        self.run_command("SMOVE", vec![source_key, destination_key, member])
    }

    /// See redis [SREM](https://redis.io/commands/srem) command.
    pub fn srem(self: &mut Client, key: &str, member: &str) -> RedisEmptyResult {
        self.run_command("SREM", vec![key, member])
    }

    /// See redis [ZADD](https://redis.io/commands/zadd) command.
    pub fn zadd(self: &mut Client, key: &str, score: isize, member: &str) -> RedisResult<i32> {
        self.run_command("ZADD", vec![key, &score.to_string(), member])
    }

    /// See redis [ZRANGE](https://redis.io/commands/zrange) command.
    pub fn zrange(
        self: &mut Client,
        key: &str,
        start: isize,
        stop: isize,
    ) -> RedisResult<Vec<String>> {
        self.run_command::<Vec<String>>("ZRANGE", vec![key, &start.to_string(), &stop.to_string()])
    }
}
