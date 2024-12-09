#!/bin/bash
BUCKET_URL="https://storage.googleapis.com/stwo-cairo-testing-artifacts"
DEST_DIR="crates/prover/test_data"

# Create the destination directory
mkdir -p "${DEST_DIR}"

# Fetch the list of files and download them
curl -s "${BUCKET_URL}" | grep -oP '(?<=<Key>).*?(?=</Key>)' | while read -r FILE; do
    echo "Downloading ${FILE}..."
    curl -Lo "${DEST_DIR}/${FILE}" "${BUCKET_URL}/${FILE}" || echo "Failed to download ${FILE}"
done