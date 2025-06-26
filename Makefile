# Makefile for docx-handlebars

.PHONY: all build test clean install dev help wasm wasm-web wasm-node wasm-bundler check fmt clippy docs

# Default target
all: build wasm

help: ## Show this help message
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-15s %s\n", $$1, $$2}'

# Basic builds
build: ## Build the Rust library
	@echo "📦 Building Rust library..."
	cargo build --release

test: ## Run tests
	@echo "🧪 Running tests..."
	cargo test

check: ## Check code without building
	@echo "🔍 Checking code..."
	cargo check

fmt: ## Format code
	@echo "✨ Formatting code..."
	cargo fmt

clippy: ## Run clippy linter
	@echo "📎 Running clippy..."
	cargo clippy -- -D warnings

docs: ## Generate documentation
	@echo "📚 Generating documentation..."
	cargo doc --no-deps --open

# WASM builds
wasm: wasm-web wasm-node wasm-bundler ## Build all WASM targets

wasm-web: ## Build WASM for web
	@echo "🌐 Building WASM for web..."
	wasm-pack build --target web --out-dir pkg

wasm-node: ## Build WASM for Node.js
	@echo "📦 Building WASM for Node.js..."
	wasm-pack build --target nodejs --out-dir pkg-node

wasm-bundler: ## Build WASM for bundler
	@echo "📦 Building WASM for bundler..."
	wasm-pack build --target bundler --out-dir pkg-bundler

# Development
dev: ## Development build
	@echo "🛠️  Development build..."
	cargo build

install: ## Install dependencies
	@echo "⬇️  Installing dependencies..."
	@echo "Please ensure you have:"
	@echo "  - Rust 1.70+"
	@echo "  - wasm-pack"
	@echo "  - Node.js 16+"

# Clean up
clean: ## Clean build artifacts
	@echo "🧹 Cleaning..."
	cargo clean
	rm -rf pkg pkg-node pkg-bundler
	rm -f *.docx

# Examples
run-example: ## Run basic example
	@echo "🚀 Running basic example..."
	cargo run --example basic_usage

# Publishing
publish-crates: build test ## Publish to crates.io
	@echo "📤 Publishing to crates.io..."
	cargo publish

publish-npm: wasm-web ## Publish to npm
	@echo "📤 Publishing to npm..."
	cd pkg && npm publish

publish-jsr: ## Publish to JSR
	@echo "📤 Publishing to JSR..."
	deno publish

# CI/CD
ci: check test clippy wasm ## Run all CI checks

# Platform-specific builds
build-windows: ## Build for Windows
	cargo build --release --target x86_64-pc-windows-msvc

build-linux: ## Build for Linux
	cargo build --release --target x86_64-unknown-linux-gnu

build-macos: ## Build for macOS
	cargo build --release --target x86_64-apple-darwin

# Performance
bench: ## Run benchmarks
	@echo "⚡ Running benchmarks..."
	cargo bench

profile: ## Profile the build
	@echo "📊 Profiling..."
	cargo build --release
	perf record --call-graph=dwarf target/release/docx-handlebars
	perf report
