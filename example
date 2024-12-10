#!/usr/bin/env bash

set -e

export RUST_MIN_STACK=268435456

rm -f tests/cargo-run.ofvr

cargo build --example commit-blob
rust-lldb target/debug/examples/commit-blob -- $*
