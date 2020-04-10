//! # commands
//!
//! Manages the pubsub subscriber connection and if needed resubscribes in case of reconnections.
//!

#[cfg(test)]
#[path = "./subscriber_test.rs"]
mod subscriber_test;

use crate::types::{ErrorInfo, Interrupts, Message, RedisEmptyResult, RedisError};
use std::option::Option;
use std::time::Duration;

/// The redis pubsub wrapper.
pub(crate) struct Subscriber {
    subscriptions: Vec<String>,
    psubscriptions: Vec<String>,
    redis_connection: Option<redis::Connection>,
}

fn subscribe_all<'a>(
    subscriber: &'a mut Subscriber,
    client: &redis::Client,
) -> Result<redis::PubSub<'a>, RedisError> {
    // get pubsub
    match client.get_connection() {
        Ok(redis_connection) => {
            let redis_connection_ref = subscriber.redis_connection.get_or_insert(redis_connection);
            let mut redis_pubsub = redis_connection_ref.as_pubsub();

            for channel in &subscriber.subscriptions {
                let result = redis_pubsub.subscribe(channel);

                if result.is_err() {
                    let subscription_error = match result.err() {
                        Some(error) => Err(RedisError {
                            info: ErrorInfo::RedisError(error),
                        }),
                        None => Err(RedisError {
                            info: ErrorInfo::Description("Error while subscribing."),
                        }),
                    };

                    return subscription_error;
                }
            }

            for channel in &subscriber.psubscriptions {
                let result = redis_pubsub.psubscribe(channel);

                if result.is_err() {
                    let subscription_error = match result.err() {
                        Some(error) => Err(RedisError {
                            info: ErrorInfo::RedisError(error),
                        }),
                        None => Err(RedisError {
                            info: ErrorInfo::Description("Error while subscribing."),
                        }),
                    };

                    return subscription_error;
                }
            }

            Ok(redis_pubsub)
        }
        Err(error) => Err(RedisError {
            info: ErrorInfo::RedisError(error),
        }),
    }
}

fn fetch_messages(
    mut redis_pubsub: redis::PubSub,
    on_message: &mut dyn FnMut(Message) -> bool,
    poll_interrupts: &mut dyn FnMut() -> Interrupts,
) -> RedisEmptyResult {
    loop {
        let interrupts = poll_interrupts();
        if interrupts.stop {
            return Ok(());
        } else {
            let duration_millis = interrupts.next_polling_time.unwrap_or(5000);

            let read_timeout = if duration_millis == 0 {
                None
            } else {
                Some(Duration::from_millis(duration_millis))
            };
            if let Err(error) = redis_pubsub.set_read_timeout(read_timeout) {
                return Err(RedisError {
                    info: ErrorInfo::RedisError(error),
                });
            };

            let message_result = redis_pubsub.get_message();

            match message_result {
                Ok(message) => {
                    if on_message(message) {
                        return Ok(());
                    }
                }
                Err(error) => {
                    if !error.is_timeout() {
                        return Err(RedisError {
                            info: ErrorInfo::RedisError(error),
                        });
                    }
                }
            }
        }
    }
}

fn subscribe_and_fetch(
    subscriber: &mut Subscriber,
    client: &redis::Client,
    on_message: &mut dyn FnMut(Message) -> bool,
    poll_interrupts: &mut dyn FnMut() -> Interrupts,
) -> RedisEmptyResult {
    match subscribe_all(subscriber, client) {
        Err(error) => Err(error),
        Ok(pubsub) => fetch_messages(pubsub, on_message, poll_interrupts),
    }
}

fn subscribe(subscriber: &mut Subscriber, channel: &str, pattern: bool) -> RedisEmptyResult {
    if pattern {
        subscriber.psubscriptions.push(channel.to_string());
    } else {
        subscriber.subscriptions.push(channel.to_string());
    }

    Ok(())
}

fn unsubscribe(subscriber: &mut Subscriber, channel: &str, pattern: bool) -> RedisEmptyResult {
    let search_result = if pattern {
        subscriber.psubscriptions.iter().position(|x| x == channel)
    } else {
        subscriber.subscriptions.iter().position(|x| x == channel)
    };

    match search_result {
        Some(index) => {
            if pattern {
                subscriber.psubscriptions.remove(index);
            } else {
                subscriber.subscriptions.remove(index);
            }

            Ok(())
        }
        None => Ok(()),
    }
}

impl Subscriber {
    pub(crate) fn subscribe(self: &mut Subscriber, channel: &str) -> RedisEmptyResult {
        subscribe(self, channel, false)
    }

    pub(crate) fn psubscribe(self: &mut Subscriber, channel: &str) -> RedisEmptyResult {
        subscribe(self, channel, true)
    }

    pub(crate) fn unsubscribe(self: &mut Subscriber, channel: &str) -> RedisEmptyResult {
        unsubscribe(self, channel, false)
    }

    pub(crate) fn punsubscribe(self: &mut Subscriber, channel: &str) -> RedisEmptyResult {
        unsubscribe(self, channel, true)
    }

    pub(crate) fn is_subscribed(self: &mut Subscriber, channel: &str) -> bool {
        let search_result = self.subscriptions.iter().position(|x| x == channel);

        match search_result {
            None => false,
            _ => true,
        }
    }

    pub(crate) fn is_psubscribed(self: &mut Subscriber, channel: &str) -> bool {
        let search_result = self.psubscriptions.iter().position(|x| x == channel);

        match search_result {
            None => false,
            _ => true,
        }
    }

    pub(crate) fn unsubscribe_all(self: &mut Subscriber) -> RedisEmptyResult {
        self.subscriptions.clear();
        self.psubscriptions.clear();

        Ok(())
    }

    fn has_subscriptions(self: &Subscriber) -> bool {
        !self.subscriptions.is_empty() || !self.psubscriptions.is_empty()
    }

    pub(crate) fn fetch_messages(
        self: &mut Subscriber,
        client: &redis::Client,
        on_message: &mut dyn FnMut(Message) -> bool,
        poll_interrupts: &mut dyn FnMut() -> Interrupts,
    ) -> RedisEmptyResult {
        if !self.has_subscriptions() {
            Err(RedisError {
                info: ErrorInfo::Description("No subscriptions defined."),
            })
        } else {
            subscribe_and_fetch(self, client, on_message, poll_interrupts)
        }
    }
}

/// Creates and returns a new connection
pub(crate) fn create() -> Subscriber {
    Subscriber {
        subscriptions: vec![],
        psubscriptions: vec![],
        redis_connection: None,
    }
}
