use super::*;

#[test]
fn create_check_state() {
    let connection = create();
    assert!(!connection.is_connection_open());
}
