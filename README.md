# stwo-cairo-air
```
This repo utilizes Google Cloud Storage for test artifacts. The test artifacts can be found at the following URL:
https://storage.googleapis.com/stwo-cairo-testing-artifacts

To run the tests "test_read_from_large_files", which marked with the "ci-test" feature, use the run_large_test.sh script. 

This script will:
Download the required input files from Google Cloud Storage.
Execute the test.
Delete the downloaded files after the test is complete.
This test is configured to run automatically in the CI pipeline and will be ignored unless the --features "ci-test" flag is specified in the cargo test command."

```
