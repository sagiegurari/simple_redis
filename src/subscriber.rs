//! # commands
//!
//! Manages the pubsub subscriber connection and if needed resubscribes in case of reconnections.
//!

extern crate redis;
use std::option::Option;
use types::{ErrorInfo, RedisEmptyResult, RedisError, RedisMessageResult};

/// The redis pubsub wrapper.
pub struct Subscriber {
    subscribed: bool,
    subscriptions: Vec<String>,
    psubscriptions: Vec<String>,
    pubsub: Option<redis::PubSub>
}

fn subscribe_all(
    subscriber: &mut Subscriber,
    client: &redis::Client,
) -> RedisEmptyResult {
    // get pubsub
    match client.get_pubsub() {
        Ok(mut redis_pubsub) => {
            for channel in &subscriber.subscriptions {
                let result = redis_pubsub.subscribe(channel);

                if result.is_err() {
                    let subscription_error = match result.err() {
                        Some(error) => Err(RedisError { info: ErrorInfo::RedisError(error) }),
                        None => Err(RedisError { info: ErrorInfo::Description("Error while subscribing.") }),
                    };

                    return subscription_error;
                }
            }

            for channel in &subscriber.psubscriptions {
                let result = redis_pubsub.psubscribe(channel);

                if result.is_err() {
                    let subscription_error = match result.err() {
                        Some(error) => Err(RedisError { info: ErrorInfo::RedisError(error) }),
                        None => Err(RedisError { info: ErrorInfo::Description("Error while subscribing.") }),
                    };

                    return subscription_error;
                }
            }

            subscriber.subscribed = true;
            subscriber.pubsub = Some(redis_pubsub);

            Ok(())
        }
        Err(error) => Err(RedisError { info: ErrorInfo::RedisError(error) }),
    }
}


fn get_message(subscriber: &mut Subscriber) -> RedisMessageResult {
    match subscriber.pubsub {
        Some(ref redis_pubsub) => {
            match redis_pubsub.get_message() {
                Ok(message) => Ok(message),
                Err(error) => {
                    subscriber.subscribed = false;
                    Err(RedisError { info: ErrorInfo::RedisError(error) })
                }
            }
        }
        None => Err(RedisError { info: ErrorInfo::Description("Error while fetching pubsub.") }),
    }
}

fn subscribe_and_get(
    subscriber: &mut Subscriber,
    client: &redis::Client,
) -> RedisMessageResult {
    match subscribe_all(subscriber, client) {
        Err(error) => Err(error),
        _ => {
            match get_message(subscriber) {
                Ok(message) => Ok(message),
                Err(error) => Err(error),
            }
        }
    }
}

fn subscribe(
    subscriber: &mut Subscriber,
    channel: &str,
    pattern: bool,
) -> RedisEmptyResult {
    if subscriber.subscribed {
        if pattern {
            subscriber.psubscriptions.push(channel.to_string());
        } else {
            subscriber.subscriptions.push(channel.to_string());
        }

        match subscriber.pubsub {
            Some(ref mut redis_pubsub) => {
                if pattern {
                    match redis_pubsub.psubscribe(channel.to_string()) {
                        Err(error) => Err(RedisError { info: ErrorInfo::RedisError(error) }),
                        _ => Ok(()),
                    }
                } else {
                    match redis_pubsub.subscribe(channel.to_string()) {
                        Err(error) => Err(RedisError { info: ErrorInfo::RedisError(error) }),
                        _ => Ok(()),
                    }
                }
            }
            None => Err(RedisError { info: ErrorInfo::Description("Error while fetching pubsub.") }),
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

fn unsubscribe(
    subscriber: &mut Subscriber,
    channel: &str,
    pattern: bool,
) -> RedisEmptyResult {
    let search_result;
    if pattern {
        search_result = subscriber.psubscriptions.iter().position(|x| *x == channel.to_string());
    } else {
        search_result = subscriber.subscriptions.iter().position(|x| *x == channel.to_string());
    }

    match search_result {
        Some(index) => {
            let mut unsub_result = Ok(());

            if subscriber.subscribed {
                unsub_result = match subscriber.pubsub {
                    Some(ref mut redis_pubsub) => {
                        if pattern {
                            match redis_pubsub.punsubscribe(channel.to_string()) {
                                Err(error) => Err(RedisError { info: ErrorInfo::RedisError(error) }),
                                _ => Ok(()),
                            }
                        } else {
                            match redis_pubsub.unsubscribe(channel.to_string()) {
                                Err(error) => Err(RedisError { info: ErrorInfo::RedisError(error) }),
                                _ => Ok(()),
                            }
                        }
                    }
                    None => Err(RedisError { info: ErrorInfo::Description("Error while fetching pubsub.") }),
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
    pub fn subscribe(
        self: &mut Subscriber,
        channel: &str,
    ) -> RedisEmptyResult {
        subscribe(self, channel, false)
    }

    pub fn psubscribe(
        self: &mut Subscriber,
        channel: &str,
    ) -> RedisEmptyResult {
        subscribe(self, channel, true)
    }

    pub fn unsubscribe(
        self: &mut Subscriber,
        channel: &str,
    ) -> RedisEmptyResult {
        unsubscribe(self, channel, false)
    }

    pub fn punsubscribe(
        self: &mut Subscriber,
        channel: &str,
    ) -> RedisEmptyResult {
        unsubscribe(self, channel, true)
    }

    pub fn get_message(
        self: &mut Subscriber,
        client: &redis::Client,
    ) -> RedisMessageResult {
        if self.pubsub.is_some() {
            match get_message(self) {
                Ok(message) => Ok(message),
                _ => subscribe_and_get(self, client),
            }
        } else {
            subscribe_and_get(self, client)
        }

    }
}

/// Creates and returns a new connection
pub fn create() -> Subscriber {
    Subscriber { subscribed: false, subscriptions: vec![], psubscriptions: vec![], pubsub: None }
}
