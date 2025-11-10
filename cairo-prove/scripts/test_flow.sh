#!/bin/bash

# Exit on any error
set -e

echo "🚀 Starting test flow..."

# Build the project
echo "📦 Building cairo-prove..."
./build.sh

# Compile the example
echo "🔨 Compiling example..."
cd example
scarb --profile release build
cd ..

# Generate proof
echo "🔍 Generating proof..."
DEBUGINFOD_URLS= gdb -ex run -ex bt -ex quit --args ./target/release/cairo-prove prove \
    example/target/release/example.executable.json \
    ./example_proof.json --arguments 100

# Verify proof
echo "✅ Verifying proof..."
./target/release/cairo-prove verify ./example_proof.json

echo "✨ Test flow completed successfully!"
