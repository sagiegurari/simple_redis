use super::*;
use std::io::Write;

#[test]
fn redis_error_description() {
    let redis_error = RedisError {
        info: ErrorInfo::Description("test"),
    };

    assert_eq!(redis_error.to_string(), "test");

    let mut writer = Vec::new();
    write!(&mut writer, "formatted {}", redis_error).unwrap();
    assert_eq!(writer, b"formatted test");
}
