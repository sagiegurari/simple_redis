use simple_redis;
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
