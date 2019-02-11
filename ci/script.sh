#!/usr/bin/env bash
if [ "${TRAVIS_OS_NAME}" == "osx" ]; then
    rvm get head || true
fi

set -ex

RUST_BACKTRACE=1 cargo test -vv --features "$CLANG_VERSION assert-minimum" -- --nocapture
if [ "${CLANG_VERSION}" \< "clang_3_7" ]; then
    RUST_BACKTRACE=1 cargo test -vv --features "$CLANG_VERSION static" -- --nocapture
fi
RUST_BACKTRACE=1 cargo test -vv --features "$CLANG_VERSION assert-minimum runtime" -- --nocapture

RUST_BACKTRACE=1 cargo test --manifest-path=clang-sys-test/Cargo.toml -vv -- --nocapture
