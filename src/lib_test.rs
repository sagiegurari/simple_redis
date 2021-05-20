use super::*;
use doc_comment as _;

#[test]
fn create_invalid_url() {
    let result = create("test/bad/url");
    assert!(result.is_err());
}

#[test]
fn create_valid_url() {
    let mut client = create("redis://127.0.0.1:6379/").unwrap();
    assert!(!client.is_connection_open());
}
