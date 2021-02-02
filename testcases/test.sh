#!/bin/bash
# run like `bash testcases/smokebin.sh`. not used in CI, but mimics it.

set -euvx

cargo run --manifest-path=testcases/smokebin/Cargo.toml
cargo build --manifest-path=testcases/dylib/Cargo.toml
cargo run --manifest-path testcases/dylib_runner/Cargo.toml -- testcases/target/debug
