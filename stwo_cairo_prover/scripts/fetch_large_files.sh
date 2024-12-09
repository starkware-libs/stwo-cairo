#!/bin/bash

BUCKET_URL="https://storage.googleapis.com/stwo-cairo-testing-artifacts"
DEST_DIR="crates/prover/test_data"

# Create the destination directory
mkdir -p "${DEST_DIR}"

curl -s "${BUCKET_URL}" | grep -oP '(?<=<Key>).*?(?=</Key>)' | while read -r FILE; do
    echo "Downloading ${FILE}..."

    # Create the necessary directories
    mkdir -p "${DEST_DIR}/$(dirname "${FILE}")"

    # Download the file
    curl -Lo "${DEST_DIR}/${FILE}" "${BUCKET_URL}/${FILE}" || {
        echo "Failed to download ${FILE}" >&2
    }
done
