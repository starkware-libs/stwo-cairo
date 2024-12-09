# Slow Tests Using Google Cloud Storage

[Google Cloud Storage Browser](https://console.cloud.google.com/storage/browser/stwo-cairo-testing-artifacts?project=starkware-thirdparties)

## How to Test

1. **Download Tests Files**  
   ```bash
   ./scripts/fetch_large_files.sh
   ```

2. **Execute Tests**  
   ```bash
   cargo test test_read_from_large_files --features "slow-tests"
   cargo test test_read_from_small_file --features "slow-tests"
   ```

## Adding a New "Slow-Test"

1. **Create a Directory**  
   Create a new directory at the Google Cloud Storage URL above and add the required files.

2. **Mark the Test**  
   Mark the new test with `#[cfg(feature = "slow-tests")]`.

4. **Run Tests Locally**  
    Run the test locally using:
    ```bash
    cargo test "<test_name>" --features "slow-tests"


