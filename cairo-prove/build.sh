#!/bin/bash

# Exit on error
set -e

# Build with native CPU optimizations
RUSTFLAGS="-C target-cpu=native" cargo build --release

# Create a symlink to the binary in a directory that's in PATH
# You might need to adjust the path based on your system
echo "Build complete. Binary is available at: target/release/cairo-prove"
