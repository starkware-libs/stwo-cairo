# Slow Tests Using Google Cloud Storage

[Google Cloud Storage Browser](https://console.cloud.google.com/storage/browser/stwo-cairo-testing-artifacts?project=starkware-thirdparties)
Note: This is an external directory.

## How to Test

1. **Download Tests Files**  
   ```bash
   ./scripts/fetch_large_files.sh ./scripts/slow_tests.json
   ```

2. **Execute Tests**  
   ```bash
   cargo test --features "slow-tests"
   ```

## Adding a New "Slow-Test"

1. **Create a Directory**  
   Create a new folder at the Cloud Storage URL above and upload the necessary files. 
   Note: priv.json should have the relative path to the memory and trace files. Absolute paths won't work in the CI.

2. **Update the Slow-Tests JSON**  
   Update ./scripts/slow_tests.json by specifying the directory name and the files saved there.

3. **Mark the Test**  
   Mark the new test with `#[cfg(feature = "slow-tests")]`.

4. **Run Tests Locally**  
    Run the test locally using:
    ```bash
    cargo test "<test_name>" --features "slow-tests"


# Note on running tests locally.
Testing unoptimized auto-generated code that runs in parallel is difficult due to stack overflows and compile-time-optimizations taking too long.
Refer to `stwo_cairo_prover/cargo.toml` files for info on how to alleviate that issue.
