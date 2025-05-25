# ⚡ Stwo Cairo ⚡

Prove Cairo programs with the blazing-fast [Stwo prover](https://github.com/starkware-libs/stwo), powered by the cryptographic breakthrough of [Circle STARKs](https://eprint.iacr.org/2024/278).

* [Disclaimer](#disclaimer)
* [Prerequisites](#prerequisites)
* [Usage](#usage)
  * [Example](#example)
* [Inputs](#inputs)
* [Limitations](#limitations)
  * [Gas](#gas)
  * [Syscalls](#syscalls)
  * [Padding](#padding)
  * [Pedersen](#pedersen)
* [`scarb-prove`](#scarb-prove)

## Disclaimer

⚠️ S-Two is a work in progress and should be used at your own risk ⚠️

In particular:

* S-Two is not yet sound

* Breaking API changes might happen often

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install/)

  - See [`cairo-prove/rust-toolchain.toml`](cairo-prove/rust-toolchain.toml) for the specific version
- [Scarb](https://docs.swmansion.com/scarb/download.html)
  - The recommended installation method is using [asdf](https://asdf-vm.com/)
  - Make sure to use version 2.10.0 and onwards, and preferably the latest nightly version.
  
    To use the latest nightly version, run:
    
    ```
    asdf set -u scarb latest:nightly
    ```

## Installation

First, clone this repo and navigate to its directory:

```
git clone https://github.com/starkware-libs/stwo-cairo.git
cd stwo-cairo
```

Then, navigate to the `cairo-prove` directory and build the project:

```
cd cairo-prove
./build.sh
```

Finally, add the `cairo-prove` binary to your PATH:

```
sudo cp target/release/cairo-prove /usr/local/bin/
```

*Note: Adding the binary to your path is optional but highly recommended, as otherwise `cairo-prove`'s path needs to be specified each time it is used.*

## Usage

To prove an execution of a Cairo program you must first create its executable. To do so, navigate to its directory and run:

```
scarb build
```

To generate a proof for an execution of a program's executable, run:

```
cairo-prove prove <path-to-executable> <path-to-output> --arguments <args>
```
or:

```
cairo-prove prove <path-to-executable> <path-to-output-file> --arguments-file <path-to-args-file>
```

*Note: For information about the formats of `arguments` and `arguments-file`, see [Inputs](#inputs).*

To verify a proof, run:

```
cairo-prove verify <path-to-proof-file>
```

or, if the Pedersen builtin is used in the proof:

```
cairo-prove verify <path-to-proof-file> --with-pedersen
```

*Note: To learn more about the effects of the Pedersen builtin, see [Pedersen](#pedersen).*

### Example

The following can be used inside the `cairo-prove/example` directory to create an executable of [cairo-prove/example/src/lib.cairo](cairo-prove/example/src/lib.cairo), prove its execution, and verify the proof:

```terminal
scarb build
cairo-prove prove \
  target/dev/example.executable.json \
  ./example_proof.json \
  --arguments 10000
cairo-prove verify ./example_proof.json
```

If successful, the result should be similar to the following:

```
Compiling example v0.1.0 (stwo-cairo/cairo-prove/example/Scarb.toml)
Finished `dev` profile target(s) in 4 seconds
[2025-05-27T10:23:36Z INFO  cairo_prove] Generating proof for target: "target/dev/example.executable.json"
[2025-05-27T10:23:36Z INFO  cairo_prove::execute] Executing program...
[2025-05-27T10:23:36Z INFO  cairo_prove::execute] Program executed successfully.
[2025-05-27T10:23:36Z INFO  cairo_prove::prove] Generating input for the prover...
[2025-05-27T10:23:36Z INFO  cairo_prove::prove] Input for the prover generated successfully.
[2025-05-27T10:23:52Z INFO  cairo_prove] Proof saved to: "./example_proof.json"
[2025-05-27T10:23:52Z INFO  cairo_prove] Proof generation completed in 15.74s
[2025-05-27T10:26:02Z INFO  cairo_prove] Verifying proof from: "./example_proof.json"
[2025-05-27T10:26:02Z INFO  cairo_prove] Verification successful
```

## Inputs

The expected input provided to `cairo-prove prove` using the `--arguments` option is a comma-separated list of integers. This list should correspond to the [serialization](https://docs.starknet.io/architecture-and-concepts/smart-contracts/serialization-of-cairo-types/) of the `main` function’s arguments, for example:

| main’s signature | valid arguments example |
| :---- | :---- |
| `fn main(num: u8)` | 1 |
| `fn main(num1: u8, num2: u16)` | 1,2 |
| `fn main(num1: u8, tuple: (u16, u16))` | 1,2,3 |
| `fn main(num1: u8, num2: u256)` | 1,2,3 |
| `fn main(num1: u8, arr: Array<u8>)` | 1,2,1,2 |


When using the `--arguments-file` option, the expected content of the file is an array of the equivalent hex strings. For example, `1,2,3` in the above table becomes `["0x1", "0x2", "0x3"]`.

## Limitations

### Gas

Executables must be created with the `enable-gas = false` config in project's `Scarb.toml` file (e.g., [cairo-prove/example/Scarb.toml](cairo-prove/example/Scarb.toml)).

This limitation exists because gas tracking introduces computational overhead, which does not make sense in non-Starknet contexts.

### Syscalls

Executables cannot be created from programs that use [syscalls](https://book.cairo-lang.org/appendix-08-system-calls.html), either directly or via functions from [the Cairo Core library](https://docs.cairo-lang.org/core/) that use syscalls (such as `sha256`, `keccak`, and `secp256k1`/`secp256r1` operations).

### Padding

Execution resources (the number of steps and builtin invocations) are currently padded to the next power of 2, with respect to the ratios in the [`all_cairo` layout](https://github.com/lambdaclass/cairo-vm/blob/15bf79470cdd8eff29f41fc0a87143dce5499c7e/vm/src/types/instance_definitions/builtins_instance_def.rs#L157).

This padding exists for legacy reasons and will be removed in a future version, as Stwo does not rely on it.

### Pedersen

When the [Pedersen builtin](https://book.cairo-lang.org/ch204-02-01-pedersen.html) is used in an execution of a program, additional preprocessed columns need to be added to its proof.

This variant is automatically deduced by `cairo-prove prove`, but requires adding the `--with-pedersen` option to `cairo-prove verify`.

## `scarb prove`

As of Scarb version 2.10.0, `scarb prove` can be used instead of manually building and running `stwo-cairo`.

However, `scarb prove` is still a work in progress, and using `stwo-cairo` directly is preferable for now.
