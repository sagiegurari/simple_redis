
set -e

export RUST_BACKTRACE=1

#clean old docs
rm -Rf ./docs/api
rm -Rf ./target/doc

cargo fmt
cargo doc --no-deps
cargo build
cargo test

#move docs
mkdir -p ./docs/api
mv ./target/doc/* ./docs/api
