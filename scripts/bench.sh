#!/bin/sh

cargo build --release

RUN_SIZE=10000000

hyperfine --warmup 3 "./target/release/naive-thread $RUN_SIZE" "./target/release/naive $RUN_SIZE"