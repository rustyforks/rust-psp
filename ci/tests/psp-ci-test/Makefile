CWD = $(shell pwd)
CARGO_TARGET_DIR ?= ./target

# Change this to the cargo package name
TARGET_NAME = psp-ci-test

.PHONY: release
release:
	@RUST_TARGET_PATH="$(CWD)" RUSTFLAGS="-C link-dead-code" \
		xargo build --target psp --release

	@psp-prxgen \
		"$(CARGO_TARGET_DIR)"/psp/release/$(TARGET_NAME) \
		"$(CARGO_TARGET_DIR)"/psp/release/$(TARGET_NAME).prx

	@mksfo "PSP Rust CI Test" "$(CARGO_TARGET_DIR)"/psp/release/PARAM.SFO
	@pack-pbp "$(CARGO_TARGET_DIR)"/psp/release/EBOOT.PBP \
		"$(CARGO_TARGET_DIR)"/psp/release/PARAM.SFO NULL NULL NULL NULL NULL \
		"$(CARGO_TARGET_DIR)"/psp/release/$(TARGET_NAME).prx NULL

	@echo Saved to "$(CARGO_TARGET_DIR)"/psp/release/EBOOT.PBP
