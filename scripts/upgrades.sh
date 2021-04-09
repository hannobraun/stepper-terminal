#!/usr/bin/env bash
set -e

(
    cd cli
    cargo upgrades) &&
(
    cd firmware
    cargo upgrades)
