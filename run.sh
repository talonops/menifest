#!/bin/bash
# cargo-run-quiet.sh - Run cargo without warnings

RUSTFLAGS="-Awarnings" cargo run "$@"