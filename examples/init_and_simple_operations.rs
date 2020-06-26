use simple_redis;

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
