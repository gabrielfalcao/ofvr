#!/usr/bin/env bash
set -e

rm -f tests/cargo-run.ofvr

run() {
    g o -e "\033[1;38;5;220mofvr ${*}\033[0m"
    cargo run -q -- $*
}
cargo check

cat >tests/cargo-run.txt <<EOF
# ofvr commit 1 @ $(date)
EOF
run commit -m 'test' tests/cargo-run.txt

cat >tests/cargo-run.txt <<EOF
# ofvr commit 2 @ $(date)
EOF

run commit -m 'test' tests/cargo-run.txt

run log tests/cargo-run.ofvr

cat >tests/cargo-run.txt <<EOF
# ofvr commit 3 @ $(date)
EOF

run diff tests/cargo-run.txt

g o -e "\033[1;38;5;154mcargo test\033[0m"
cargo test
