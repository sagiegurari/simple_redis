#![feature(test)]
extern crate simple_redis;
extern crate test;

use test::Bencher;

#[bench]
fn set_get_del(bencher: &mut Bencher) {
    let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();

    client.set("bnch_set_get", "my_value").unwrap();
    client.get_string("bnch_set_get").unwrap();
    client.del("bnch_set_get").unwrap();

    assert!(client.is_connection_open());

    bencher.iter(|| {
        client.set("bnch_set_get", "my_value").unwrap();
        client.get_string("bnch_set_get").unwrap();
        client.del("bnch_set_get").unwrap();
    });
}
