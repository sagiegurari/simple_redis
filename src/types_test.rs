use super::*;
use std::error::Error;
use std::io::Write;

#[test]
fn redis_error_description() {
    let redis_error = RedisError {
        info: ErrorInfo::Description("test"),
    };

    assert_eq!(redis_error.description(), "test");
    assert!(redis_error.cause().is_none());

    let mut writer = Vec::new();
    write!(&mut writer, "formatted {}", redis_error).unwrap();
    assert_eq!(writer, b"formatted test");
}

#[test]
fn redis_error_timeout_error() {
    let redis_error = RedisError {
        info: ErrorInfo::TimeoutError("timeout"),
    };

    assert_eq!(redis_error.description(), "timeout");
    assert!(redis_error.cause().is_none());

    let mut writer = Vec::new();
    write!(&mut writer, "formatted {}", redis_error).unwrap();
    assert_eq!(writer, b"formatted timeout");
}
