extern crate simple_redis;

#[test]
fn set_get() {
    match simple_redis::create("redis://127.0.0.1:6379/") {
        Ok(mut client) => {
            assert!(!client.is_connection_open());

            let result = client.set("int_set_get", "my_value");
            assert!(result.is_ok());

            assert!(client.is_connection_open());

            match client.get("int_set_get") {
                Ok(value) => assert_eq!(value, "my_value"),
                _ => panic!("test error"),
            }
        }
        _ => panic!("test error"),
    };
}
