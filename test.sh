#!/usr/bin/env bash
set -e

set -x
cargo test
set +x

rm -f tests/cargo-run.ofvr

cat >tests/cargo-run.txt <<EOF
# ofvr commit 1 @ $(date)
EOF
set -x
cargo run -q -- commit -m 'test' tests/cargo-run.txt
set +x

cat >tests/cargo-run.txt <<EOF
# ofvr commit 2 @ $(date)
EOF
set -x
cargo run -q -- commit -m 'test' tests/cargo-run.txt
set +x
