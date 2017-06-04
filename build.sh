
function kill_redis {
    ps -ef | grep redis-server | awk '{print $2}' | xargs kill -9
}

kill_redis

set -e

redis-server &
sleep 1

export RUST_BACKTRACE=1

#cleanups
rm -Rf ./docs/api
cargo clean

cargo fmt
cargo doc --no-deps
cargo build
cargo test
cargo bench

#move docs
mkdir -p ./docs/api
mv ./target/doc/* ./docs/api

#cleanups
cargo clean

kill_redis
