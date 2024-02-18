.PHONY: bench profile build

build:
	cargo build --release


RUN_SIZE := 10000000

ALL_TARGETS := multijoin-nthread-rtrb-dashmap naive-1thread-none-dashmap naive-1thread-none-hashbrown naive-nthread-rtrb-dashmap naive-nthread-rtrb-hashbrown naive-nthread-std-dashmap naive-nthread-std-hashbrown 

TARGET_BASE_PATH := ./target/release/
BENCH_COMMAND := $(foreach wrd,$(ALL_TARGETS),"$(TARGET_BASE_PATH)$(wrd) $(RUN_SIZE)")

bench: build
	hyperfine --warmup 3 $(BENCH_COMMAND)


bin?=naive
profile:
	sudo CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph -o="$(bin)-flamegraph.svg" --bin=$(bin) -- $(RUN_SIZE)
