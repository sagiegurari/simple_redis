
function build_echo {
    echo "[Build]${1}"
}

function kill_redis {
    build_echo "[kill_redis] Stopping any running Redis"
    ps -ef | grep [r]edis-server | awk '{print $2}' | xargs kill -9
}

kill_redis

set -e

build_echo "[start_redis] Starting Redis"
redis-server --loglevel warning &
sleep 1

export RUST_BACKTRACE=1

#cleanups
build_echo "[clean] Clearning Old Build"
rm -Rf ./docs/api
cargo clean

build_echo "[fmt] Formatting Code"
cargo fmt

build_echo "[doc] Generating Docs"
cargo doc --no-deps

build_echo "[build] Start..."
cargo build

build_echo "[test] Start..."
cargo test
if [ "$1" = "full" ]; then
    build_echo "[bench] Start..."
    cargo bench

    build_echo "[verify-project] Verify Project Manifest"
    cargo verify-project

    build_echo "[audit] Check for Known Security Issues"
    cargo audit

    build_echo "[outdated] Check for Outdated Depedencies"
    cargo outdated --root-deps-only --exit-code 1
fi

#move docs
build_echo "[doc] Moving Docs"
mkdir -p ./docs/api
mv ./target/doc/* ./docs/api

#cleanups
build_echo "[clean] Clearning Current Build"
cargo clean

kill_redis
