

# Install LLD for the build to go faster as a dev
# source: https://bevyengine.org/learn/book/getting-started/setup/
# Details:

# Ubuntu: sudo apt-get install lld
# Arch: sudo pacman -S lld
# MacOS: brew install llvm

# Run is the default execution for dev work, with dynamic linking
# so it builds faster.
run:
	cargo run --features bevy/dynamic_linking

test:
	cargo test --features bevy/dynamic_linking

# Release builds do not use the dynamic linker, so the build is slow.
release: test
	cargo build --release