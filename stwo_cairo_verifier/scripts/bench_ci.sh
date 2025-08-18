#!/bin/bash

set -e

# Check if output file is provided as first argument
if [ $# -eq 0 ]; then
    echo "Usage: $0 <output_file>"
    echo "Example: $0 target/bench_output.json"
    exit 1
fi

OUTPUT_FILE="$1"

# Define variables
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TEST_DATA_DIR="$PROJECT_ROOT/stwo_cairo_prover/test_data"
TARGET_DIR="$PROJECT_ROOT/stwo_cairo_verifier/target"
SCARB_PROFILE="proving"

# Function to extract bytecode size from executable JSON
extract_bytecode_size() {
    jq '.program.bytecode | length' "$TARGET_DIR/$SCARB_PROFILE/stwo_cairo_verifier.executable.json"
}

echo "Running benchmark without qm31_opcode feature"
scarb --profile $SCARB_PROFILE execute \
    --package stwo_cairo_verifier \
    --features poseidon252_verifier \
    --arguments-file "$TEST_DATA_DIR/test_prove_verify_ret_opcode/proof.json" \
    --print-resource-usage \
    --output none \
    2>&1 | tee $TARGET_DIR/bench_output_0.txt

bytecode_size_0=$(extract_bytecode_size)

echo "Running benchmark with qm31_opcode feature"
scarb --profile $SCARB_PROFILE execute \
    --package stwo_cairo_verifier \
    --features qm31_opcode \
    --arguments-file "$TEST_DATA_DIR/test_prove_verify_all_opcode_components/proof.json" \
    --print-resource-usage \
    --output none \
    2>&1 | tee $TARGET_DIR/bench_output_1.txt

bytecode_size_1=$(extract_bytecode_size)

echo "Running benchmark with qm31_opcode and outputs_packing features"
scarb --profile $SCARB_PROFILE execute \
    --package stwo_cairo_verifier  \
    --features qm31_opcode \
    --features outputs_packing \
    --print-resource-usage \
    --output none \
    --arguments-file "$TEST_DATA_DIR/test_prove_verify_all_opcode_components/proof.json" \
    2>&1 | tee $TARGET_DIR/bench_output_2.txt

bytecode_size_2=$(extract_bytecode_size)

# Function to extract steps from scarb execute output
extract_steps() {
    grep -o "steps: [0-9,]*" "$1" | sed 's/steps: //' | tr -d ','
}

# Extract steps from output files
cairo_steps_0=$(extract_steps "$TARGET_DIR/bench_output_0.txt")
cairo_steps_1=$(extract_steps "$TARGET_DIR/bench_output_1.txt")
cairo_steps_2=$(extract_steps "$TARGET_DIR/bench_output_2.txt")

# Generate JSON output to file
cat << EOF > "$OUTPUT_FILE"
[
    {
        "name": "Poseidon252 verifier: Cairo steps",
        "unit": "",
        "value": $cairo_steps_0
    },
    {
        "name": "Poseidon252 verifier: bytecode size",
        "unit": "",
        "value": $bytecode_size_0
    },
    {
        "name": "QM31 opcode: Cairo steps",
        "unit": "",
        "value": $cairo_steps_1
    },
    {
        "name": "QM31 opcode: bytecode size",
        "unit": "",
        "value": $bytecode_size_1
    },
    {
        "name": "QM31 opcode + outputs packing: Cairo steps",
        "unit": "",
        "value": $cairo_steps_2
    },
    {
        "name": "QM31 opcode + outputs packing: bytecode size",
        "unit": "",
        "value": $bytecode_size_2
    }
]
EOF

# Print the contents of the output file
cat "$OUTPUT_FILE"
