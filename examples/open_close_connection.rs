use simple_redis;

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
