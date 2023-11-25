.PHONY: bench profile build

build:
	cargo build --release


RUN_SIZE := 10000000

#ALL_TARGETS := naive-dash-hash naive-std-hash naive-threads-std naive-threads-rtrb thread-multi-join
ALL_TARGETS := naive-std-hash thread-std-hash-std-channel

TARGET_BASE_PATH := ./target/release/
BENCH_COMMAND := $(foreach wrd,$(ALL_TARGETS),"$(TARGET_BASE_PATH)$(wrd) $(RUN_SIZE)")

bench: build
	hyperfine --warmup 3 $(BENCH_COMMAND)


bin?=naive
profile:
	sudo CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph -o="$(bin)-flamegraph.svg" --bin=$(bin) -- $(RUN_SIZE)
