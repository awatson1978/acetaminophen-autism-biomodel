#!/bin/bash

set -e

echo "🔧 Building BioModels WASM..."

if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    cargo install wasm-pack
fi

echo "Building for web target..."
wasm-pack build --target web --out-dir pkg

echo "Building for Node.js target..."
wasm-pack build --target nodejs --out-dir pkg-node

echo "Building for bundler target..."
wasm-pack build --target bundler --out-dir pkg-bundler

echo "✅ Build complete!"
echo ""
echo "To test:"
echo "  Web:    npm run example:web"
echo "  Node:   npm run example:node"