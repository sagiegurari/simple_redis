//! # commands
//!
//! Manages the pubsub subscriber connection and if needed resubscribes in case of reconnections.
//!

#[cfg(test)]
#[path = "./subscriber_test.rs"]
mod subscriber_test;

use crate::types::{ErrorInfo, RedisEmptyResult, RedisError, RedisMessageResult};
use std::ops::Add;
use std::option::Option;
use std::time::Duration;
use std::time::SystemTime;

/// The redis pubsub wrapper.
pub(crate) struct Subscriber {
    subscribed: bool,
    subscriptions: Vec<String>,
    psubscriptions: Vec<String>,
    redis_connection: Option<redis::Connection>,
}

fn subscribe_all(subscriber: &mut Subscriber, client: &redis::Client) -> RedisEmptyResult {
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

            subscriber.subscribed = true;

            Ok(())
        }
        Err(error) => Err(RedisError {
            info: ErrorInfo::RedisError(error),
        }),
    }
}

fn get_message(subscriber: &mut Subscriber, timeout: u64) -> RedisMessageResult {
    match subscriber.redis_connection {
        Some(ref mut redis_connection) => {
            let mut redis_pubsub = redis_connection.as_pubsub();

            let duration;
            let timeout_duration;
            if timeout > 0 {
                timeout_duration = Duration::from_millis(timeout);
                duration = Some(timeout_duration);
            } else {
                timeout_duration = Duration::new(0, 0);
                duration = None;
            }

            let output;

            let mut timeout_result = redis_pubsub.set_read_timeout(duration);

            if timeout_result.is_err() {
                output = Err(RedisError {
                    info: ErrorInfo::Description("Unable to set read timeout."),
                })
            } else {
                let start = SystemTime::now();

                let message_result = redis_pubsub.get_message();

                timeout_result = redis_pubsub.set_read_timeout(None);
                if timeout_result.is_err() {
                    output = Err(RedisError {
                        info: ErrorInfo::Description("Unable to set read timeout."),
                    })
                } else {
                    output = match message_result {
                        Ok(message) => Ok(message),
                        Err(error) => {
                            let max_end = start.add(timeout_duration);
                            let mut actual_end = SystemTime::now();
                            actual_end = actual_end.add(Duration::from_millis(50)); // possible miscalculation

                            if timeout > 0 && actual_end >= max_end {
                                Err(RedisError {
                                    info: ErrorInfo::TimeoutError("Timeout"),
                                })
                            } else {
                                subscriber.subscribed = false;
                                Err(RedisError {
                                    info: ErrorInfo::RedisError(error),
                                })
                            }
                        }
                    }
                }
            }

            output
        }
        None => Err(RedisError {
            info: ErrorInfo::Description("Error while fetching pubsub."),
        }),
    }
}

fn subscribe_and_get(
    subscriber: &mut Subscriber,
    client: &redis::Client,
    timeout: u64,
) -> RedisMessageResult {
    match subscribe_all(subscriber, client) {
        Err(error) => Err(error),
        _ => match get_message(subscriber, timeout) {
            Ok(message) => Ok(message),
            Err(error) => Err(error),
        },
    }
}

fn subscribe(subscriber: &mut Subscriber, channel: &str, pattern: bool) -> RedisEmptyResult {
    if subscriber.subscribed {
        if pattern {
            subscriber.psubscriptions.push(channel.to_string());
        } else {
            subscriber.subscriptions.push(channel.to_string());
        }

        match subscriber.redis_connection {
            Some(ref mut redis_connection) => {
                let mut redis_pubsub = redis_connection.as_pubsub();

                if pattern {
                    match redis_pubsub.psubscribe(channel.to_string()) {
                        Err(error) => Err(RedisError {
                            info: ErrorInfo::RedisError(error),
                        }),
                        _ => Ok(()),
                    }
                } else {
                    match redis_pubsub.subscribe(channel.to_string()) {
                        Err(error) => Err(RedisError {
                            info: ErrorInfo::RedisError(error),
                        }),
                        _ => Ok(()),
                    }
                }
            }
            None => Err(RedisError {
                info: ErrorInfo::Description("Error while fetching pubsub."),
            }),
        }
    } else {
        if pattern {
            subscriber.psubscriptions.push(channel.to_string());
        } else {
            subscriber.subscriptions.push(channel.to_string());
        }

        Ok(())
    }
}

fn unsubscribe(subscriber: &mut Subscriber, channel: &str, pattern: bool) -> RedisEmptyResult {
    let search_result = if pattern {
        subscriber.psubscriptions.iter().position(|x| x == channel)
    } else {
        subscriber.subscriptions.iter().position(|x| x == channel)
    };

    match search_result {
        Some(index) => {
            let mut unsub_result = Ok(());

            if subscriber.subscribed {
                unsub_result = match subscriber.redis_connection {
                    Some(ref mut redis_connection) => {
                        let mut redis_pubsub = redis_connection.as_pubsub();

                        if pattern {
                            match redis_pubsub.punsubscribe(channel.to_string()) {
                                Err(error) => Err(RedisError {
                                    info: ErrorInfo::RedisError(error),
                                }),
                                _ => Ok(()),
                            }
                        } else {
                            match redis_pubsub.unsubscribe(channel.to_string()) {
                                Err(error) => Err(RedisError {
                                    info: ErrorInfo::RedisError(error),
                                }),
                                _ => Ok(()),
                            }
                        }
                    }
                    None => Err(RedisError {
                        info: ErrorInfo::Description("Error while fetching pubsub."),
                    }),
                }
            }

            if unsub_result.is_ok() {
                if pattern {
                    subscriber.psubscriptions.remove(index);
                } else {
                    subscriber.subscriptions.remove(index);
                }
            }

            unsub_result
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

    pub(crate) fn get_message(
        self: &mut Subscriber,
        client: &redis::Client,
        timeout: u64,
    ) -> RedisMessageResult {
        if self.redis_connection.is_some() {
            match get_message(self, timeout) {
                Ok(message) => Ok(message),
                Err(error) => match error.info {
                    ErrorInfo::TimeoutError(description) => Err(RedisError {
                        info: ErrorInfo::TimeoutError(description),
                    }),
                    _ => subscribe_and_get(self, client, timeout),
                },
            }
        } else {
            subscribe_and_get(self, client, timeout)
        }
    }

    pub(crate) fn unsubscribe_all(self: &mut Subscriber) -> RedisEmptyResult {
        let mut result = Ok(());

        if self.subscribed {
            let mut list = self.subscriptions.to_vec();
            for channel in &list {
                result = self.unsubscribe(channel);

                if result.is_err() {
                    break;
                }
            }

            list = self.psubscriptions.to_vec();
            for channel in &list {
                result = self.punsubscribe(channel);

                if result.is_err() {
                    break;
                }
            }
        } else {
            self.subscriptions.clear();
            self.psubscriptions.clear();
        }

        result
    }
}

/// Creates and returns a new connection
pub(crate) fn create() -> Subscriber {
    Subscriber {
        subscribed: false,
        subscriptions: vec![],
        psubscriptions: vec![],
        redis_connection: None,
    }
}
