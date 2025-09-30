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

Modify [`Scarb.toml`](./Scarb.toml) to use [Starknet Foundry](https://github.com/foundry-rs/starknet-foundry).

```diff
[dev-dependencies]
- cairo_test = "2.11.4"
+ snforge_std = { git = "https://github.com/foundry-rs/starknet-foundry", tag = "v0.33.0" }
+ assert_macros = "2.9.2"
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

## Profile executable

In order to run the verifier program we need to prepare arguments (proof) first.

Build Stwo prover from the current commit:

```sh
cd ../stwo_cairo_prover
RUSTFLAGS="-C target-cpu=native -C opt-level=3" cargo build --release --bin prove_from_compiled_program
```

Generate a proof for one of the test programs:

```sh
mkdir -p target/bench
../stwo_cairo_prover/target/release/prove_from_compiled_program \
    --compiled_program ../stwo_cairo_prover/test_data/test_prove_verify_all_opcode_components/compiled.json \
    --proof_path target/bench/proof.serde.json \
    --proof-format cairo-serde \
    --verify
```

### Used resources

To estimate the execution resources (total number of steps, builtin usage) run the following command:

```sh
scarb execute \
    --package stwo_cairo_verifier \
    --arguments-file target/bench/proof.serde.json \
    --print-resource-usage \
    --output none
```

Arguments file is the proof (in Cairo serde format) that we generated on the previous step.

### Scoped sierra statements

For more insights you can generate a scoped Sierra profile (loops and recursions are collapsed to improve readability).

First of all, clone `cairo` locally (if you haven't yet), then install `cairo-execute` binary:

```sh
git clone https://github.com/starkware-libs/cairo ~/cairo
cd cairo
cargo install --path crates/bin/cairo-execute cairo-execute
```

Also from the current directory create a symlink to the `corelib`:

```sh
# Working directory is stwo_cairo_verifier
ln -s ~/cairo/corelib corelib
```

Now we can generate the profile:

```sh
cairo-execute \
    --run-profiler scoped \
    --args-file target/bench/proof.serde.json \
    --output-path target/stwo_cairo_verifier.pie.zip \
    --executable stwo_cairo_verifier::main \
    --layout all_cairo_stwo \
    --ignore-warnings \
    crates/cairo_verifier/blake_cairo_project > target/stwo_cairo_verifier.profile.txt
```

You can visualize it with `flamegraph.pl` or with `scarb-burn` tool.  
Use `crates/cairo_verifier/poseidon_cairo_project` if you want to profile the project with `poseidon252_verifier` feature enabled.  

```sh
cargo install --git https://github.com/m-kus/scarb-burn --rev f01a5164576e29c002098ab397fb015808a4fb7b scarb-burn
```

To render a flamechart and open in the browser:

```sh
scarb-burn \
    --profile-file target/stwo_cairo_verifier.profile.txt \
    --output-file target/stwo_cairo_verifier.flamegraph.svg \
    --output-type flamegraph \
    --open-in-browser
```

To generate pprof file and run the service in the browser (requires golang toolchain and pprof package installed):

```sh
scarb-burn \
    --profile-file target/stwo_cairo_verifier.profile.txt \
    --output-file target/stwo_cairo_verifier.pprof.gz \
    --output-type pprof \
    --open-in-browser
```
