#!/bin/bash

# Exit on any error
set -e

# Define packages
PACKAGES="stwo_verifier_core stwo_cairo_air stwo_cairo_verifier stwo_verifier_utils"

echo "🚀 Running all tests..."

# Format check
echo "📝 Checking formatting..."
scarb fmt --check

# Run tests for poseidon252_verifier feature
echo "🧪 Running poseidon252_verifier tests..."
for package in $PACKAGES; do
    scarb test --features=poseidon252_verifier --package $package
done

# Run proof verification for poseidon252
echo "✨ Running poseidon252 proof verification..."
scarb --profile proving execute --package stwo_cairo_verifier \
    --features poseidon252_verifier --print-resource-usage --output none \
    --arguments-file ../stwo_cairo_prover/test_data/test_prove_verify_ret_opcode/proof.json

# Run tests for qm31_opcode feature
echo "🧪 Running qm31_opcode tests..."
for package in $PACKAGES; do
    scarb test --features=qm31_opcode --package $package
done

# Run proof verification for qm31
echo "✨ Running qm31 proof verification..."
scarb --profile proving execute --package stwo_cairo_verifier \
    --features qm31_opcode --print-resource-usage --output none \
    --arguments-file ../stwo_cairo_prover/test_data/test_prove_verify_all_opcode_components/proof.json

echo "✅ All tests completed successfully!"
