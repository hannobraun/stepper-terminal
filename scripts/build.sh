#!/usr/bin/env bash
set -e

# Fail build, if there are any warnings.
export RUSTFLAGS="-D warnings"

(
    cd cli
    cargo build) &&
(
    cd firmware
    cargo build)
