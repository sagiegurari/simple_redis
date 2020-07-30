# simple_redis

[![crates.io](https://img.shields.io/crates/v/simple_redis.svg)](https://crates.io/crates/simple_redis) [![CI](https://github.com/sagiegurari/simple_redis/workflows/CI/badge.svg?branch=master)](https://github.com/sagiegurari/simple_redis/actions) [![codecov](https://codecov.io/gh/sagiegurari/simple_redis/branch/master/graph/badge.svg)](https://codecov.io/gh/sagiegurari/simple_redis)<br>
[![license](https://img.shields.io/crates/l/simple_redis.svg)](https://github.com/sagiegurari/simple_redis/blob/master/LICENSE) [![Libraries.io for GitHub](https://img.shields.io/librariesio/github/sagiegurari/simple_redis.svg)](https://libraries.io/cargo/simple_redis) [![Documentation](https://docs.rs/simple_redis/badge.svg)](https://docs.rs/crate/simple_redis/) [![downloads](https://img.shields.io/crates/d/simple_redis.svg)](https://crates.io/crates/simple_redis)<br>
[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)

> Simple and resilient [redis](https://redis.io/) client for [rust](https://www.rust-lang.org/).

* [Overview](#overview)
    * [Connection Resiliency](#overview-connection)
    * [Subscription Resiliency](#overview-subscription)
* [Usage](#usage)
* [Installation](#installation)
* [API Documentation](https://sagiegurari.github.io/simple_redis/)
* [Contributing](.github/CONTRIBUTING.md)
* [Release History](CHANGELOG.md)
* [License](#license)

<a name="overview"></a>
## Overview
This library provides a very basic, simple API for the most common redis operations.<br>
While not as comprehensive or flexible as [redis-rs](https://crates.io/crates/redis),
it does provide a simpler api for most common use cases and operations as well as automatic and resilient internal connection
and subscription (pubsub) handling.<br>
In addition, the entire API is accessible via simple client interface and there is no need to manage multiple entities such as connection or pubsub in parallel.<br>

<a name="overview-connection"></a>
### Connection Resiliency
Connection resiliency is managed by verifying the internally managed connection before every operation against the redis server.<br>
In case of any connection issue, a new connection will be allocated to ensure the operation is invoked on a valid
connection only.<br>
However, this comes at a small performance cost of PING operation to the redis server.<br>
<br>
//!
*In [redis-rs](https://crates.io/crates/redis), connections are no longer usable in case the connection is broken and if operations are invoked
on the client directly, it will basically open a new connection for every operation which is very costly.*

<a name="overview-subscription"></a>
### Subscription Resiliency
Subscription resiliency is ensured by recreating the internal pubsub and issuing new subscription requests
automatically in case of any error while fetching a message from the subscribed channels.<br>
<br>
*[redis-rs](https://crates.io/crates/redis) doesn't provide any such automatic resiliency and re-subscription capabilities.*


<a name="usage"></a>
## Usage

### Initialization and Simple Operations

<!--{ "examples/init_and_simple_operations.rs" | lines: 3 | code: rust }-->
```rust
fn main() {
    match simple_redis::create("redis://127.0.0.1:6379/") {
        Ok(mut client) => {
            println!("Created Redis Client");

            match client.set("my_key", "my_value") {
                Err(error) => println!("Unable to set value in Redis: {}", error),
                _ => println!("Value set in Redis"),
            };

            match client.get_string("my_key") {
                Ok(value) => println!("Read value from Redis: {}", value),
                Err(error) => println!("Unable to get value from Redis: {}", error),
            };

            match client.set("my_numeric_key", 255.5) {
                Err(error) => println!("Unable to set value in Redis: {}", error),
                _ => println!("Value set in Redis"),
            };

            match client.get::<f32>("my_numeric_key") {
                Ok(value) => println!("Read value from Redis: {}", value),
                Err(error) => println!("Unable to get value from Redis: {}", error),
            };

            match client.hgetall("my_map") {
                Ok(map) => match map.get("my_field") {
                    Some(value) => println!("Got field value from map: {}", value),
                    None => println!("Map field is empty"),
                },
                Err(error) => println!("Unable to read map from Redis: {}", error),
            };

            // run some command that is not built in the library
            match client.run_command::<String>("ECHO", vec!["testing"]) {
                Ok(value) => assert_eq!(value, "testing"),
                _ => panic!("test error"),
            };

            // publish messages
            let result = client.publish("news_channel", "test message");
            assert!(result.is_ok());
        }
        Err(error) => println!("Unable to create Redis client: {}", error),
    }
}
```
<!--{ end }-->

### Subscription Flow

<!--{ "examples/subscription_flow.rs" | lines: 2 | code: rust }-->
```rust
use simple_redis::{Interrupts, Message};

fn main() {
    match simple_redis::create("redis://127.0.0.1:6379/") {
        Ok(mut client) => {
            println!("Created Redis Client");

            let mut result = client.subscribe("important_notifications");
            assert!(result.is_ok());
            result = client.psubscribe("*_notifications");
            assert!(result.is_ok());

            // fetch messages from all subscriptions
            let mut polling_counter: usize = 0;
            client
                .fetch_messages(
                    &mut |message: Message| -> bool {
                        let payload: String = message.get_payload().unwrap();
                        println!("Got message: {}", payload);

                        // continue fetching
                        false
                    },
                    // interrupts enable you to break the fetching blocking call
                    &mut || -> Interrupts {
                        let mut interrupts = Interrupts::new();
                        interrupts.next_polling_time = Some(150);

                        polling_counter = polling_counter + 1;
                        if polling_counter > 3 {
                            interrupts.stop = true;
                        }

                        interrupts
                    },
                )
                .unwrap();
        }
        Err(error) => println!("Unable to create Redis client: {}", error),
    }
}
```
<!--{ end }-->

### Closing Connection

<!--{ "examples/open_close_connection.rs" | lines: 3 | code: rust }-->
```rust
fn main() {
    match simple_redis::create("redis://127.0.0.1:6379/") {
        Ok(mut client) => {
            println!("Created Redis Client");

            match client.set("my_key", "my_value") {
                Err(error) => println!("Unable to set value in Redis: {}", error),
                _ => println!("Value set in Redis"),
            };

            match client.quit() {
                Err(error) => println!("Error: {}", error),
                _ => println!("Connection Closed."),
            }
        }
        Err(error) => println!("Unable to create Redis client: {}", error),
    }
}
```
<!--{ end }-->

<a name="installation"></a>
## Installation
In order to use this library, just add it as a dependency:

```ini
[dependencies]
simple_redis = "^0.5.2"
```

## API Documentation
See full docs at: [API Docs](https://sagiegurari.github.io/simple_redis/)

## Contributing
See [contributing guide](.github/CONTRIBUTING.md)

<a name="history"></a>
## Release History

See [Changelog](CHANGELOG.md)

<a name="license"></a>
## License
Developed by Sagie Gur-Ari and licensed under the Apache 2 open source license.
