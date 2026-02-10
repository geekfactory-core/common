#!/bin/bash
set -euo pipefail

. ./bin/utils.sh

header "Generate candid"

cargo run --manifest-path candid/Cargo.toml > candid/can.did
