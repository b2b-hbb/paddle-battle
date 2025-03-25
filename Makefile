.PHONY: test web-build stylus-check website-install website-start clean test-stylus

# Run cargo tests
test:
	cargo test --lib

# Run stylus integration tests
integration:
	cd stylus-test && cargo test

# Build web assets
web:
	rm -rf ./website/src/pkg
	wasm-pack build --out-dir ./website/src/pkg/ --target web --weak-refs --debug -- --no-default-features --features web

# Check stylus code
stylus:
	cargo stylus check

# Install website dependencies
website-install:
	cd website && yarn install

# Start website development server
website-start:
	cd website && yarn start

# Clean build artifacts
clean:
	cargo clean
	rm -rf ./website/src/pkg
	rm -rf ./website/node_modules
	rm -rf ./website/build


all: test web-build stylus-check website-install

dev: web website-install website-start 
