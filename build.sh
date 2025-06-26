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
echo "✅ All builds completed successfully!"
echo ""
echo "📁 Output directories:"
echo "  - Rust: target/release/"
echo "  - Web/Browser: pkg/ (supports both web and bundlers)"
echo "  - Node.js: pkg-node/"
echo ""
echo "🎉 Ready for publishing!"
