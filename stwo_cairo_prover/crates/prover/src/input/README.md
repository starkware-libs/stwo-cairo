# Slow Tests Using Google Cloud Storage

## Input Files

The input files for the tests in vm_import are stored in Google Cloud Storage:  
[Google Cloud Storage Browser](https://console.cloud.google.com/storage/browser/stwo-cairo-testing-artifacts?project=starkware-thirdparties)

## How to Run the Tests

1. **Download Input Files**  
   Run the following script to fetch the required input files from Google Cloud Storage:
   ```bash
   ./scripts/fetch_large_files.sh
   ```

2. **Execute Tests**  
   Use the following commands to run the tests:
   ```bash
   cargo test test_read_from_large_files --features "slow-tests"
   cargo test test_read_from_small_file --features "slow-tests"
   ```

## CI Configuration

These tests are configured to run automatically in the CI pipeline.  
However, note that they will be ignored unless the `--features "slow-tests"` flag is specified in the `cargo test` command.

## Adding a New "Slow-Test"

1. **Create a Directory**  
   Create a new directory at the Google Cloud Storage URL above and add the required files.

2. **Automated Download**  
   The files will be automatically downloaded by the script to a directory with the same name under `crate/input/test_data`.

3. **Mark the Test**  
   Mark the new test with `#[cfg(feature = "slow-tests")]` to ensure it runs in the CI pipeline.

4. **Make Sure the Test Passes Locally**  
    Run the test locally using:
    ```bash
    cargo test "<test_name>" --features "slow-tests"


