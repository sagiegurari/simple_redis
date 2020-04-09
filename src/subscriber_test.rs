use super::*;

#[test]
fn create_check_state() {
    let subscriber = create();
    assert!(!subscriber.subscribed);
    assert_eq!(subscriber.subscriptions.len(), 0);
    assert_eq!(subscriber.psubscriptions.len(), 0);
    assert!(subscriber.redis_connection.is_none());
}
