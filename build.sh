#!/bin/bash

echo "🚀 Building docx-handlebars for all platforms..."

echo ""
echo "📦 Building Rust library..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "❌ Rust build failed!"
    exit 1
fi

echo ""
echo "🌐 Building WASM for web..."
wasm-pack build --target web --out-dir pkg
if [ $? -ne 0 ]; then
    echo "❌ WASM web build failed!"
    exit 1
fi

echo ""
echo "📦 Building WASM for Node.js..."
wasm-pack build --target nodejs --out-dir pkg-node
if [ $? -ne 0 ]; then
    echo "❌ WASM Node.js build failed!"
    exit 1
fi

echo ""
echo "📦 Building WASM for bundler..."
wasm-pack build --target bundler --out-dir pkg-bundler
if [ $? -ne 0 ]; then
    echo "❌ WASM bundler build failed!"
    exit 1
fi

echo ""
echo "✅ All builds completed successfully!"
echo ""
echo "📁 Output directories:"
echo "  - Rust: target/release/"
echo "  - Web: pkg/"
echo "  - Node.js: pkg-node/"
echo "  - Bundler: pkg-bundler/"
echo ""
echo "🎉 Ready for publishing!"
