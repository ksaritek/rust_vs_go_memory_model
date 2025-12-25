.PHONY: run build clean check fmt clippy help examples refcell-panic

# Run the playground
run:
	@echo "==> Running Rust Playground..."
	cargo run

# Build the binary
build:
	@echo "==> Building binary..."
	cargo build --release

# Clean build artifacts
clean:
	@echo "==> Cleaning..."
	cargo clean

# Run refcell_panic example
refcell-panic:
	@echo "==> Running RefCell panic example..."
	@echo ""
	cargo run --example refcell_panic
