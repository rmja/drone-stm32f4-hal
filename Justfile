export DRONE_RUSTFLAGS := '--cfg cortexm_core="cortexm4f_r0p1" --cfg stm32_mcu="stm32f429"'
target := 'thumbv7em-none-eabihf'
features := ''
name := `basename $(pwd)`
release_bin := "target/" + target + "/release/" + name

# Install dependencies
deps:
	rustup target add {{target}}
	rustup component add rust-src
	rustup component add rustfmt
	rustup component add clippy
	type cargo-objdump >/dev/null || cargo +stable install cargo-readme

# Reformat the source code
fmt:
	cargo fmt

# Check the source code for mistakes
lint:
	drone env {{target}} -- cargo clippy --features "{{features}}"

# Build the documentation
doc:
	drone env {{target}} -- cargo doc --features "{{features}}"

# Open the documentation in a browser
doc-open: doc
	drone env {{target}} -- cargo doc --features "{{features}}" --open

# Run the tests
test:
	drone env -- cargo test --features "std {{features}}"

# Update README.md
readme:
	cargo readme -o README.md