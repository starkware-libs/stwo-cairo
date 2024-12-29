#!/bin/bash

BUCKET_URL="https://storage.googleapis.com/stwo-cairo-testing-artifacts"
DEST_DIR="crates/prover/test_data"
TEST_NAME="test_read_from_large_files"

# Create the destination directory
mkdir -p "${DEST_DIR}/${TEST_NAME}"

# Fetch the list of files and download them
curl -s "${BUCKET_URL}" | grep -oP '(?<=<Key>).*?(?=</Key>)' | while read -r FILE; do
    echo "Downloading ${FILE}..."
    curl -Lo "${DEST_DIR}/${FILE}" "${BUCKET_URL}/${FILE}" || {
        echo "Failed to download ${FILE}"
    }
done

# Run the test
echo "Running the test: ${TEST_NAME}..."
cargo test ${TEST_NAME} -- --ignored|| {
    echo "Test failed"

    # Delete the downloaded files
    echo "Cleaning up downloaded files..."
    rm -rf "${DEST_DIR}/${TEST_NAME}"/*
    exit 1
}

# Delete the downloaded files
echo "Cleaning up downloaded files..."
rm -rf "${DEST_DIR}/${TEST_NAME}"*

echo "Done!"