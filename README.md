# stwo-cairo-air
```
This repo utilizes Google Cloud Storage for test artifacts. The test artifacts can be found and modified at the following URL:
https://console.cloud.google.com/storage/browser/stwo-cairo-testing-artifacts?project=starkware-thirdparties

To run the test: "test_read_from_large_files", use the run_large_test.sh script. 

This script will:
Download the required input files from Google Cloud Storage.
Execute the test.
Delete the downloaded files after the test is complete.

This test is configured to run automatically in the CI pipeline and will be ignored unless the --features "ci-test" flag is specified in the cargo test command."

```
