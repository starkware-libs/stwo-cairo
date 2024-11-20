# Stwo Cairo Verifier

A [Cairo](https://github.com/starkware-libs/cairo) program that verifies a [Stwo](https://github.com/starkware-libs/stwo) proof.

## Install dependencies

[Install asdf](https://asdf-vm.com/guide/getting-started.html#_3-install-asdf) and run:

```bash
asdf plugin add scarb
asdf plugin add starknet-foundry
asdf install
```

## Run tests

Make sure [dependencies are installed](#install-dependencies). Run:

```bash
scarb test
```

## Profile tests

Modify [`Scarb.toml`](./Scarb.toml) to use [Starknet Foundary](https://github.com/foundry-rs/starknet-foundry).

```diff
[dev-dependencies]
- cairo_test = "2.8.5"
+ snforge_std = { git = "https://github.com/foundry-rs/starknet-foundry", tag = "v0.33.0" }
+ assert_macros = "2.8.5"
+
+ [scripts]
+ test = "snforge test --max-n-steps 100000000"
```

Generate trace for all tests.

<!-- TODO(andrew): Debug error on Linux. -->
> :warning: Command produces errors on Linux.

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
