#!/bin/sh

cargo clean
cargo build --release

RUN_SIZE=10000000

hyperfine --warmup 3 "./target/release/naive-threads-rtrb $RUN_SIZE" "./target/release/naive-threads-std $RUN_SIZE" "./target/release/naive $RUN_SIZE"
