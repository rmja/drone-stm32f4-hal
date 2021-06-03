features := 'dma exti fmc gpio rcc tim uart spi'
target := `drone print target 2>/dev/null || echo ""`

# Install dependencies
deps:
	type cargo-readme >/dev/null || cargo +stable install cargo-readme
	type drone >/dev/null || cargo install drone
	rustup target add {{target}}

# Reformat the source code
fmt:
	cargo fmt

# Check the source code for mistakes
lint:
	cargo clippy --features "{{features}}"

# Build the documentation
doc:
	cargo doc --features "{{features}}"

# Open the documentation in a browser
doc-open: doc
	cargo doc --features "{{features}}" --open

# Run the tests
test:
	cargo test --features "std {{features}}" \
		--target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')

# Update README.md
readme:
	cargo readme -o README.md