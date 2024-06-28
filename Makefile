
CONTRACT_PATH := ./rust-program
CLIENT_PATH := ./rust-client


PROGRAM_OUT_DIR := ./contract/program


contract-build:
	cargo build-bpf --manifest-path=$(CONTRACT_PATH)/Cargo.toml --bpf-out-dir=$(PROGRAM_OUT_DIR)


rust-client-start:
	@echo "Starting Rust client..."
	rustup install nightly 
	 rustup override set nightly 
	 @cargo run --manifest-path=$(CLIENT_PATH)/Cargo.toml

rust-test:
	cargo test --manifest-path=$(CLIENT_PATH)/Cargo.toml

clean-program:
	cargo clean --manifest-path=$(CLIENT_PATH)/Cargo.toml
	rm -rf $(PROGRAM_OUT_DIR)




