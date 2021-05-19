#!/usr/bin/env bash
set -e

(
    cd cli
    cargo clean) &&
(
    cd firmware
    cargo clean)
