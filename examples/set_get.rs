use simple_redis;
use std::{thread, time};

fn main() {
    match simple_redis::create("redis://127.0.0.1:6379/") {
        Ok(mut client) => {
            println!("Created Redis Client");

            match client.set("my_key", "my_value") {
                Err(error) => println!("Unable to set value in Redis: {}", error),
                _ => println!("Value set in Redis"),
            }

            // get the value as string
            match client.get_string("my_key") {
                Ok(value) => println!("Read value from Redis: {}", value),
                Err(error) => println!("Unable to get value from Redis: {}", error),
            }

            println!("going to sleep, you can restart redis to test connection resiliency...");
            thread::sleep(time::Duration::from_secs(10));
            println!("back...");

            match client.set("my_key", 500) {
                Err(error) => println!("Unable to set value in Redis: {}", error),
                _ => println!("Value set in Redis"),
            }

            // get the value as i64
            match client.get::<i64>("my_key") {
                Ok(value) => println!("Read value from Redis: {}", value),
                Err(error) => println!("Unable to get value from Redis: {}", error),
            }
        }
        Err(error) => println!("Unable to create Redis client: {}", error),
    }
}
