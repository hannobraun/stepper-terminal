#!/usr/bin/env bash
set -e

(
    cd cli
    cargo update) &&
(
    cd firmware
    cargo update)
