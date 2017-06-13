# simple_redis

[![crates.io](https://img.shields.io/crates/v/simple_redis.svg)](https://crates.io/crates/simple_redis) [![Build Status](https://travis-ci.org/sagiegurari/simple_redis.svg)](http://travis-ci.org/sagiegurari/simple_redis) [![Build status](https://ci.appveyor.com/api/projects/status/knyrs33tyjqgt06u?svg=true)](https://ci.appveyor.com/project/sagiegurari/simple-redis) [![codecov](https://codecov.io/gh/sagiegurari/simple_redis/branch/master/graph/badge.svg)](https://codecov.io/gh/sagiegurari/simple_redis)<br>
[![license](https://img.shields.io/crates/l/simple_redis.svg)](https://github.com/sagiegurari/simple_redis/blob/master/LICENSE) [![Libraries.io for GitHub](https://img.shields.io/librariesio/github/sagiegurari/simple_redis.svg)](https://libraries.io/cargo/simple_redis) [![Documentation](https://docs.rs/simple_redis/badge.svg)](https://docs.rs/crate/simple_redis/) [![downloads](https://img.shields.io/crates/d/simple_redis.svg)](https://crates.io/crates/simple_redis)

> Simple and resilient [redis](https://redis.io/) client for [rust](https://www.rust-lang.org/).

* [Overview](#overview)
    * [Connection Resiliency](#overview-connection)
    * [Subscription Resiliency](#overview-subscription)
* [Usage](#usage)
* [Installation](#installation)
* [API Documentation](https://sagiegurari.github.io/simple_redis/)
* [Contributing](.github/CONTRIBUTING.md)
* [Release History](#history)
* [License](#license)

<a name="overview"></a>
## Overview
This library provides a very basic, simple API for the most common redis operations.<br>
While not as comprehensive or flexiable as [redis-rs](https://crates.io/crates/redis),
it does provide a simpler api for most common use cases and operations as well as automatic and resilient internal connection
and subscription (pubsub) handling.<br>
In addition, the entire API is accessible via redis client and there is no need to manage connection or pubsub instances in parallel.<br>

<a name="overview-connection"></a>
### Connection Resiliency
Connection resiliency is managed by verifying the internally managed connection before every operation against the redis server.<br>
In case of any connection issue, a new connection will be allocated to ensure the operation is invoked on a valid
connection only.<br>
However, this comes at a small performance cost of PING operation to the redis server.<br>
<br>
*In [redis-rs](https://crates.io/crates/redis), connections are no longer usable in case the connection is broken and if operations are invoked
on the client directly, it will basically open a new connection for every operation which is very costly.*

<a name="overview-subscription"></a>
### Subscription Resiliency
Subscription resiliency is ensured by recreating the internal pubsub and issuing new subscription requests
automatically in case of any error while fetching a message from the subscribed channels.<br>
<br>
*[redis-rs](https://crates.io/crates/redis) doesn't provide any such automatic resiliency and resubscription capabilities.*

<a name="usage"></a>
## Usage
In order to use this library, you need to first include the crate as follows:

````rust
extern crate simple_redis;
````

Afterwards create a redis client using a connection string:

````rust
match simple_redis::create("redis://127.0.0.1:6379/") {
    Ok(mut client) =>  println!("Created Redis Client"),
    Err(error) => println!("Unable to create Redis client: {}", error)
}
````

Once you have a redis client, you can invoke any of the available commands directly or use the run_command function to invoke operations that were not implemented by the library.

````rust
match client.set("my_key", "my_value") {
    Err(error) => println!("Unable to set value in Redis: {}", error),
    _ => println!("Value set in Redis")
};

match client.get_string("my_key") {
    Ok(value) => println!("Read value from Redis: {}", value),
    Err(error) => println!("Unable to get value from Redis: {}", error)
};

match client.set("my_numeric_key", 255.5) {
    Err(error) => println!("Unable to set value in Redis: {}", error),
    _ => println!("Value set in Redis")
};

match client.get::<f32>("my_numeric_key") {
    Ok(value) => println!("Read value from Redis: {}", value),
    Err(error) => println!("Unable to get value from Redis: {}", error)
};

match client.hgetall("my_map") {
    Ok(map) => {
        match map.get("my_field") {
            Some(value) => println!("Got field value from map: {}", value),
            None => println!("Map field is emtpy"),
        }
    },
    Err(error) => println!("Unable to read map from Redis: {}", error),
};

/// run some command that is not built in the library
match client.run_command::<String>("ECHO", vec!["testing"]) {
    Ok(value) => assert_eq!(value, "testing"),
    _ => panic!("test error"),
};

/// publish messages
let mut result = client.publish("news_channel", "test message");
assert!(result.is_ok());

/// subscribe to channels
result = client.subscribe("important_notifications");
assert!(result.is_ok());
result = client.psubscribe("*_notifications");
assert!(result.is_ok());

loop {
    // fetch next message (wait up to 5 seconds, 0 for no timeout)
    match client.get_message(5000) {
        Ok(message) => {
            let payload: String = message.get_payload().unwrap();
            assert_eq!(payload, "my important message")
        },
        Err(error) => println!("Error while fetching message, should retry again, info: {}", error),
    }
}
````

<a name="installation"></a>
## Installation
In order to use this library, just add it as a dependency:

```ini
[dependencies]
simple_redis = "*"
```

## API Documentation
See full docs at: [API Docs](https://sagiegurari.github.io/simple_redis/)

## Contributing
See [contributing guide](.github/CONTRIBUTING.md)

<a name="history"></a>
## Release History

| Date        | Version | Description |
| ----------- | ------- | ----------- |
| 2017-06-13  | v0.3.5  | Maintenance |
| 2017-06-10  | v0.3.1  | Added timeout support for get_message |
| 2017-06-08  | v0.2.8  | More commands added |
| 2017-06-03  | v0.1.7  | pubsub support added |
| 2017-06-02  | v0.1.6  | Initial release. |

<a name="license"></a>
## License
Developed by Sagie Gur-Ari and licensed under the Apache 2 open source license.