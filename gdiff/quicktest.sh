#!/usr/bin/env bash
set -e
set -x

g o foo | gzip > foo
g o bar | gzip > bar

cargo run -- ./foo ./bar
