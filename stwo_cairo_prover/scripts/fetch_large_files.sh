#!/bin/bash

set -e
set -o pipefail

# Ensure the input file is provided
if [[ $# -ne 1 ]]; then
    echo "Usage: $0 <input_json>"
    exit 1
fi

# Check if jq is installed
if ! command -v jq &> /dev/null; then
    echo "Error: 'jq' is not installed. Run: sudo apt-get install js."
    exit 1
fi

INPUT_FILE="$1"
BUCKET_URL="https://storage.googleapis.com/stwo-cairo-testing-artifacts"
BASE_DEST_DIR="test_data"

# Check if the input file exists
if [[ ! -f "${INPUT_FILE}" ]]; then
    echo "Error: File '${INPUT_FILE}' not found."
    exit 1
fi

# Parse JSON and download files
jq -r 'to_entries[] | .key as $dir | .value[] | "\($dir) \(.)"' "$INPUT_FILE" | while read -r DIR_NAME FILE; do
    DEST_DIR="${BASE_DEST_DIR}/${DIR_NAME}"
    mkdir -p "${DEST_DIR}"

    echo "Downloading ${FILE} to ${DEST_DIR}..."

    # Download the file
    curl -Lo "${DEST_DIR}/${FILE}" "${BUCKET_URL}/${DIR_NAME}/${FILE}" || {
        echo "Error: Failed to download ${FILE}" >&2
    }
done

echo "All files specified in '${INPUT_FILE}' have been downloaded to '${BASE_DEST_DIR}'."
