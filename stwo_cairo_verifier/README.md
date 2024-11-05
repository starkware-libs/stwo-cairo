# Stwo Cairo Verifier

A [Cairo](https://github.com/starkware-libs/cairo) program that verifies a [Stwo](https://github.com/starkware-libs/stwo) proof.

## Running tests

[Install asdf](https://asdf-vm.com/guide/getting-started.html#_3-install-asdf).

```bash
asdf install
scarb test
```

## Visualising test step counts.

Modify [`Scarb.toml`](./Scarb.toml) to use [Starknet Foundary](https://github.com/foundry-rs/starknet-foundry).

```diff
[dev-dependencies]
- cairo_test = "2.8.0"
+ snforge_std = { git = "https://github.com/foundry-rs/starknet-foundry", tag = "v0.32.0" }
+ assert_macros = "2.8.0"
+
+ [scripts]
+ test = "snforge test --max-n-steps 100000000"
```

Generate trace for all tests.

> :warning: Get errors with this on Linux.

```bash
scarb test -- --save-trace-data
```

Install [cairo-profiler](https://github.com/software-mansion/cairo-profiler) and run:

```bash
# Replace `TEST_NAME` with the name of the test you want profiled.
cairo-profiler ./snfoundry_trace/TEST_NAME.json
```

Visualise profile in the browser.

```bash
# Once opened navigate to `Sample -> steps`.
go tool pprof -http=":8000" profile.pb.gz
```
