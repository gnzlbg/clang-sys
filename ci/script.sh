#!/usr/bin/env sh

set -ex
export RUST_BACKTRACE=1

command -v llvm-config
llvm-config --prefix

FEATURES=
if [ "$CLANG_VERSION" != "" ]; then
    FEATURES="assert-minimum"
fi

cargo test -vv --features "$CLANG_VERSION $FEATURES" -- --nocapture
cargo test -vv --features "$CLANG_VERSION static $FEATURES" -- --nocapture
cargo test -vv --features "$CLANG_VERSION runtime $FEATURES" -- --nocapture
cargo test -vv --manifest-path=clang-sys-test/Cargo.toml -- --nocapture
