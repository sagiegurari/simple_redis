
set -e

export RUST_BACKTRACE=1

#cleanups
rm -Rf ./docs/api
rm -Rf ./target/doc
rm -Rf ./target/package

cargo fmt
cargo doc --no-deps
cargo build
cargo test

#move docs
mkdir -p ./docs/api
mv ./target/doc/* ./docs/api
