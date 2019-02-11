#!/usr/bin/env bash

set -ex

FEATURES=
if [ "$CLANG_VERSION" != "" ]; then
    FEATURES="assert-minimum"
fi

RUST_BACKTRACE=1 cargo test -vv --features "$CLANG_VERSION $FEATURES" -- --nocapture
RUST_BACKTRACE=1 cargo test -vv --features "$CLANG_VERSION static $FEATURES" -- --nocapture
RUST_BACKTRACE=1 cargo test -vv --features "$CLANG_VERSION runtime $FEATURES" -- --nocapture

RUST_BACKTRACE=1 cargo test --manifest-path=clang-sys-test/Cargo.toml -vv -- --nocapture
