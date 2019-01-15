bootimage:
	bootimage build

build:
	cargo xbuild

qemu:
	bootimage run

clean:
	cargo clean

deps:
	rustup toolchain install nightly
	rustup default nightly
	rustup component add rls rust-analysis rust-src
	cargo install --force racer cargo-xbuild
	cargo install bootimage --version "^0.5.0"

